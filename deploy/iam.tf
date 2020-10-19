data "aws_partition" "current" {}
# == "aws" in most cases
data "aws_region" "current" {}
# project region
data "aws_caller_identity" "current" {}
# account id and more

##########
# Lambda #
##########

# assume role policy
data "aws_iam_policy_document" "lambda_assume_role_policy" {
  statement {
    actions = [
    "sts:AssumeRole"]

    principals {
      type = "Service"
      identifiers = [
      "lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "lambda_role" {
  name               = "${var.tags.project}-lambda-role"
  path               = "/system/"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role_policy.json


  tags = merge(var.tags, {
    Name = "${var.tags.project}-lambda-role"
  })
}

# execution policy
data "aws_iam_policy_document" "lambda_execution_policy_document" {
  // Logs
  statement {
    actions = [
      "logs:CreateLogStream",
      "logs:CreateLogGroup",
      "logs:PutLogEvents"
    ]
    resources = ["arn:${data.aws_partition.current.partition}:logs:*:*:*"]
  }
  // X-Ray
  statement {
    actions   = ["xray:PutTraceSegments", "xray:PutTelemetryRecords"]
    resources = ["*"]
  }
  // Enhanced Monitoring
  statement {
    actions   = ["ssm:GetParameter"]
    resources = ["arn:${data.aws_partition.current.partition}:ssm:*:*:parameter/AmazonCloudWatch-*"]
  }
  // Emit Notifications
  statement {
    actions = [
    "SNS:Publish"]
    resources = ["arn:${data.aws_partition.current.partition}:sns:*:*:*"]
  }
  // DynamoDB Invocation
  statement {
    actions = [
      "dynamodb:DescribeStream",
      "dynamodb:GetRecords",
      "dynamodb:GetShardIterator",
      "dynamodb:ListStreams"
    ]
    resources = ["arn:${data.aws_partition.current.partition}:dynamodb:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:*"]
  }
}

resource "aws_iam_policy" "lambda_execution_policy" {
  name        = "${var.tags.project}-lambda-execution-policy"
  path        = "/"
  description = "IAM policy for logging from a lambda"
  policy      = data.aws_iam_policy_document.lambda_execution_policy_document.json
}

resource "aws_iam_role_policy_attachment" "lambda_policy" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_execution_policy.arn
}

#############
# S3 Bucket #
#############

data "aws_iam_policy_document" "s3_bucket_policy_document" {
  // denies all s3 access without ssl
  // complies with s3-bucket-ssl-requests-only rule
  statement {
    effect = "Deny"
    actions = [
    "s3:*"]
    resources = ["arn:${data.aws_partition.current.partition}:s3:::${aws_s3_bucket.static_files.bucket}/*"]

    # anonymous user
    principals {
      identifiers = [
      "*"]
      type = "*"
    }

    condition {
      test = "Bool"
      values = [
      false]
      variable = "aws:SecureTransport"
    }
  }
}

resource "aws_s3_bucket_policy" "static_files_policy" {
  bucket = aws_s3_bucket.static_files.id
  policy = data.aws_iam_policy_document.s3_bucket_policy_document.json
}
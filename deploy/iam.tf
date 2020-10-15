data "aws_partition" "current" {}       # == "aws" in most cases
data "aws_region" "current" {}          # project region
data "aws_caller_identity" "current" {} # account id and more

##########
# Lambda #
##########

data "aws_iam_policy_document" "lambda_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
  statement {
    actions   = ["logs:CreateLogStream", "logs:CreateLogGroup"]
    resources = ["arn:${data.aws_partition.current.partition}:logs:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:log-group:${aws_cloudwatch_log_group.lambda_log_group.name}*:*"]
  }
  statement {
    actions   = ["logs:PutLogEvents"]
    resources = ["arn:${data.aws_partition.current.partition}:logs:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:log-group:${aws_cloudwatch_log_group.lambda_log_group.name}*:*:*"]
  }
}

resource "aws_iam_role" "lambda_role" {
  name               = "${var.tags.project}-lambda-role"
  path               = "/system/"
  assume_role_policy = data.aws_iam_policy_document.lambda_role_policy.json

  tags = merge(var.tags, {
    Name = "${var.tags.project}-lambda-role"
  })
}

#############
# S3 Bucket #
#############

data "aws_iam_policy_document" "s3_bucket_policy_document" {
  // denies all s3 access without ssl
  // complies with s3-bucket-ssl-requests-only rule
  statement {
    effect    = "Deny"
    actions   = ["s3:*"]
    resources = ["arn:${data.aws_partition.current.partition}:s3:::${aws_s3_bucket.static_files.bucket}/*"]

    # anonymous user
    principals {
      identifiers = ["*"]
      type        = "*"
    }

    condition {
      test     = "Bool"
      values   = [false]
      variable = "aws:SecureTransport"
    }
  }
}

resource "aws_s3_bucket_policy" "static_files_policy" {
  bucket = aws_s3_bucket.static_files.id
  policy = data.aws_iam_policy_document.s3_bucket_policy_document.json
}
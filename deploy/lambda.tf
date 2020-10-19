variable lambda_file {
  type = map(string)
  default = {
    filename = "lambda.zip"
    location = "./lambda.zip"
  }
}

resource "aws_s3_bucket_object" "lambda_codebase_zip" {
  bucket = aws_s3_bucket.static_files.id
  key    = var.lambda_file.filename
  source = var.lambda_file.location
  etag   = filemd5(var.lambda_file.location)
}

resource "aws_lambda_function" "strategy_lambda" {
  depends_on = [
    aws_s3_bucket_object.lambda_codebase_zip,
    aws_iam_role_policy_attachment.lambda_policy,
  ]

  # S3 bucket must exist with a packaged .zip before terraform apply
  s3_bucket        = aws_s3_bucket.static_files.bucket
  s3_key           = aws_s3_bucket_object.lambda_codebase_zip.key
  source_code_hash = filebase64sha256(var.lambda_file.location)

  publish       = true
  function_name = "${var.tags.project}-lambda-function"
  description   = "process a dynamodb stream and notify users with sns"
  role          = aws_iam_role.lambda_role.arn
  # handler value syntax is `{cargo-package-name}.{bin-name}`
  # or `{cargo-package-name}` for short when you are building a
  # default bin for a given package.
  handler     = "doesnt.matter" # when using an executable, the Handler information is not needed.
  memory_size = 128
  timeout     = 6
  runtime     = "provided.al2" # Custom Runtime - Amazon Linux 2

  environment {
    variables = {
      RUST_BACKTRACE = 1
    }
  }

  tracing_config {
    mode = "Active" # disables X-Ray
  }

  tags = merge(var.tags, {
    Name = "${var.tags.project}-lambda"
  })
}

resource "aws_lambda_event_source_mapping" "dynamodb_event" {
  event_source_arn       = aws_dynamodb_table.input_dynamodb_table.stream_arn
  function_name          = aws_lambda_function.strategy_lambda.arn
  starting_position      = "LATEST"
  maximum_retry_attempts = 1
}

resource "aws_lambda_function_event_invoke_config" "lambda_output_signal" {
  function_name = aws_lambda_function.strategy_lambda.function_name

  destination_config {
    //    on_failure {
    //      destination = aws_sqs_queue.
    //    }

    on_success {
      destination = aws_sns_topic.lambda_output.arn
    }
  }
}
resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${var.tags.project}"
  retention_in_days = 90


  tags = merge(var.tags, {
    Name = "${var.tags.project}-lambda-log-group"
  })
}
resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${var.tags.project}"
  retention_in_days = 90

  tags = merge(var.tags, {
    Name = "${var.tags.project}-lambda-log-group"
  })
}

############################
# Lambda Scheduled Trigger #
############################

resource "aws_cloudwatch_event_rule" "every_half_hour" {
  is_enabled = true

  name        = "half-hour-trigger"
  description = "Fires every 30 minutes"

  // might need to review this - use cron
  schedule_expression = "rate(30 minutes)"

  tags = merge(var.tags, {
    Name = "lambda schedule rule"
  })
}

resource "aws_cloudwatch_event_target" "lambda_schedule_target" {
  target_id = "TriggerLambda"
  rule      = aws_cloudwatch_event_rule.every_half_hour.name
  arn       = aws_lambda_function.strategy_lambda.arn
}

resource "aws_lambda_permission" "allow_cloudwatch_to_call_function" {
  statement_id = "AllowExecutionFromCloudWatch"
  action       = "lambda:InvokeFunction"
  principal    = "events.amazonaws.com"

  function_name = aws_lambda_function.strategy_lambda.function_name
  source_arn    = aws_cloudwatch_event_rule.every_half_hour.arn
}

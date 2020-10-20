//resource "aws_sns_topic" "lambda_output" {
//  name = "${var.tags.project}-sns-output"
//
//  tags = merge(var.tags, {
//    Name = "${var.tags.project}-sns-output"
//  })
//}

//resource "aws_sns_topic_subscription" "user_updates_sms" {
//  topic_arn = aws_sns_topic.lambda_output.arn
//  protocol  = "sms"
//  endpoint  = var.user.phone
//}

//
//resource "aws_sns_topic" "lambda_failure" {
//  name = "${var.tags.project}-failure"
//
//  tags = merge(var.tags, {
//    Name = "${var.tags.project}-sns-output"
//  })
//}

//resource "aws_lambda_function_event_invoke_config" "lambda_output_signal" {
//  function_name = aws_lambda_function.strategy_lambda.function_name
//
//  destination_config {
//    //    on_failure {
//    //      destination = aws_sqs_queue.
//    //    }
//
//    on_success {
//      destination = aws_sns_topic.lambda_output.arn
//    }
//  }
//}
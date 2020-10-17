resource "aws_sns_topic" "lambda_output" {
  name = "${var.tags.project}-sns-output"

  tags = merge(var.tags, {
    Name = "${var.tags.project}-sns-output"
  })
}
//
//resource "aws_sns_topic" "lambda_failure" {
//  name = "${var.tags.project}-failure"
//
//  tags = merge(var.tags, {
//    Name = "${var.tags.project}-sns-output"
//  })
//}
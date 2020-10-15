resource "aws_s3_bucket" "static_files" {
  bucket = "${var.tags.project}-bucket"
  acl    = "private"


  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }


  tags = merge(var.tags, {
    Name = "${var.tags.project}-files"
  })
}
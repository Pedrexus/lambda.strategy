#
# infrastructure architecture description:
#
# process:
#
# 1. user `assets` data -> dynamodb table
# 2. cloudwatch cron trigger -> lambda function
# 3. dynamo table -> lambda -> sns notification
# 4. sns topic -> user sms
#
# details:
#
# - lambda code <- s3 bucket
# - lambda logs <- logs group
# - api gateway <- lambda

terraform {
  required_providers {
    aws = {
      # uses the latest provider from hashicorp registry
      source = "hashicorp/aws"
    }
  }
  backend "remote" {
    organization = "phvv"

    workspaces {
      name = "github-actions"
    }
  }
}

provider "aws" {
  # uses the default credentials
  profile = "default"
  region  = "us-east-1"
  # N. Virginia
  version = "~> 3.6.0"
}


variable "tags" {
  type = map(string)
  default = {
    "project" = "Strategy"
    "owner"   = "Pedro Valois"
  }
}

resource "aws_dynamodb_table" "input_dynamodb_table" {
  name             = "Input"
  billing_mode     = "PROVISIONED"
  stream_enabled   = true
  stream_view_type = "KEYS_ONLY"
  read_capacity    = 5
  write_capacity   = 5
  hash_key         = "symbol"
  range_key        = "source"

  attribute {
    name = "symbol"
    type = "S"
  }

  attribute {
    name = "source"
    type = "S"
  }

  tags = merge(var.tags, {
    Name = "${var.tags.project}-dynamodb-table-1"
  })
}

resource "aws_dynamodb_table_item" "input_1" {
  table_name = aws_dynamodb_table.input_dynamodb_table.name
  hash_key   = aws_dynamodb_table.input_dynamodb_table.hash_key
  range_key  = aws_dynamodb_table.input_dynamodb_table.range_key

  item = <<ITEM
{
  "symbol": {
    "S": "PETR4.SA"
  },
  "source": {
    "S": "Yahoo"
  },
  "strategy": {
    "S": "RSI"
  },
  "parameters": {
    "M": {
      "window": {
        "N": "14"
      },
      "lower bound": {
        "N": "20"
      },
      "upper bound": {
        "N": "80"
      }
    }
  },
  "settings": {
    "M": {
      "range": {
        "S": "6mo"
      },
      "interval": {
        "S": "1d"
      }
    }
  }
}
ITEM
}

resource "aws_dynamodb_table_item" "input_2" {
  table_name = aws_dynamodb_table.input_dynamodb_table.name
  hash_key   = aws_dynamodb_table.input_dynamodb_table.hash_key
  range_key  = aws_dynamodb_table.input_dynamodb_table.range_key

  item = <<ITEM
{
  "symbol": {
    "S": "BTC_XMR"
  },
  "source": {
    "S": "Poloniex"
  },
  "strategy": {
    "S": "RSI"
  },
  "parameters": {
    "M": {
      "window": {
        "N": "14"
      },
      "lower bound": {
        "N": "20"
      },
      "upper bound": {
        "N": "80"
      }
    }
  },
  "settings": {
    "M": {
      "range": {
        "S": "5d"
      },
      "interval": {
        "S": "30m"
      }
    }
  }
}
ITEM
}
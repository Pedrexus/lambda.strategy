use dynomite::dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use dynomite::retry::{Policy, RetryingDynamoDb};
use dynomite::{AttributeValue, DynamoDbExt, FromAttributes, Retries};
use futures::{future, StreamExt, TryStreamExt};
use rusoto_core::Region;
use std::collections::HashMap;

const REGION: Region = Region::UsEast1;
const RETRY_POLICY: Policy = Policy::default();
const CLIENT: RetryingDynamoDb<DynamoDbClient> =
    DynamoDbClient::new(REGION).with_retries(RETRY_POLICY);
const TABLE_NAME: String = String::from("Input");

#[derive(Attributes, Debug, Clone)]
pub struct Parameters {
    window: u64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Attributes, Debug, Clone)]
pub struct Settings {
    range: ChartRange,
    interval: CandlestickInterval,
}

#[derive(Item, Debug, Clone)]
pub struct Input {
    #[dynomite(partition_key)]
    symbol: String,
    #[dynomite(sort_key)]
    source: String,
    strategy: String,
    parameters: Option<String>,
    settings: Option<Settings>,
}

fn read_table() -> Vec<Input> {
    let scan_input = ScanInput {
        table_name: TABLE_NAME,
        ..ScanInput::default()
    };

    fn from_attrs(item: HashMap<String, AttributeValue>) -> Input {
        Input::from_attrs(item)
    }

    let mut data: Vec<Input> = Vec::new();

    CLIENT
        .scan_pages(scan_input)
        .try_for_each(|item| {
            data.push(Input::from_attrs(item));
            future::ready(Ok(()))
        })
        .await?;

    data
}

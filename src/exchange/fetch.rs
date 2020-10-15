use reqwest::{Url, Response};
use std::env;

use crate::exchange::{models::API, error, error::{Result}};

use snafu::{ ensure, OptionExt, ResultExt };

/// Helper function to build up the main query URL
pub fn fetch(api: API, base_url: &str, symbol: &str) -> Result<Url> {
   Ok(Url::parse(base_url).context(error::InternalURL { url: base_url })?
      .join(symbol).context(error::InternalURL { url: symbol })?)
}
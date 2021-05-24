#[macro_use]
extern crate failure;
extern crate reqwest;

use failure::Error;
use reqwest::{StatusCode, Url};

/// A description of availability on npmjs.com
pub enum Availability {
  /// Package is available
  Available,
  /// Package is unavailable
  Unavailable,
  /// Availability is unknown because an unknown status code was returned.
  Unknown,
}

/// Get the availability for a package on npmjs.com.
pub fn get(name: &str) -> Result<Availability, Error> {
  ensure!(!name.is_empty(), "name should be more than 0 characters");
  let addr = format!("https://registry.npmjs.org/{}/", name);
  let url = Url::parse(&addr)?;
  let res = reqwest::get(url)?;
  let status = match res.status() {
    StatusCode::OK => Availability::Unavailable,
    StatusCode::NOT_FOUND => Availability::Available,
    _ => Availability::Unknown,
  };
  Ok(status)
}

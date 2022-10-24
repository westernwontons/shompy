use std::{borrow::Cow, fmt::Display, num::ParseFloatError};

use prisma_client_rust::chrono::{NaiveDate, ParseError as ParseDateError};

/// Items to hold the elements of the input boxes
#[derive(Debug, Clone)]
pub struct Item {
  name: String,
  price: String,
  amount: String,
  purchase_date: String
}

impl Display for Item {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} | {} | {} | {}",
      self.name, self.price, self.amount, self.purchase_date
    )
  }
}

impl Item {
  /// Construct a new `Item` with the given elements
  pub fn new<'a, T>(name: T, price: T, amount: T, purchase_date: T) -> Self
  where
    // Convert whatever we get into a `Cow` then convert that into a `String`
    T: Into<Cow<'a, str>>
  {
    Self {
      name: name.into().into_owned(),
      price: price.into().into_owned(),
      amount: amount.into().into_owned(),
      purchase_date: purchase_date.into().into_owned()
    }
  }

  /// Return every field in a `Vec`
  pub fn as_vec(&self) -> Vec<&str> {
    vec![
      self.name.as_str(),
      self.price.as_str(),
      self.amount.as_str(),
      self.purchase_date.as_str(),
    ]
  }

  /// Parse the value of `Price`
  pub fn validate_price(&self) -> Result<f64, ParseFloatError> {
    self.price.parse::<f64>()
  }
  /// Parse the value of `Amount`
  pub fn validate_amount(&self) -> Result<f64, ParseFloatError> {
    self.amount.parse::<f64>()
  }

  /// Parse the value of `Purchase Date`
  pub fn validate_purchase_date(&self) -> Result<NaiveDate, ParseDateError> {
    NaiveDate::parse_from_str(&self.purchase_date, "%Y/%m/%d")
  }

  /// Joins the structs elements together with the given separator.
  /// Used when adding an `Item` to a `SelectView`
  pub fn joined(&self, separator: &str) -> String {
    self.as_vec().join(separator)
  }

  /// Check if all the fields are empty
  pub fn are_empty(&self) -> bool {
    self.as_vec().iter().all(|element| element.is_empty())
  }

  /// Deconstruct an `Item` instance into it's parts
  pub fn to_parts(self) -> (String, String, String, String) {
    (self.name, self.price, self.amount, self.purchase_date)
  }
}

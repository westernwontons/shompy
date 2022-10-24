use std::borrow::Cow;

use prisma_client_rust::chrono::{DateTime, FixedOffset};

use super::Item;

/// Holds the same data as `Item`, just parsed into the correct data format
#[derive(Debug)]
pub struct FoodItem {
  table_name: String,
  name: String,
  price: f64,
  amount: f64,
  total: f64,
  purchase_date: DateTime<FixedOffset>
}

// Convert an `&Item` into a `FoodItem`
impl From<&Item> for FoodItem {
  fn from(value: &Item) -> Self {
    let mut food_item = Self::new();

    let (name, price, amount, mut purchase_date) = value.to_owned().to_parts();

    purchase_date.push_str(" 00:00:00 +0000");

    food_item.set_name(name);
    food_item.set_price(price.parse::<f64>().unwrap());
    food_item.set_amount(amount.parse::<f64>().unwrap());
    food_item.set_purchase_date(
      DateTime::parse_from_str(&purchase_date, "%Y/%m/%d %H:%M:%S %z").unwrap()
    );
    food_item.calc_total();

    food_item
  }
}

impl FoodItem {
  pub fn new() -> Self {
    Self {
      table_name: String::default(),
      name: String::default(),
      price: f64::default(),
      amount: f64::default(),
      total: f64::default(),
      purchase_date: DateTime::default()
    }
  }

  /// Consume Self and return the building blocks
  pub fn to_parts(
    self
  ) -> (String, String, f64, f64, f64, DateTime<FixedOffset>) {
    (
      self.table_name,
      self.name,
      self.price,
      self.amount,
      self.total,
      self.purchase_date
    )
  }

  /// Set the `table_name`, which is one of the childs of the `Add` menu
  pub fn set_table_name<'a, T>(&mut self, table_name: T) -> &mut Self
  where
    T: Into<Cow<'a, str>>
  {
    self.table_name = table_name.into().into_owned();
    self
  }

  /// Set the product name
  pub fn set_name<'a, T>(&mut self, name: T) -> &mut Self
  where
    T: Into<Cow<'a, str>>
  {
    self.name = name.into().into_owned();
    self
  }

  /// Set the product price
  pub fn set_price(&mut self, price: f64) -> &mut Self {
    self.price = price;
    self
  }

  /// Set the amount of products purchased
  pub fn set_amount(&mut self, amount: f64) -> &mut Self {
    self.amount = amount;
    self
  }

  /// Set total, which is `price * amount`
  pub fn calc_total(&mut self) -> &mut Self {
    self.total = self.price * self.amount;
    self
  }

  /// Set the purchase date
  pub fn set_purchase_date(
    &mut self,
    purchase_date: DateTime<FixedOffset>
  ) -> &mut Self {
    self.purchase_date = purchase_date;
    self
  }
}

use cursive::{
  views::{Dialog, EditView, SelectView},
  Cursive
};

use crate::item::Item;

/// Get the content of a `NamedView<EditView>`
fn get_edit_view_content(s: &mut Cursive, name: &str) -> Option<String> {
  s.call_on_name(name, |edit_view: &mut EditView| {
    String::clone(&edit_view.get_content())
  })
}

/// Functionality for the `Add` button
pub fn add_button(s: &mut Cursive, select_view_name: &str) {
  let mut select_view = match s.find_name::<SelectView<Item>>(select_view_name)
  {
    Some(view) => view,
    None => {
      return s.add_layer(Dialog::info("Couldn't find a view with that name"));
    }
  };

  let product_name = match get_edit_view_content(s, "product_name") {
    Some(content) => {
      if content.is_empty() {
        return s.add_layer(Dialog::info("Name cannot be empty"));
      } else {
        content
      }
    }
    None => {
      return s
        .add_layer(Dialog::info("Couldn't find content with the given name"));
    }
  };

  let product_price = match get_edit_view_content(s, "product_price") {
    Some(content) => {
      if content.is_empty() {
        return s.add_layer(Dialog::info("Price cannot be empty"));
      } else {
        content
      }
    }
    None => {
      return s
        .add_layer(Dialog::info("Couldn't find content with the given name"));
    }
  };

  let product_amount = match get_edit_view_content(s, "product_amount") {
    Some(content) => {
      if content.is_empty() {
        return s.add_layer(Dialog::info("Amount cannot be empty"));
      } else {
        content
      }
    }
    None => {
      return s
        .add_layer(Dialog::info("Couldn't find content with the given name"));
    }
  };

  let product_date_of_purchase =
    match get_edit_view_content(s, "product_date_of_purchase") {
      Some(content) => {
        if content.is_empty() {
          return s.add_layer(Dialog::info("Purchase Date cannot be empty"));
        } else {
          content
        }
      }
      None => {
        return s.add_layer(Dialog::info(
          "Couldn't find content with the given name"
        ));
      }
    };

  let item = Item::new(
    product_name,
    product_price,
    product_amount,
    product_date_of_purchase
  );

  if item.are_empty() {
    return s.add_layer(Dialog::info("Every field must have a value"));
  }

  let product_price = match item.validate_price() {
    Ok(price) => price,
    Err(_) => {
      return s
        .add_layer(Dialog::info("Price field does not have a valid format"))
    }
  };

  let product_amount = match item.validate_amount() {
    Ok(amount) => amount,
    Err(_) => {
      return s
        .add_layer(Dialog::info("Amount field does not have a valid format"))
    }
  };

  let product_date_of_purchase = match item.validate_purchase_date() {
    Ok(date) => date,
    Err(err) => {
      return s.add_layer(Dialog::info(format!(
        "Purchase Date field doesn't not have a valid format:\n{}",
        err
      )))
    }
  };

  select_view.add_item(item.to_string(), item);
}

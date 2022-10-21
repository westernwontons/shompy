use cursive::{
  views::{Dialog, EditView, SelectView},
  Cursive
};
use prisma_client_rust::chrono::NaiveDate;

/// Get the input values from `EditView`s and add them to `SelectView`
pub fn add_button_cb(s: &mut Cursive) {
  /// Extract the content of an `EditView` and add it to a vector
  fn add_to_vector(edit_view: &mut EditView, vector: &mut Vec<String>) {
    // clone the contents of an `Rc<String>` to get an owned `String`
    let item = String::clone(&edit_view.get_content());

    if item.is_empty() {
      return;
    }

    vector.push(item);
  }

  /// Extract the content of the `EditView` and add it to a `Vec`
  fn extract_and_add_to_vector(
    s: &mut Cursive,
    vector: &mut Vec<String>,
    name: &str
  ) {
    s.call_on_name(name, |edit_view: &mut EditView| {
      add_to_vector(edit_view, vector)
    });
  }

  let mut items_added = Vec::<String>::with_capacity(3);

  let mut select_view =
    s.find_name::<SelectView<String>>("select_item").unwrap();

  extract_and_add_to_vector(s, &mut items_added, "product_name");
  extract_and_add_to_vector(s, &mut items_added, "product_price");
  extract_and_add_to_vector(s, &mut items_added, "product_date_of_purchase");

  // We don't want less than three elements
  if items_added.len() < 3 {
    return s.add_layer(Dialog::info("All input fields must be filled"));
  }

  // We don't want bogus elements (like spaces only)
  if items_added
    .iter()
    .any(|el| el.chars().all(|char| char == ' '))
  {
    return s.add_layer(Dialog::info("No bogus please"));
  }

  // We need to check whether the date provided is the correct format
  match NaiveDate::parse_from_str(items_added[2].as_str(), "%Y/%m/%d") {
    Ok(_) => {}
    Err(err) => {
      return s.add_layer(Dialog::info(
        "Date provided is not the correct format. Use YYYY/MM/DD"
      ))
    }
  }

  // Pipes are unlikely to appear in input
  let joined = items_added.join(" | ");
  select_view.add_item_str(joined);
}

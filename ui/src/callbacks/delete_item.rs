use cursive::{
  views::{Dialog, SelectView, TextView},
  Cursive
};

use crate::item::Item;

/// Delete an item from the `select_item` list
pub fn delete_item(s: &mut Cursive, item: &Item) {
  let selected = s.find_name::<SelectView<Item>>("select_item").unwrap();

  // match on the currently selected item in `SelectView`
  match selected.selected_id() {
    None => {
      s.add_layer(Dialog::info("Nothing to remove!").button("Dismiss", |s| {
        s.pop_layer();
      }));
    }

    Some(item) => {
      s.add_layer(
        // Popup window that asks for confirmation
        Dialog::around(TextView::new("Remove item from list?"))
          .button("Confirm", move |s| {
            s.find_name::<SelectView<Item>>("select_item")
              .unwrap()
              .remove_item(item);

            // remove the window from the view upon action
            s.pop_layer();
          })
          .button("Cancel", |s| {
            s.pop_layer();
          })
      );
    }
  }
}

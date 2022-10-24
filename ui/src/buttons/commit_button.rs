use std::sync::Arc;

use cursive::{
  views::{Dialog, SelectView, TextView},
  Cursive
};
use prisma_client_rust::QueryError;

use crate::{
  item::{FoodItem, Item},
  model::Model,
  prisma::food::Data
};

/// The stuff that happens when you press the `Commit` button
///
/// Get every item of the `SelectView` one by one, trim and lowercase them,
/// then collect them into a `Vec<PrismaItem>`, which will be sent to the database
/// in a Tokio thread so the operation doesn't block the main thread
pub fn commit_button(s: &mut Cursive, model: Arc<Model>, dialog_name: String) {
  let select_view_items_len = s
    .find_name::<SelectView<Item>>("select_item")
    .unwrap()
    .len();

  match select_view_items_len {
    // if nothing has been added, notify the user and short circuit
    0 => {
      return s.add_layer(Dialog::info("List is empty. Nothing to commit."));
    }

    // otherwise send the contents off to the database
    // then clear the elements from the view
    n => {
      s.add_layer(
        Dialog::around(TextView::new(format!("Commit {} items?", n)))
          .button("Commit", move |s| {
            s.pop_layer();

            // does the actual heavy lifting
            handle_commit(s, dialog_name.clone(), model.clone());

            s.call_on_name(
              "select_item",
              |select_view: &mut SelectView<Item>| {
                select_view.clear();
              }
            );
          })
          .button("Cancel", |s| {
            s.pop_layer();
          })
      );
    }
  }
}

/// Gets every `Item` from the `SelectView`, then parse each into a `FoodItem`.
/// I'm having a feeling that having both types around is redundant, since I can just store everyting in `Item`,
/// but then the question arises how am I going to validate each of it's fields. Maybe later
fn handle_commit(s: &mut Cursive, dialog_name: String, model: Arc<Model>) {
  s.call_on_name("select_item", |select_view: &mut SelectView<Item>| {
    let mut food_items = Vec::<FoodItem>::with_capacity(10);

    for idx in 0..select_view.len() {
      let (_, item) = select_view.get_item(idx).unwrap();

      let mut food_item = FoodItem::from(item);

      food_item.set_table_name(dialog_name.clone());

      food_items.push(food_item);
    }

    // send items to database with prisma in a tokio thread
    tokio::spawn(async move {
      let create = food_items
        .into_iter()
        .map(|item| model.create_item(item, vec![]).exec())
        .collect::<Vec<_>>();

      // collect the result of the query and if any of them fail, let the user know about it
      // i'm interested in handling the errors in a create_many so that if any fails, I can rewind the whole transaction
      let mut results = Vec::<Result<Data, QueryError>>::with_capacity(10);

      for result in create {
        results.push(result.await);
      }

      // check whether every `Result` holds an `Ok`
      if results.into_iter().all(|item| item.is_ok()) {
        // ! Unwrapping because I wanna catch any threading related errors
        model
          .use_cb_sink(|s| {
            s.add_layer(Dialog::info("All set!"));
          })
          .unwrap();
      } else {
        // ! Unwrapping because I wanna catch any threading related errors
        model
          .use_cb_sink(|s| {
            s.add_layer(Dialog::info("Error occured when setting records"));
          })
          .unwrap();
      }
    });
  });
}

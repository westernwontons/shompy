use std::sync::Arc;

use cursive::{
  view::{Margins, Nameable, Resizable},
  views::{
    Dialog, DummyView, EditView, LinearLayout, NamedView, SelectView, TextView
  },
  Cursive, CursiveRunnable
};
use prisma_client_rust::{chrono::DateTime, QueryError};

use crate::{
  helpers::{add_button_cb, pop_cursive_layer},
  prisma::{food::Data, PrismaClient}
};

/// Container for PrismaClient and a callback sink provided by Cursive
/// which is used to call and build UI elements inside tokio tasks
pub struct Model {
  cb_sink: cursive::CbSink,
  prisma: PrismaClient
}

impl Model {
  /// Construct a `Model` struct wrapped in an `Arc`
  pub fn new(siv: &CursiveRunnable, prisma: PrismaClient) -> Arc<Self> {
    Arc::new(Self {
      cb_sink: siv.cb_sink().clone(),
      prisma
    })
  }

  /// Clone the Model
  pub fn clone(model: &Arc<Self>) -> Arc<Self> {
    Arc::clone(model)
  }

  /// Returns a reference to `PrismaClient`
  pub fn get_prisma(&self) -> &PrismaClient {
    &self.prisma
  }

  /// Use a cursive::CbSink in a closure
  pub fn use_cb_sink<F>(&self, f: F)
  where
    F: FnOnce(cursive::CbSink)
  {
    f(self.cb_sink.clone());
  }
}

/// Add a new named dialog to the menu tree.
/// Not necessary to configure with distinct names, because only one is active at a time
/// TODO dynamic sizing
pub fn new_dialog(
  model: Arc<Model>,
  dialog_name: &str,
  dialog_title: &'static str
) -> NamedView<Dialog> {
  Dialog::around(
    LinearLayout::vertical()
      .child(DummyView)
      .child(
        Dialog::around(new_menu_item())
          .button("Add", add_button_cb)
          .button("Commit", move |s| {
            on_commit(s, Model::clone(&model), dialog_title)
          })
          .button("Back", pop_cursive_layer)
      )
      .child(
        SelectView::<String>::new()
          .on_submit(delete_item)
          .with_name("select_item")
          .full_screen()
      )
  )
  .title(dialog_title)
  .with_name(dialog_name)
}

/// Return a new named menu item
pub fn new_menu_item() -> LinearLayout {
  LinearLayout::vertical()
    .child(DummyView)
    .child(
      Dialog::around(EditView::new().with_name("product_name"))
        .title("Name")
        .padding(Margins::lrtb(1, 1, 1, 1))
    )
    .child(DummyView)
    .child(
      Dialog::around(EditView::new().with_name("product_price"))
        .title("Price")
        .padding(Margins::lrtb(1, 1, 1, 1))
    )
    .child(DummyView)
    .child(
      Dialog::around(EditView::new().with_name("product_date_of_purchase"))
        .title("Purchase Date")
        .padding(Margins::lrtb(1, 1, 1, 1))
    )
}

/// Delete an item from the `select_item` list
pub fn delete_item(s: &mut Cursive, item: &str) {
  let selected = s.find_name::<SelectView<String>>("select_item").unwrap();

  // match on the currently selected item in `SelectView`
  match selected.selected_id() {
    None => {
      s.add_layer(
        Dialog::info("Nothing to remove!").button("Dismiss", pop_cursive_layer)
      );
    }

    Some(item) => {
      s.add_layer(
        // Popup window that asks for confirmation
        Dialog::around(TextView::new("Remove item from list?"))
          .button("Confirm", move |s| {
            s.find_name::<SelectView<String>>("select_item")
              .unwrap()
              .remove_item(item);

            // remove the window from the view upon action
            s.pop_layer();
          })
          .button("Cancel", pop_cursive_layer)
      );
    }
  }
}

fn handle_commit(s: &mut Cursive, model: Arc<Model>, dialog_name: &str) {
  let mut _len = 0;
  let select_view_items = s
    .call_on_name("select_item", |select_view: &mut SelectView<String>| {
      let len = select_view.len();
      _len = len;

      (0..len)
        .map(|index| {
          select_view
            .get_item(index)
            // we only need the items's name, not the index
            .map(|(_name, _index)| {
              _name
                .split("|")
                .map(|element| element.trim().to_string())
                .collect::<Vec<_>>()
            })
            .unwrap()
        })
        .collect::<Vec<_>>()
    })
    .unwrap();

  let _dialog_name = dialog_name.to_string();

  // handle the db update in a task
  tokio::spawn(async move {
    let prisma = model.get_prisma();

    let mut results_vec = Vec::<Result<Data, QueryError>>::with_capacity(_len);

    for mut element in select_view_items.into_iter() {
      element[2].push_str(" 00:00:00 +0000");

      let product_name = element[0].clone();
      let product_price = element[1].parse::<f64>().unwrap();
      let product_date_of_purchase =
        DateTime::parse_from_str(element[2].as_str(), "%Y/%m/%d %H:%M:%S %z")
          .unwrap();

      let result = prisma
        .food()
        .create(
          _dialog_name.clone(),
          product_name,
          product_price,
          product_date_of_purchase,
          vec![]
        )
        .exec()
        .await;

      results_vec.push(result);
    }

    // only keep the errors
    // later I might prompt to retry if appropriate
    results_vec.retain(|element| element.is_err());

    // let the user know about errors
    if !results_vec.is_empty() {
      model.use_cb_sink(|sink| {
        sink
          .send(Box::new(|s| {
            s.pop_layer();

            s.add_layer(Dialog::info("Error sending data to database"));
          }))
          .unwrap()
      });
    }
  });
}

/// Action to do on press of the `Commit` button
pub fn on_commit(s: &mut Cursive, model: Arc<Model>, dialog_title: &str) {
  let select_view_items_len = s
    .find_name::<SelectView<String>>("select_item")
    .unwrap()
    .len();

  // if nothing has been added, notify the user and short circuit
  if select_view_items_len == 0 {
    return s.add_layer(Dialog::info("List is empty. Nothing to commit."));
  }

  let _dialog_title = dialog_title.to_owned();
  s.add_layer(
    Dialog::around(TextView::new("Commit following items?"))
      .button("Commit", move |s| {
        handle_commit(s, Model::clone(&model), _dialog_title.as_str())
      })
      .dismiss_button("Cancel")
  )
}

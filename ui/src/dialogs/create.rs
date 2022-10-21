use std::sync::Arc;

use cursive::{
  view::{Margins, Nameable, Resizable},
  views::{
    Dialog, DummyView, EditView, LinearLayout, NamedView, SelectView, TextView
  },
  Cursive, CursiveRunnable
};
use prisma_client_rust::chrono::DateTime;

use crate::{
  helpers::{add_button_cb, pop_cursive_layer},
  prisma::PrismaClient
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
  dialog_name: String,
  dialog_title: String
) -> NamedView<Dialog> {
  let cloned_dialog_name = dialog_name.clone();
  Dialog::around(
    LinearLayout::vertical()
      .child(DummyView)
      .child(
        Dialog::around(new_menu_item())
          .button("Add", add_button_cb)
          .button("Commit", move |s| {
            on_commit(s, Model::clone(&model), cloned_dialog_name.clone())
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

/// Action to do on press of the `Commit` button
pub fn on_commit(s: &mut Cursive, model: Arc<Model>, dialog_name: String) {
  fn handle_commit(s: &mut Cursive, model: Arc<Model>, dialog_name: String) {
    let select_view_items = s
      .call_on_name("select_item", |select_view: &mut SelectView<String>| {
        let len = select_view.len();

        (0..len)
          .map(|index| {
            select_view
              .get_item(index)
              // we only need the items's name, not the index
              .map(|(_str, _string)| _str.to_owned())
              .unwrap()
          })
          .collect::<Vec<_>>()
      })
      .unwrap();

    // handle the db update in a task
    tokio::spawn(async move {
      let prisma = model.get_prisma();

      let result = prisma
        .food()
        .create(
          dialog_name,
          select_view_items[0].clone(),
          select_view_items[1].parse::<f64>().unwrap(),
          DateTime::parse_from_str(
            select_view_items[2].as_str(),
            "%Y/%m/%d %H:%M:%S"
          )
          .unwrap(),
          vec![]
        )
        .exec()
        .await;

      match result {
        Ok(ok) => todo!(),
        Err(err) => todo!()
      }
    });
  }

  let select_view_items_len = s
    .find_name::<SelectView<String>>("select_item")
    .unwrap()
    .len();

  // if nothing has been added, notify the user and short circuit
  if select_view_items_len == 0 {
    return s.add_layer(Dialog::info("List is empty. Nothing to commit."));
  }

  s.add_layer(
    Dialog::around(TextView::new("Commit following items?"))
      .button("Commit", move |s| {
        handle_commit(s, Model::clone(&model), dialog_name.clone())
      })
      .dismiss_button("Cancel")
  )
}

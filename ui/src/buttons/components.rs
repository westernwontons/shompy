mod dialog {
  use super::menu_item::create_menu_item;
  use crate::{
    buttons::{add_button, commit_button::commit_button},
    callbacks::delete_item,
    item::Item,
    model::Model
  };
  use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, DummyView, LinearLayout, NamedView, SelectView}
  };
  use std::sync::Arc;

  /// Add a new named dialog to the menu tree.
  /// Not necessary to configure with distinct names, because only one is active at a time
  pub fn create_dialog(
    model: Arc<Model>,
    dialog_name: &'static str,
    dialog_title: &'static str
  ) -> NamedView<Dialog> {
    Dialog::around(
      LinearLayout::vertical()
        .child(DummyView)
        .child(
          Dialog::around(create_menu_item())
            .button("Add", |s| add_button(s, "select_item"))
            .button("Commit", move |s| {
              commit_button(s, Model::clone(&model), dialog_title.to_string())
            })
            .button("Back", |s| {
              s.pop_layer();
            })
        )
        .child(
          SelectView::<Item>::new()
            .on_submit(|s, item| delete_item(s, item))
            .with_name("select_item")
            .full_screen()
        )
    )
    .title(dialog_title)
    .with_name(dialog_name)
  }
}

mod menu_item {
  use cursive::{
    view::{Margins, Nameable},
    views::{Dialog, DummyView, EditView, LinearLayout}
  };

  /// Return a new named menu item
  pub fn create_menu_item() -> LinearLayout {
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
        Dialog::around(EditView::new().with_name("product_amount"))
          .title("Amount")
          .padding(Margins::lrtb(1, 1, 1, 1))
      )
      .child(DummyView)
      .child(
        Dialog::around(EditView::new().with_name("product_date_of_purchase"))
          .title("Purchase Date")
          .padding(Margins::lrtb(1, 1, 1, 1))
      )
  }
}

pub mod leaf {
  use super::dialog::create_dialog;
  use crate::model::Model;
  use cursive::Cursive;
  use std::sync::Arc;

  pub fn create_leaf(
    s: &mut Cursive,
    model: Arc<Model>,
    name: &'static str,
    title: &'static str
  ) {
    // Pop previous layer
    s.pop_layer();

    s.add_layer(create_dialog(model, name, title));
  }
}

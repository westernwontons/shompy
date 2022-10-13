#![allow(unused_variables, unused_mut, dead_code)]
#![feature(async_closure)]

mod dialogs;
mod global_handlers;
mod prisma;

use cursive::{
  menu::Tree,
  view::{Nameable, Resizable},
  views::{Dialog, EditView, LinearLayout, SelectView, TextView},
  Cursive,
};
use global_handlers::{MenuConfig, QuitHandler};
use prisma::PrismaClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  // instantiate a Cursive instance
  let mut siv = cursive::default();

  // adds the exit on 'q' press with a confirmation window
  QuitHandler::default().add_global_quit_handler(&mut siv);

  // Configures the menu at the top
  MenuConfig::new(&mut siv, false).configure();

  // Storing stuff in an sqlite database for dev, prod will be postgres
  let _client = prisma::new_client().await;

  // Failing to instantiate a client should result in a quit (for now)
  let client = match _client {
    Ok(success) => success,
    Err(error) => {
      dialogs::quit_dialog(&mut siv);
      return siv.run();
    }
  };

  let model = Arc::new(Model {
    cb_sink: siv.cb_sink().clone(),
    prisma: client,
  });

  // top menu
  siv.menubar().add_subtree(
    "Add",
    Tree::new().leaf("Meat", move |s| {
      let cloned_arc_model = Arc::clone(&model);
      let cbsink = cloned_arc_model.cb_sink.clone();

      tokio::spawn(async move {
        match cloned_arc_model
          .prisma
          .user()
          .create("display_name".into(), Vec::new())
          .exec()
          .await
        {
          Ok(result) => {
            cbsink
              .send(Box::new(move |s| {
                s.add_layer(
                  Dialog::around(TextView::new(format!("{result:?}")))
                    .dismiss_button("Dismiss"),
                );
              }))
              .unwrap();
          }
          Err(err) => {
            cbsink
              .send(Box::new(move |s| {
                s.add_layer(
                  Dialog::around(TextView::new(format!("{err:?}")))
                    .dismiss_button("Dismiss"),
                );
              }))
              .unwrap();
          }
        }
      });

      s.add_layer(Dialog::around(
        LinearLayout::vertical()
          .child(
            LinearLayout::vertical().child(
              Dialog::around(
                EditView::new()
                  .on_submit(submit_item)
                  .with_name("add_item")
                  .fixed_width(40),
              )
              .button("Commit", on_commit)
              .button("Back", |s| {
                s.pop_layer();
              }),
            ),
          )
          .child(
            SelectView::<String>::new()
              .on_submit(delete_item)
              .with_name("select_item")
              .min_size::<(u8, u8)>((20, 24)),
          ),
      ));
    }),
  );

  siv.run();
}

struct Model {
  cb_sink: cursive::CbSink,
  prisma: PrismaClient,
}

fn submit_item(s: &mut Cursive, name: &str) {
  if name.is_empty() {
    s.add_layer(
      Dialog::around(TextView::new("Can't add empty item!"))
        .dismiss_button("Dismiss"),
    );
  } else {
    s.call_on_name("select_item", |select_item: &mut SelectView<String>| {
      select_item.add_item_str(name);
    });
  }
}

fn delete_item(s: &mut Cursive, item: &str) {
  let mut selected = s.find_name::<SelectView<String>>("select_item").unwrap();

  match selected.selected_id() {
    None => {
      s.add_layer(Dialog::info("Nothing to remove!"));
    }
    Some(item) => {
      selected.remove_item(item);
    }
  }
}

fn on_commit(s: &mut Cursive) {
  fn handle_commit(s: &mut Cursive) {
    // send to db with prisma
    todo!()
  }

  s.add_layer(
    Dialog::info("Commit following items?")
      .content(Dialog::around(SelectView::<String>::new()))
      .button("Commit", handle_commit)
      .dismiss_button("Cancel"),
  )
}

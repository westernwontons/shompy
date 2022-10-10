#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

mod global_callbacks;
mod layers;

use std::error::Error;

use cursive::reexports::crossbeam_channel::Select;
use cursive::traits::*;
use cursive::views::{
  Button, Dialog, DummyView, EditView, LinearLayout, ResizedView, SelectView,
};
use cursive::Cursive;

mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Storing stuff in an sqlite database for dev, prod will be postgres
  let client: PrismaClient = prisma::new_client().await?;

  let mut siv = cursive::default();

  // adds the exit on 'q' press command with a confirmation window
  global_callbacks::add_quit_handler(&mut siv);

  let select = SelectView::<String>::new()
    .on_submit(on_submit)
    .with_name("select")
    .fixed_size::<(u32, u32)>((10, 5));

  let buttons = LinearLayout::vertical()
    .child(Button::new("Add new", add_name))
    .child(Button::new("Delete", delete_name))
    .child(DummyView)
    .child(Button::new("Quit", Cursive::quit));

  siv.add_layer(layers(select, buttons));

  siv.run();

  Ok(())
}

fn on_submit(s: &mut Cursive, name: &str) {
  s.add_layer(
    Dialog::text(format!("Name: {}", name))
      .title(format!("{}'s info", name))
      .button("Back", |s| {
        s.pop_layer();
      }),
  );
}

fn add_name(s: &mut Cursive) {
  fn ok(s: &mut Cursive, name: &str) {
    s.call_on_name("select", |view: &mut SelectView<String>| {
      view.add_item_str(name);
    });
    s.pop_layer();
  }

  s.add_layer(
    Dialog::around(
      EditView::new()
        .on_submit(ok)
        .with_name("name")
        .fixed_width(10),
    )
    .title("Enter new name")
    .button("Ok", |s| {
      let name = s
        .call_on_name("name", |view: &mut EditView| view.get_content())
        .unwrap();
      ok(s, &name);
    })
    .button("Cancel", |s| {
      s.pop_layer();
    }),
  )
}

fn delete_name(s: &mut Cursive) {
  let mut select = s.find_name::<SelectView<String>>("select").unwrap();
  match select.selected_id() {
    None => s.add_layer(Dialog::info("No name to remove")),
    Some(focus) => {
      select.remove_item(focus);
    }
  }
}

fn layers(select: impl View, buttons: impl View) -> Dialog {
  Dialog::around(
    LinearLayout::horizontal()
      .child(select)
      .child(DummyView)
      .child(buttons),
  )
  .title("Select a profile")
}

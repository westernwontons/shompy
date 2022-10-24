#![allow(unused_variables, unused_mut, dead_code)]
#![feature(slice_concat_trait)]

mod buttons;
mod callbacks;
mod item;
mod model;
mod prisma;

use buttons::create_leaf;
use cursive::{
  menu::Tree,
  views::{Dialog, TextView},
  Cursive
};
use model::Model;

#[tokio::main]
async fn main() {
  // instantiate a Cursive instance
  let mut siv = cursive::default();

  // adds the exit on 'q' press with a confirmation window
  siv.set_global_callback('q', move |s| {
    s.add_layer(
      Dialog::text("Are you sure you want to quit?")
        .title("Quit")
        .button("Yes", |s| s.quit())
        .button("No", |s| {
          s.pop_layer();
        })
    )
  });

  // Configures the menu at the top
  siv.set_autohide_menu(true);
  siv.add_global_callback(cursive::event::Key::Esc, |s| {
    s.select_menubar();
  });

  // Failing to instantiate a client should result in a quit (for now)
  let client = match prisma::new_client().await {
    Ok(success) => success,
    Err(error) => {
      siv.add_layer(
        Dialog::around(TextView::new("Failed to initialize prisma. Quitting"))
          .button("Quit", Cursive::quit)
      );

      return siv.run();
    }
  };

  let model = Model::new(&siv, client);

  // separate, cloned `Model` for every `leaf`
  let meat_model = Model::clone(&model);
  let vegetable_model = Model::clone(&model);
  let fruit_model = Model::clone(&model);
  let side_model = Model::clone(&model);
  let pasta_model = Model::clone(&model);
  let bread_model = Model::clone(&model);
  let ingredient_model = Model::clone(&model);

  // top menu
  siv.menubar().add_subtree(
    "Food",
    Tree::new()
      .leaf("Meat", move |s| {
        create_leaf(s, Model::clone(&meat_model), "meat_dialog", "Meat")
      })
      .leaf("Vegetable", move |s| {
        create_leaf(
          s,
          Model::clone(&vegetable_model),
          "vegetable_dialog",
          "Vegetable"
        )
      })
      .leaf("Fruit", move |s| {
        create_leaf(s, Model::clone(&fruit_model), "fruit_dialog", "Fruit")
      })
      .leaf("Side", move |s| {
        create_leaf(s, Model::clone(&side_model), "side_dialog", "Side")
      })
      .leaf("Bread", move |s| {
        create_leaf(s, Model::clone(&bread_model), "bread_dialog", "Bread")
      })
      .leaf("Pasta", move |s| {
        create_leaf(s, Model::clone(&pasta_model), "pasta_dialog", "Pasta");
      })
      .leaf("Ingredients", move |s| {
        create_leaf(
          s,
          Model::clone(&ingredient_model),
          "ingredient_dialog",
          "Ingredient"
        )
      })
  );

  siv.run();
}

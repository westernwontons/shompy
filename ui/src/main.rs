#![allow(unused_variables, unused_mut, dead_code)]

mod dialogs;
mod global_handlers;
mod helpers;
mod prisma;

use cursive::menu::Tree;
use dialogs::{new_dialog, Model};
use global_handlers::{MenuConfig, QuitHandler};

#[tokio::main]
async fn main() {
  // instantiate a Cursive instance
  let mut siv = cursive::default();

  // adds the exit on 'q' press with a confirmation window
  QuitHandler::default().add_global_quit_handler(&mut siv);

  // Configures the menu at the top
  MenuConfig::new(&mut siv, false).configure();

  // Failing to instantiate a client should result in a quit (for now)
  let client = match prisma::new_client().await {
    Ok(success) => success,
    Err(error) => {
      dialogs::prisma_init_error_quit_dialog(&mut siv);
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
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&meat_model),
          "meat_dialog",
          "Meat"
        ));
      })
      .leaf("Vegetable", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&vegetable_model),
          "vegetable_dialog",
          "Vegetable"
        ));
      })
      .leaf("Fruit", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&fruit_model),
          "fruit_dialog",
          "Fruit"
        ));
      })
      .leaf("Side", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&side_model),
          "side_dialog",
          "Side"
        ));
      })
      .leaf("Bread", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&bread_model),
          "bread_dialog",
          "Bread"
        ));
      })
      .leaf("Pasta", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&pasta_model),
          "pasta_dialog",
          "Pasta"
        ));
      })
      .leaf("Ingredients", move |s| {
        // Pop previous layer
        s.pop_layer();

        s.add_layer(new_dialog(
          Model::clone(&ingredient_model),
          "ingredient_dialog",
          "Ingredient"
        ));
      })
  );

  siv.run();
}

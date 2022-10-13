#![allow(unused_variables, unused_mut, dead_code)]

mod dialogs;
mod global_handlers;
mod prisma;

use global_handlers::{MenuConfig, QuitHandler};

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

  siv.run();
}

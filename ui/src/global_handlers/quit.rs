use cursive::{views::Dialog, CursiveRunnable};

/// Struct that's concerned about all the ways to let the user quit the application
pub struct QuitHandler {
  text: &'static str,
  title: &'static str,
}

/// Default gives some base text unless else is provided
impl Default for QuitHandler {
  fn default() -> Self {
    Self {
      text: "Are you sure you want to quit?".into(),
      title: "Quit".into(),
    }
  }
}

impl QuitHandler {
  /// Gives a title for the window and some text to display
  pub fn new(title: &'static str, text: &'static str) -> Self {
    Self { title, text }
  }

  /// Register a global quit handler when `q` is pressed.
  /// A window will pop up to ask for confirmation
  pub fn add_global_quit_handler(self, siv: &mut CursiveRunnable) {
    siv.set_global_callback('q', move |s| {
      s.add_layer(
        Dialog::text(self.text)
          .title(self.title)
          .button("Yes", |s| s.quit())
          .button("No", |s| {
            s.pop_layer();
          }),
      )
    });
  }
}

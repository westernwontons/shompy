use cursive::{event::Key, CursiveRunnable};

pub struct MenuConfig<'a> {
  autohide: bool,
  cursive: &'a mut CursiveRunnable,
}

impl<'a> MenuConfig<'a> {
  pub fn new(cursive: &'a mut CursiveRunnable, autohide: bool) -> Self {
    Self { autohide, cursive }
  }

  pub fn configure(&mut self) {
    self.cursive.set_autohide_menu(self.autohide);
    self.menu_activator();
  }

  fn menu_activator(&mut self) {
    self.cursive.add_global_callback(Key::Esc, |s| {
      s.select_menubar();
    });
  }
}

use cursive::{
  views::{Dialog, TextView},
  Cursive, CursiveRunnable
};

/// A dialog that's triggered if Prisma initialization fails
pub fn prisma_init_error_quit_dialog(cursive: &mut CursiveRunnable) {
  cursive.add_layer(
    Dialog::around(TextView::new("Failed to initialize prisma. Quitting"))
      .button("Quit", Cursive::quit)
  )
}

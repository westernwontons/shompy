use cursive::{
  views::{Dialog, TextView},
  CursiveRunnable,
};

pub fn quit_dialog(cursive: &mut CursiveRunnable) {
  cursive.add_layer(
    Dialog::around(TextView::new("Are you sure you want to quit?"))
      .button("Yes", |s| {
        s.quit();
      })
      .button("No", |s| {
        s.pop_layer();
      }),
  )
}

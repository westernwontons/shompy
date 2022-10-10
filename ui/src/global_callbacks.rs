use cursive::{views::Dialog, Cursive, CursiveRunnable};

pub fn add_quit_handler(siv: &mut CursiveRunnable) {
  siv.set_global_callback('q', |s| {
    s.add_layer(
      Dialog::text("Are you sure you want to quit?")
        .button("YES!", |s| s.quit())
        .button("No", |s| {
          s.pop_layer();
        }),
    )
  });
}

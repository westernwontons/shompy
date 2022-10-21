use cursive::Cursive;

/// Pop a cursive layer
pub fn pop_cursive_layer(s: &mut Cursive) {
  s.pop_layer();
}

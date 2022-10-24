use std::sync::Arc;

use cursive::{
  reexports::crossbeam_channel::SendError, Cursive, CursiveRunnable
};
use prisma_client_rust::Create;

use crate::{
  item::FoodItem,
  prisma::{
    food::{Data, SetParam, WithParam},
    PrismaClient
  }
};

/// Container for PrismaClient and a callback sink provided by Cursive
/// which is used to call and build UI elements inside tokio tasks
pub struct Model {
  cb_sink: cursive::CbSink,
  prisma: PrismaClient
}

impl Model {
  /// Construct a `Model` struct wrapped in an `Arc`
  pub fn new(siv: &CursiveRunnable, prisma: PrismaClient) -> Arc<Self> {
    Arc::new(Self {
      cb_sink: siv.cb_sink().clone(),
      prisma
    })
  }

  pub fn clone(model: &Arc<Self>) -> Arc<Self> {
    Arc::clone(model)
  }

  /// Creates a record that will need to be executed in an async context
  /// Basically, call `.exec()` on the result of this fn
  pub fn create_item(
    &self,
    food_item: FoodItem,
    params: Vec<SetParam>
  ) -> Create<SetParam, WithParam, Data> {
    let (table_name, name, price, amount, total, purchase_date) =
      food_item.to_parts();

    self.prisma.food().create(
      table_name,
      name,
      price,
      amount,
      total,
      purchase_date,
      params
    )
  }

  /// The callback sink let's you call methods on `Cursive` in a multithreaded context
  /// Uses a passed closure that receives `Cursive` as an argument
  pub fn use_cb_sink<F>(
    &self,
    f: F
  ) -> Result<(), SendError<Box<dyn FnOnce(&mut Cursive) + Send>>>
  where
    F: FnOnce(&mut Cursive) + Send + 'static
  {
    self.cb_sink.clone().send(Box::new(f))
  }
}

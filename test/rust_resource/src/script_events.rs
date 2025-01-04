use std::{
  cell::RefCell,
  rc::Rc,
  sync::{Arc, RwLock},
};

use altv::mvalue::DynMValueArgs;

pub(crate) fn test_script_events() {
  use altv::events;
  let controller: Arc<RwLock<Option<events::LocalEventController>>> = Default::default();

  let controller_ = controller.clone();
  controller
    .write()
    .unwrap()
    .replace(events::on("test", move |context| {
      dbg!(context);
      let borrow_mut = &mut controller_.write().unwrap();
      let controller = borrow_mut.as_mut().unwrap();
      dbg!(&controller);
      assert!(controller.destroy().is_ok());
      dbg!(&controller);
      assert!(controller.destroy().is_err());
    }));

  let args: DynMValueArgs = &[&true, &false, &123_i32, &123.5_f32];
  events::emit("test", args).unwrap();
  events::emit("test", args).unwrap();
}

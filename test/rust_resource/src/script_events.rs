use std::{cell::RefCell, rc::Rc};

use altv::mvalue::{DynMValueArgs};

pub(crate) fn test_script_events() {
    use altv::events;
    let controller: Rc<RefCell<Option<events::LocalEventController>>> = Rc::new(RefCell::new(None));

    let controller_ = controller.clone();
    controller
        .borrow_mut()
        .replace(events::on("test", move |c| {
            dbg!(c);
            let borrow_mut = &mut controller_.borrow_mut();
            let controller = borrow_mut.as_mut().unwrap();
            dbg!(&controller);
            assert!(controller.destroy().is_ok());
            dbg!(&controller);
            assert!(controller.destroy().is_err());
        }));

    let args: DynMValueArgs = &[&true, &false];
    events::emit("test", args).unwrap();
    events::emit("test", args).unwrap();
}

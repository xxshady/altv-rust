use std::{
  collections::HashMap,
  sync::{LazyLock, Mutex, RwLock},
  thread::ThreadId,
};

use core_shared::ResourceName;

use crate::{helpers::current_thread_id, resource_manager::ResourceManager, Module};

static SCHEDULE_START_INSTANCE: LazyLock<RwLock<ScheduleStart>> = LazyLock::new(Default::default);

static MAIN_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);

pub fn init_main_thread() {
  let mut main_thread_id = MAIN_THREAD_ID.lock().unwrap();
  *main_thread_id = Some(current_thread_id());
}

#[derive(Default)]
pub struct ScheduleStart {
  pub resources: HashMap<ResourceName, ResourceSchedule>,
}

impl ScheduleStart {
  pub fn add(resource_name: ResourceName, schedule: ResourceSchedule) {
    SCHEDULE_START_INSTANCE
      .write()
      .unwrap()
      .resources
      .insert(resource_name, schedule);
  }

  pub fn start_if_not_already(resource_name: String) -> ResourceStarted {
    if current_thread_id() != *MAIN_THREAD_ID.lock().unwrap().as_ref().unwrap() {
      return ResourceStarted::No;
    }

    let mut instance = SCHEDULE_START_INSTANCE.write().unwrap();

    let Some(resource) = instance.resources.remove(&resource_name) else {
      return ResourceStarted::No;
    };

    // assert_ne!(
    //   resource.thread_id,
    //   current_thread_id(),
    //   "Resources must be started from main thread"
    // );

    dbg!(&resource_name);
    ResourceManager::start_resource(resource_name, resource.full_main_path);
    ResourceStarted::Yes
  }

  pub fn start_all() {
    // TODO: better impl

    let keys: Vec<String> = SCHEDULE_START_INSTANCE
      .read()
      .unwrap()
      .resources
      .keys()
      .cloned()
      .collect();
    for resource_name in keys {
      Self::start_if_not_already(resource_name);
    }
  }
}

pub struct ResourceSchedule {
  pub full_main_path: String,
}

pub fn avoid_self_resource_start(
  resource_name: &str,
  event_type: altv_sdk::EventType,
) -> AvoidEvent {
  if event_type != altv_sdk::EventType::ResourceStart {
    return AvoidEvent::No;
  }

  match ScheduleStart::start_if_not_already(resource_name.to_owned()) {
    ResourceStarted::Yes => AvoidEvent::No,
    ResourceStarted::No => AvoidEvent::Yes,
  }
}

pub enum ResourceStarted {
  Yes,
  No,
}

pub enum AvoidEvent {
  Yes,
  No,
}

pub struct ModuleWrapper(pub Module);

// TODO: its unsound
// SAFETY: see thread id check in start_if_not_already
unsafe impl Send for ModuleWrapper {}
unsafe impl Sync for ModuleWrapper {}

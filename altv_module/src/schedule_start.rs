use std::{
  collections::HashMap,
  sync::{LazyLock, RwLock},
  thread::ThreadId,
};

use core_shared::ResourceName;

use crate::{helpers::current_thread_id, resource_manager::ResourceManager, Module};

static SCHEDULE_START_INSTANCE: LazyLock<RwLock<ScheduleStart>> = LazyLock::new(Default::default);

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
    let instance = SCHEDULE_START_INSTANCE.read().unwrap();

    let Some(resource) = instance.resources.get(&resource_name) else {
      return ResourceStarted::No;
    };

    if resource.thread_id == current_thread_id() {
      return ResourceStarted::No;
    }

    drop(instance);

    let Some(resource) = SCHEDULE_START_INSTANCE
      .write()
      .unwrap()
      .resources
      .remove(&resource_name)
    else {
      return ResourceStarted::No;
    };

    // assert_ne!(
    //   resource.thread_id,
    //   current_thread_id(),
    //   "Resources must be started from main thread"
    // );

    ResourceManager::start_resource(resource_name, resource.module.0);
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
  pub module: ModuleWrapper,
  pub thread_id: ThreadId,
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

// SAFETY: see thread id check in start_if_not_already
unsafe impl Send for ModuleWrapper {}
unsafe impl Sync for ModuleWrapper {}

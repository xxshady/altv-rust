use std::{path::PathBuf, rc::Rc};

pub struct ResourceManager {
    // TODO: private
    pub resources: Vec<Rc<alt::MainResource>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { resources: vec![] }
    }

    pub fn add(&mut self, path: PathBuf, resource: Rc<alt::MainResource>) {
        self.resources.push(resource);
    }
}

// pub struct Resource {
//     path: std::path::PathBuf,
// }

use std::{collections::HashMap, path::PathBuf};

pub struct ResourceManager {
    // TODO: private
    pub resources: HashMap<PathBuf, alt::MainResource>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: PathBuf, resource: alt::MainResource) {
        self.resources.insert(path, resource);
    }
}

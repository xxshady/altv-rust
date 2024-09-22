use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Id(u64);

#[derive(Default)]
pub struct IdProvider {
  current: u64,
}

impl IdProvider {
  pub fn next(&mut self) -> Id {
    self.current = self.current.checked_add(1).unwrap();
    Id(self.current)
  }
}

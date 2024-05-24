use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct SuperDrill {
    pub(crate) tier: u8,
}
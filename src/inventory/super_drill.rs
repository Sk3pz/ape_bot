use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SuperDrill {
    pub(crate) tier: u8,
}
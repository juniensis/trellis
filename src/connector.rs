use serde::{Deserialize, Serialize};

use crate::world::id::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Connector {
    id: Id,
    from: Id,
    to: Id,
}

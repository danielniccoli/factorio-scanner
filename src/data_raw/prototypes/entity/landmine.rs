use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/LandMinePrototype`](https://lua-api.factorio.com/latest/prototypes/LandMinePrototype.html)
pub type LandMinePrototype = EntityWithOwnerPrototype<LandMineData>;

/// [`Prototypes/LandMinePrototype`](https://lua-api.factorio.com/latest/prototypes/LandMinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LandMineData {
    pub picture_safe: Sprite,
    pub picture_set: Sprite,
    pub trigger_radius: f64,

    pub picture_set_enemy: Option<Sprite>,

    #[serde(
        default = "helper::u32_120",
        skip_serializing_if = "helper::is_120_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub timeout: u32,

    pub ammo_category: Option<AmmoCategoryID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub force_die_on_attack: bool,

    #[serde(
        default = "ForceCondition::enemy",
        skip_serializing_if = "ForceCondition::is_enemy"
    )]
    pub trigger_force: ForceCondition,

    pub trigger_collision_mask: Option<CollisionMask>,
    // not implemented
    // pub action: Option<Trigger>,
}

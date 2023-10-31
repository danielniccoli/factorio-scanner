use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/LandMinePrototype`](https://lua-api.factorio.com/latest/prototypes/LandMinePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LandMinePrototype(EntityWithOwnerPrototype<LandMineData>);

impl super::Renderable for LandMinePrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

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

impl super::Renderable for LandMineData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture_set.render(
            options.factorio_dir,
            &options.used_mods,
            image_cache,
            &options.into(),
        )
    }
}

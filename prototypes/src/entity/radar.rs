use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RadarPrototype(EntityWithOwnerPrototype<RadarData>);

impl Deref for RadarPrototype {
    type Target = EntityWithOwnerPrototype<RadarData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RadarPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for RadarPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
    }
}

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarData {
    pub energy_usage: Energy,
    pub energy_per_sector: Energy,
    pub energy_per_nearby_scan: Energy,
    pub energy_source: AnyEnergySource,
    pub pictures: RotatedSprite,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance_of_sector_revealed: u32,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance_of_nearby_sector_revealed: u32,

    pub radius_minimap_visualisation_color: Option<Color>,

    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub rotation_speed: f64,
}

impl super::Renderable for RadarData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.pictures
            .render(used_mods, image_cache, &options.into())
    }
}

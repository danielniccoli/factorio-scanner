use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/TransportBeltConnectablePrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltConnectablePrototype.html)
pub type TransportBeltConnectablePrototype<G, T> =
    EntityWithOwnerPrototype<TransportBeltConnectableData<G, T>>;

/// [`Prototypes/TransportBeltConnectablePrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltConnectablePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectableData<G, T> {
    pub speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_speed_coefficient: f64,

    #[serde(flatten)]
    pub graphics_set: G,

    #[serde(flatten)]
    pub child: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BeltGraphics {
    BeltAnimationSet {
        belt_animation_set: TransportBeltAnimationSet,
    },
    Individual {
        belt_horizontal: Animation,
        belt_vertical: Animation,

        ending_top: Animation,
        ending_bottom: Animation,
        ending_side: Animation,

        starting_top: Animation,
        starting_bottom: Animation,
        starting_side: Animation,

        ending_patch: Sprite4Way,

        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        ends_with_stopper: bool,
    },
}

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
pub type LinkedBeltPrototype = TransportBeltConnectablePrototype<BeltGraphics, LinkedBeltData>;

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedBeltData {
    pub structure: LinkedBeltStructure,

    pub structure_render_layer: Option<RenderLayer>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_clone_connection: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_blueprint_connection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_side_loading: bool,
    // TODO: collision_mask overridden
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedBeltStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,

    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,

    pub direction_in_side_loading: Option<Sprite4Way>,
    pub direction_out_side_loading: Option<Sprite4Way>,
}

/// [`Prototypes/LoaderPrototype`](https://lua-api.factorio.com/latest/prototypes/LoaderPrototype.html)
pub type LoaderPrototype<T> = TransportBeltConnectablePrototype<BeltGraphics, LoaderData<T>>;

/// [`Prototypes/LoaderPrototype`](https://lua-api.factorio.com/latest/prototypes/LoaderPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderData<T> {
    pub structure: LoaderStructure,
    pub filter_count: u8,

    // TODO: default
    pub structure_render_layer: Option<RenderLayer>,

    #[serde(
        default = "helper::f64_1_5",
        skip_serializing_if = "helper::is_1_5_f64"
    )]
    pub container_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_rail_interaction: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_container_interaction: bool,

    //pub belt_length: f64, // -> moved to specific variants
    pub energy_source: Option<AnyEnergySource>, // any except burner
    pub energy_per_item: Option<Energy>,

    #[serde(flatten)]
    pub child: T,
}

/// [`Types/LoaderStructure`](https://lua-api.factorio.com/latest/types/LoaderStructure.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,

    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
pub type Loader1x1Prototype = LoaderPrototype<Loader1x1Data>;

// TODO: loaders `belt_length` is not actually hardcoded but defaults to a internal hardcoded value instead..

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x1Data {
    // hardcoded to 0, validate this?
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub belt_length: f64,
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
pub type Loader1x2Prototype = LoaderPrototype<Loader1x2Data>;

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x2Data {
    // hardcoded to 0.5, validate this?
    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub belt_length: f64,
}

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
pub type SplitterPrototype = TransportBeltConnectablePrototype<BeltGraphics, SplitterData>;

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SplitterData {
    pub structure: Animation4Way,
    pub structure_patch: Option<Animation4Way>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub structure_animation_speed_coefficient: f64,

    #[serde(default = "helper::u32_10", skip_serializing_if = "helper::is_10_u32")]
    pub structure_animation_movement_cooldown: u32,
}

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
pub type TransportBeltPrototype =
    TransportBeltConnectablePrototype<BeltGraphicsWithCorners, TransportBeltData>;

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltData {
    pub connector_frame_sprites: TransportBeltConnectorFrame,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_wire_connection_point: Vec<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>,

    pub related_underground_belt: Option<EntityID>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BeltGraphicsWithCorners {
    BeltAnimationSetWithCorners {
        belt_animation_set: TransportBeltAnimationSetWithCorners,
    },
    Animations {
        animations: RotatedAnimation, // must have 12 animations
    },
}

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
pub type UndergroundBeltPrototype =
    TransportBeltConnectablePrototype<BeltGraphics, UndergroundBeltData>;

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltData {
    pub max_distance: u8,
    pub structure: UndergroundBeltStructure,
    pub underground_sprite: Sprite,
    pub underground_remove_belts_sprite: Option<Sprite>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,
    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,
    pub direction_in_side_loading: Option<Sprite4Way>,
    pub direction_out_side_loading: Option<Sprite4Way>,
}

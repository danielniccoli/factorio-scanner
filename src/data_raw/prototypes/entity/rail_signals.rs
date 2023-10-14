use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/RailSignalBasePrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalBasePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailSignalBasePrototype<T: super::Renderable>(
    EntityWithOwnerPrototype<RailSignalBaseData<T>>,
);

impl<T: super::Renderable> super::Renderable for RailSignalBasePrototype<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailSignalBasePrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalBasePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalBaseData<T: super::Renderable> {
    pub animation: RotatedAnimation,
    pub rail_piece: Option<Animation>,
    pub red_light: Option<LightDefinition>,
    pub green_light: Option<LightDefinition>,
    pub orange_light: Option<LightDefinition>,
    pub default_red_output_signal: Option<SignalIDConnector>,
    pub default_green_output_signal: Option<SignalIDConnector>,
    pub default_orange_output_signal: Option<SignalIDConnector>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>,

    #[serde(flatten)]
    pub child: T,
}

impl<T: super::Renderable> super::Renderable for RailSignalBaseData<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailChainSignalPrototype(RailSignalBasePrototype<RailChainSignalData>);

impl super::Renderable for RailChainSignalPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailChainSignalData {
    pub selection_box_offsets: Vec<Vector>,
    pub blue_light: Option<LightDefinition>,
    pub default_blue_output_signal: Option<SignalIDConnector>,
}

impl super::Renderable for RailChainSignalData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailSignalPrototype(EntityWithOwnerPrototype<RailSignalData>);

impl super::Renderable for RailSignalPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalData {}

impl super::Renderable for RailSignalData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

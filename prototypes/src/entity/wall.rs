use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/WallPrototype`](https://lua-api.factorio.com/latest/prototypes/WallPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct WallPrototype(EntityWithOwnerPrototype<WallData>);

impl super::Renderable for WallPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/WallPrototype`](https://lua-api.factorio.com/latest/prototypes/WallPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WallData {
    pub pictures: WallPictures,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub visual_merge_group: u32,

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub default_output_signal: Option<SignalIDConnector>,

    pub wall_diode_green: Option<Sprite4Way>,
    pub wall_diode_red: Option<Sprite4Way>,
    pub wall_diode_green_light_top: Option<LightDefinition>,
    pub wall_diode_green_light_right: Option<LightDefinition>,
    pub wall_diode_green_light_bottom: Option<LightDefinition>,
    pub wall_diode_green_light_left: Option<LightDefinition>,
    pub wall_diode_red_light_top: Option<LightDefinition>,
    pub wall_diode_red_light_right: Option<LightDefinition>,
    pub wall_diode_red_light_bottom: Option<LightDefinition>,
    pub wall_diode_red_light_left: Option<LightDefinition>,

    pub connected_gate_visualization: Option<Sprite>,
}

impl super::Renderable for WallData {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        let core = match options.connections.unwrap_or_default() {
            ConnectedDirections::None | ConnectedDirections::Up => &self.pictures.single,
            ConnectedDirections::Down | ConnectedDirections::UpDown => {
                &self.pictures.straight_vertical
            }
            ConnectedDirections::Left | ConnectedDirections::UpLeft => &self.pictures.ending_left,
            ConnectedDirections::Right | ConnectedDirections::UpRight => {
                &self.pictures.ending_right
            }
            ConnectedDirections::DownLeft | ConnectedDirections::UpDownLeft => {
                &self.pictures.corner_left_down
            }
            ConnectedDirections::DownRight | ConnectedDirections::UpDownRight => {
                &self.pictures.corner_right_down
            }
            ConnectedDirections::LeftRight | ConnectedDirections::UpLeftRight => {
                &self.pictures.straight_horizontal
            }
            ConnectedDirections::DownLeftRight | ConnectedDirections::All => &self.pictures.t_up,
        }
        .render(
            options.factorio_dir,
            &options.used_mods,
            image_cache,
            &options.into(),
        );

        // TODO: fillings
        let mut gate_connection_north: Option<GraphicsOutput> = None;
        let mut gate_connection_south: Option<GraphicsOutput> = None;
        let mut gate_connection_east: Option<GraphicsOutput> = None;
        let mut gate_connection_west: Option<GraphicsOutput> = None;

        for gate_direction in &options.connected_gates {
            let gate_opts = &super::RenderOpts {
                direction: Some(*gate_direction),
                ..options.clone()
            };

            let tmp = merge_renders(&[
                self.pictures.gate_connection_patch.as_ref().and_then(|s| {
                    s.render(
                        options.factorio_dir,
                        &options.used_mods,
                        image_cache,
                        &gate_opts.into(),
                    )
                }),
                self.wall_diode_red.as_ref().and_then(|s| {
                    s.render(
                        options.factorio_dir,
                        &options.used_mods,
                        image_cache,
                        &gate_opts.into(),
                    )
                }),
            ]);

            match gate_direction {
                Direction::North => gate_connection_north = tmp,
                Direction::South => gate_connection_south = tmp,
                Direction::East => gate_connection_east = tmp,
                Direction::West => gate_connection_west = tmp,
                _ => unreachable!("Invalid gate direction: {:?}", gate_direction),
            }
        }

        merge_renders(&[
            core,
            gate_connection_north,
            gate_connection_west,
            gate_connection_east,
            gate_connection_south,
        ])
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WallPictures {
    pub single: SpriteVariations,
    pub straight_vertical: SpriteVariations,
    pub straight_horizontal: SpriteVariations,
    pub corner_right_down: SpriteVariations,
    pub corner_left_down: SpriteVariations,
    pub t_up: SpriteVariations,
    pub ending_right: SpriteVariations,
    pub ending_left: SpriteVariations,
    pub filling: Option<SpriteVariations>,
    pub water_connection_patch: Option<Sprite4Way>,
    pub gate_connection_patch: Option<Sprite4Way>,
}

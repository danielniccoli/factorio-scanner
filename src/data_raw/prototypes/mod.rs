#![allow(clippy::struct_excessive_bools, clippy::module_name_repetitions)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::helper;
#[allow(clippy::wildcard_imports)]
use super::types::{Animation, BoxSpecification, LocalisedString, Order, Sprite};

mod entity;
pub use entity::*;

/// [`Prototypes/PrototypeBase`](https://lua-api.factorio.com/latest/PrototypeBase.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePrototype<T> {
    /// type can effectively be ignored, as it should be enforced by the struct/enum types itself
    #[serde(rename = "type")]
    pub type_: String,

    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub order: Order,

    pub localised_name: Option<LocalisedString>,
    pub localised_description: Option<LocalisedString>,

    #[serde(flatten)]
    pub child: T,
}

/// [`Prototypes/UtilitySprites/CursorBoxSpecification`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorBoxSpecification {
    pub regular: Vec<BoxSpecification>,
    pub not_allowed: Vec<BoxSpecification>,
    pub copy: Vec<BoxSpecification>,
    pub electricity: Vec<BoxSpecification>,
    pub logistics: Vec<BoxSpecification>,
    pub pair: Vec<BoxSpecification>,
    pub train_visualization: Vec<BoxSpecification>,
    pub blueprint_snap_rectangle: Vec<BoxSpecification>,
}

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UtilitySpritesData {
    pub cursor_box: CursorBoxSpecification,
    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,

    #[serde(flatten)]
    pub sprites: HashMap<String, Sprite>,
    /*
    pub achievement_label_failed: Sprite,
    pub achievement_label_locked: Sprite,
    pub achievement_label_unlocked: Sprite,
    pub achievement_label_unlocked_off: Sprite,
    pub add: Sprite,
    pub alert_arrow: Sprite,
    pub ammo_damage_modifier_constant: Option<Sprite>,
    pub ammo_damage_modifier_icon: Sprite,
    pub ammo_icon: Sprite,
    pub and_or: Sprite,
    pub area_icon: Sprite,
    pub arrow_button: Animation,
    pub artillery_range_modifier_constant: Option<Sprite>,
    pub artillery_range_modifier_icon: Sprite,
    pub bar_gray_pip: Sprite,
    pub battery: Sprite,
    pub bookmark: Sprite,
    pub brush_circle_shape: Sprite,
    pub brush_icon: Sprite,
    pub brush_square_shape: Sprite,
    pub cable_editor_icon: Sprite,
    pub center: Sprite,
    pub change_recipe: Sprite,
    pub character_additional_mining_categories_modifier_constant: Option<Sprite>,
    pub character_additional_mining_categories_modifier_icon: Sprite,
    pub character_build_distance_modifier_constant: Option<Sprite>,
    pub character_build_distance_modifier_icon: Sprite,
    pub character_crafting_speed_modifier_constant: Option<Sprite>,
    pub character_crafting_speed_modifier_icon: Sprite,
    pub character_health_bonus_modifier_constant: Option<Sprite>,
    pub character_health_bonus_modifier_icon: Sprite,
    pub character_inventory_slots_bonus_modifier_constant: Option<Sprite>,
    pub character_inventory_slots_bonus_modifier_icon: Sprite,
    pub character_item_drop_distance_modifier_constant: Option<Sprite>,
    pub character_item_drop_distance_modifier_icon: Sprite,
    pub character_item_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_item_pickup_distance_modifier_icon: Sprite,
    pub character_logistic_requests_modifier_constant: Option<Sprite>,
    pub character_logistic_requests_modifier_icon: Sprite,
    pub character_logistic_slots_modifier_constant: Option<Sprite>,
    pub character_logistic_slots_modifier_icon: Sprite,
    pub character_logistic_trash_slots_modifier_constant: Option<Sprite>,
    pub character_logistic_trash_slots_modifier_icon: Sprite,
    pub character_loot_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_loot_pickup_distance_modifier_icon: Sprite,
    pub character_mining_speed_modifier_constant: Option<Sprite>,
    pub character_mining_speed_modifier_icon: Sprite,
    pub character_reach_distance_modifier_constant: Option<Sprite>,
    pub character_reach_distance_modifier_icon: Sprite,
    pub character_resource_reach_distance_modifier_constant: Option<Sprite>,
    pub character_resource_reach_distance_modifier_icon: Sprite,
    pub character_running_speed_modifier_constant: Option<Sprite>,
    pub character_running_speed_modifier_icon: Sprite,
    pub check_mark: Sprite,
    pub check_mark_dark_green: Sprite,
    pub check_mark_green: Sprite,
    pub check_mark_white: Sprite,
    pub circuit_network_panel_black: Sprite,
    pub circuit_network_panel_white: Sprite,
    pub cliff_editor_icon: Sprite,
    pub clock: Sprite,
    pub clone: Sprite,
    pub clone_editor_icon: Sprite,
    pub close_black: Sprite,
    pub close_fat: Sprite,
    pub close_map_preview: Sprite,
    pub close_white: Sprite,
    pub clouds: Animation,
    pub collapse: Sprite,
    pub collapse_dark: Sprite,
    pub color_effect: Sprite,
    pub color_picker: Sprite,
    pub confirm_slot: Sprite,
    pub construction_radius_visualization: Sprite,
    pub controller_joycon_a: Sprite,
    pub controller_joycon_b: Sprite,
    pub controller_joycon_back: Sprite,
    pub controller_joycon_black_a: Sprite,
    pub controller_joycon_black_b: Sprite,
    pub controller_joycon_black_back: Sprite,
    pub controller_joycon_black_dpdown: Sprite,
    pub controller_joycon_black_dpleft: Sprite,
    pub controller_joycon_black_dpright: Sprite,
    pub controller_joycon_black_dpup: Sprite,
    pub controller_joycon_black_left_stick: Sprite,
    pub controller_joycon_black_leftshoulder: Sprite,
    pub controller_joycon_black_leftstick: Sprite,
    pub controller_joycon_black_lefttrigger: Sprite,
    pub controller_joycon_black_paddle1: Sprite,
    pub controller_joycon_black_paddle2: Sprite,
    pub controller_joycon_black_paddle3: Sprite,
    pub controller_joycon_black_paddle4: Sprite,
    pub controller_joycon_black_right_stick: Sprite,
    pub controller_joycon_black_rightshoulder: Sprite,
    pub controller_joycon_black_rightstick: Sprite,
    pub controller_joycon_black_righttrigger: Sprite,
    pub controller_joycon_black_start: Sprite,
    pub controller_joycon_black_x: Sprite,
    pub controller_joycon_black_y: Sprite,
    pub controller_joycon_dpdown: Sprite,
    pub controller_joycon_dpleft: Sprite,
    pub controller_joycon_dpright: Sprite,
    pub controller_joycon_dpup: Sprite,
    pub controller_joycon_left_stick: Sprite,
    pub controller_joycon_leftshoulder: Sprite,
    pub controller_joycon_leftstick: Sprite,
    pub controller_joycon_lefttrigger: Sprite,
    pub controller_joycon_paddle1: Sprite,
    pub controller_joycon_paddle2: Sprite,
    pub controller_joycon_paddle3: Sprite,
    pub controller_joycon_paddle4: Sprite,
    pub controller_joycon_right_stick: Sprite,
    pub controller_joycon_rightshoulder: Sprite,
    pub controller_joycon_rightstick: Sprite,
    pub controller_joycon_righttrigger: Sprite,
    pub controller_joycon_start: Sprite,
    pub controller_joycon_x: Sprite,
    pub controller_joycon_y: Sprite,
    pub controller_ps_a: Sprite,
    pub controller_ps_b: Sprite,
    pub controller_ps_back: Sprite,
    pub controller_ps_black_a: Sprite,
    pub controller_ps_black_b: Sprite,
    pub controller_ps_black_back: Sprite,
    pub controller_ps_black_dpdown: Sprite,
    pub controller_ps_black_dpleft: Sprite,
    pub controller_ps_black_dpright: Sprite,
    pub controller_ps_black_dpup: Sprite,
    pub controller_ps_black_left_stick: Sprite,
    pub controller_ps_black_leftshoulder: Sprite,
    pub controller_ps_black_leftstick: Sprite,
    pub controller_ps_black_lefttrigger: Sprite,
    pub controller_ps_black_right_stick: Sprite,
    pub controller_ps_black_rightshoulder: Sprite,
    pub controller_ps_black_rightstick: Sprite,
    pub controller_ps_black_righttrigger: Sprite,
    pub controller_ps_black_start: Sprite,
    pub controller_ps_black_x: Sprite,
    pub controller_ps_black_y: Sprite,
    pub controller_ps_dpdown: Sprite,
    pub controller_ps_dpleft: Sprite,
    pub controller_ps_dpright: Sprite,
    pub controller_ps_dpup: Sprite,
    pub controller_ps_left_stick: Sprite,
    pub controller_ps_leftshoulder: Sprite,
    pub controller_ps_leftstick: Sprite,
    pub controller_ps_lefttrigger: Sprite,
    pub controller_ps_right_stick: Sprite,
    pub controller_ps_rightshoulder: Sprite,
    pub controller_ps_rightstick: Sprite,
    pub controller_ps_righttrigger: Sprite,
    pub controller_ps_start: Sprite,
    pub controller_ps_x: Sprite,
    pub controller_ps_y: Sprite,
    pub controller_steamdeck_a: Sprite,
    pub controller_steamdeck_b: Sprite,
    pub controller_steamdeck_back: Sprite,
    pub controller_steamdeck_black_a: Sprite,
    pub controller_steamdeck_black_b: Sprite,
    pub controller_steamdeck_black_back: Sprite,
    pub controller_steamdeck_black_dpdown: Sprite,
    pub controller_steamdeck_black_dpleft: Sprite,
    pub controller_steamdeck_black_dpright: Sprite,
    pub controller_steamdeck_black_dpup: Sprite,
    pub controller_steamdeck_black_left_stick: Sprite,
    pub controller_steamdeck_black_leftshoulder: Sprite,
    pub controller_steamdeck_black_leftstick: Sprite,
    pub controller_steamdeck_black_lefttrigger: Sprite,
    pub controller_steamdeck_black_paddle1: Sprite,
    pub controller_steamdeck_black_paddle2: Sprite,
    pub controller_steamdeck_black_paddle3: Sprite,
    pub controller_steamdeck_black_paddle4: Sprite,
    pub controller_steamdeck_black_right_stick: Sprite,
    pub controller_steamdeck_black_rightshoulder: Sprite,
    pub controller_steamdeck_black_rightstick: Sprite,
    pub controller_steamdeck_black_righttrigger: Sprite,
    pub controller_steamdeck_black_start: Sprite,
    pub controller_steamdeck_black_x: Sprite,
    pub controller_steamdeck_black_y: Sprite,
    pub controller_steamdeck_dpdown: Sprite,
    pub controller_steamdeck_dpleft: Sprite,
    pub controller_steamdeck_dpright: Sprite,
    pub controller_steamdeck_dpup: Sprite,
    pub controller_steamdeck_left_stick: Sprite,
    pub controller_steamdeck_leftshoulder: Sprite,
    pub controller_steamdeck_leftstick: Sprite,
    pub controller_steamdeck_lefttrigger: Sprite,
    pub controller_steamdeck_paddle1: Sprite,
    pub controller_steamdeck_paddle2: Sprite,
    pub controller_steamdeck_paddle3: Sprite,
    pub controller_steamdeck_paddle4: Sprite,
    pub controller_steamdeck_right_stick: Sprite,
    pub controller_steamdeck_rightshoulder: Sprite,
    pub controller_steamdeck_rightstick: Sprite,
    pub controller_steamdeck_righttrigger: Sprite,
    pub controller_steamdeck_start: Sprite,
    pub controller_steamdeck_x: Sprite,
    pub controller_steamdeck_y: Sprite,
    pub controller_xbox_a: Sprite,
    pub controller_xbox_b: Sprite,
    pub controller_xbox_back: Sprite,
    pub controller_xbox_black_a: Sprite,
    pub controller_xbox_black_b: Sprite,
    pub controller_xbox_black_back: Sprite,
    pub controller_xbox_black_dpdown: Sprite,
    pub controller_xbox_black_dpleft: Sprite,
    pub controller_xbox_black_dpright: Sprite,
    pub controller_xbox_black_dpup: Sprite,
    pub controller_xbox_black_left_stick: Sprite,
    pub controller_xbox_black_leftshoulder: Sprite,
    pub controller_xbox_black_leftstick: Sprite,
    pub controller_xbox_black_lefttrigger: Sprite,
    pub controller_xbox_black_right_stick: Sprite,
    pub controller_xbox_black_rightshoulder: Sprite,
    pub controller_xbox_black_rightstick: Sprite,
    pub controller_xbox_black_righttrigger: Sprite,
    pub controller_xbox_black_start: Sprite,
    pub controller_xbox_black_x: Sprite,
    pub controller_xbox_black_y: Sprite,
    pub controller_xbox_dpdown: Sprite,
    pub controller_xbox_dpleft: Sprite,
    pub controller_xbox_dpright: Sprite,
    pub controller_xbox_dpup: Sprite,
    pub controller_xbox_left_stick: Sprite,
    pub controller_xbox_leftshoulder: Sprite,
    pub controller_xbox_leftstick: Sprite,
    pub controller_xbox_lefttrigger: Sprite,
    pub controller_xbox_right_stick: Sprite,
    pub controller_xbox_rightshoulder: Sprite,
    pub controller_xbox_rightstick: Sprite,
    pub controller_xbox_righttrigger: Sprite,
    pub controller_xbox_start: Sprite,
    pub controller_xbox_x: Sprite,
    pub controller_xbox_y: Sprite,
    pub copper_wire: Sprite,
    pub copy: Sprite,
    pub covered_chunk: Sprite,
    pub crafting_machine_recipe_not_unlocked: Sprite,
    pub cross_select: Sprite,
    pub cursor_box: CursorBoxSpecification,
    pub cursor_icon: Sprite,
    pub custom_tag_icon: Sprite,
    pub custom_tag_in_map_view: Sprite,
    pub danger_icon: Sprite,
    pub deconstruction_mark: Sprite,
    pub deconstruction_time_to_live_modifier_constant: Option<Sprite>,
    pub deconstruction_time_to_live_modifier_icon: Sprite,
    pub decorative_editor_icon: Sprite,
    pub default_ammo_damage_modifier_icon: Sprite,
    pub default_gun_speed_modifier_icon: Sprite,
    pub default_turret_attack_modifier_icon: Sprite,
    pub destroyed_icon: Sprite,
    pub down_arrow: Sprite,
    pub downloaded: Sprite,
    pub downloaded_white: Sprite,
    pub downloading: Sprite,
    pub downloading_white: Sprite,
    pub dropdown: Sprite,
    pub editor_pause: Sprite,
    pub editor_play: Sprite,
    pub editor_selection: Sprite,
    pub editor_speed_down: Sprite,
    pub editor_speed_up: Sprite,
    pub electricity_icon: Sprite,
    pub electricity_icon_unplugged: Sprite,
    pub enemy_force_icon: Sprite,
    pub enter: Sprite,
    pub entity_editor_icon: Sprite,
    pub entity_info_dark_background: Sprite,
    pub equipment_collision: Sprite,
    pub equipment_grid: Sprite,
    pub equipment_slot: Sprite,
    pub expand: Sprite,
    pub expand_dark: Sprite,
    pub expand_dots: Sprite,
    pub expand_dots_white: Sprite,
    pub explosion_chart_visualization: Animation,
    pub export: Sprite,
    pub export_slot: Sprite,
    pub favourite_server_icon: Sprite,
    pub fluid_icon: Sprite,
    pub fluid_indication_arrow: Sprite,
    pub fluid_indication_arrow_both_ways: Sprite,
    pub follower_robot_lifetime_modifier_constant: Option<Sprite>,
    pub follower_robot_lifetime_modifier_icon: Sprite,
    pub force_editor_icon: Sprite,
    pub fuel_icon: Sprite,
    pub game_stopped_visualization: Sprite,
    pub ghost_bar_pip: Sprite,
    pub ghost_cursor: Sprite,
    pub ghost_time_to_live_modifier_constant: Option<Sprite>,
    pub ghost_time_to_live_modifier_icon: Sprite,
    pub give_item_modifier_constant: Option<Sprite>,
    pub give_item_modifier_icon: Sprite,
    pub go_to_arrow: Sprite,
    pub gps_map_icon: Sprite,
    pub gradient: Sprite,
    pub green_circle: Sprite,
    pub green_dot: Sprite,
    pub green_wire: Sprite,
    pub green_wire_hightlight: Sprite,
    pub grey_placement_indicator_leg: Sprite,
    pub grey_rail_signal_placement_indicator: Sprite,
    pub grid_view: Sprite,
    pub gun_speed_modifier_constant: Option<Sprite>,
    pub gun_speed_modifier_icon: Sprite,
    pub hand: Sprite,
    pub hand_black: Sprite,
    pub health_bar_green_pip: Sprite,
    pub health_bar_red_pip: Sprite,
    pub health_bar_yellow_pip: Sprite,
    pub heat_exchange_indication: Sprite,
    pub hint_arrow_down: Sprite,
    pub hint_arrow_left: Sprite,
    pub hint_arrow_right: Sprite,
    pub hint_arrow_up: Sprite,
    pub import: Sprite,
    pub import_slot: Sprite,
    pub indication_arrow: Sprite,
    pub indication_line: Sprite,
    pub inserter_stack_size_bonus_modifier_constant: Option<Sprite>,
    pub inserter_stack_size_bonus_modifier_icon: Sprite,
    pub item_editor_icon: Sprite,
    pub laboratory_productivity_modifier_constant: Option<Sprite>,
    pub laboratory_productivity_modifier_icon: Sprite,
    pub laboratory_speed_modifier_constant: Option<Sprite>,
    pub laboratory_speed_modifier_icon: Sprite,
    pub left_arrow: Sprite,
    pub light_cone: Sprite,
    pub light_medium: Sprite,
    pub light_small: Sprite,
    pub line_icon: Sprite,
    pub list_view: Sprite,
    pub logistic_network_panel_black: Sprite,
    pub logistic_network_panel_white: Sprite,
    pub logistic_radius_visualization: Sprite,
    pub lua_snippet_tool_icon: Sprite,
    pub map: Sprite,
    pub map_exchange_string: Sprite,
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_constant: Option<Sprite>,
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_constant: Option<Sprite>,
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    pub maximum_following_robots_count_modifier_constant: Option<Sprite>,
    pub maximum_following_robots_count_modifier_icon: Sprite,
    pub medium_gui_arrow: Sprite,
    pub mining_drill_productivity_bonus_modifier_constant: Option<Sprite>,
    pub mining_drill_productivity_bonus_modifier_icon: Sprite,
    pub missing_icon: Sprite,
    pub missing_mod_icon: Sprite,
    pub mod_dependency_arrow: Sprite,
    pub mouse_cursor: Sprite,
    pub multiplayer_waiting_icon: Sprite,
    pub nature_icon: Sprite,
    pub neutral_force_icon: Sprite,
    pub no_building_material_icon: Sprite,
    pub no_nature_icon: Sprite,
    pub no_storage_space_icon: Sprite,
    pub none_editor_icon: Sprite,
    pub not_available: Sprite,
    pub not_enough_construction_robots_icon: Sprite,
    pub not_enough_repair_packs_icon: Sprite,
    pub not_played_yet_dark_green: Sprite,
    pub not_played_yet_green: Sprite,
    pub nothing_modifier_constant: Option<Sprite>,
    pub nothing_modifier_icon: Sprite,
    pub notification: Sprite,
    pub output_console_gradient: Sprite,
    pub paint_bucket_icon: Sprite,
    pub pause: Sprite,
    pub placement_indicator_leg: Sprite,
    pub play: Sprite,
    pub played_dark_green: Sprite,
    pub played_green: Sprite,
    pub player_force_icon: Sprite,
    pub preset: Sprite,
    pub pump_cannot_connect_icon: Sprite,
    pub questionmark: Sprite,
    pub rail_path_not_possible: Sprite,
    pub rail_planner_indication_arrow: Sprite,
    pub rail_planner_indication_arrow_too_far: Sprite,
    pub rail_signal_placement_indicator: Sprite,
    pub reassign: Sprite,
    pub recharge_icon: Sprite,
    pub red_wire: Sprite,
    pub red_wire_hightlight: Sprite,
    pub reference_point: Sprite,
    pub refresh: Sprite,
    pub refresh_white: Animation,
    pub rename_icon_normal: Sprite,
    pub rename_icon_small_black: Sprite,
    pub rename_icon_small_white: Sprite,
    pub reset: Sprite,
    pub reset_white: Sprite,
    pub resource_editor_icon: Sprite,
    pub right_arrow: Sprite,
    pub robot_slot: Sprite,
    pub scripting_editor_icon: Sprite,
    pub search_black: Sprite,
    pub search_icon: Sprite,
    pub search_white: Sprite,
    pub select_icon_black: Sprite,
    pub select_icon_white: Sprite,
    pub set_bar_slot: Sprite,
    pub shield_bar_pip: Sprite,
    pub shoot_cursor_green: Sprite,
    pub shoot_cursor_red: Sprite,
    pub short_indication_line: Sprite,
    pub short_indication_line_green: Sprite,
    pub show_electric_network_in_map_view: Sprite,
    pub show_electric_network_in_map_view_black: Sprite,
    pub show_logistics_network_in_map_view: Sprite,
    pub show_logistics_network_in_map_view_black: Sprite,
    pub show_player_names_in_map_view: Sprite,
    pub show_player_names_in_map_view_black: Sprite,
    pub show_pollution_in_map_view: Sprite,
    pub show_pollution_in_map_view_black: Sprite,
    pub show_rail_signal_states_in_map_view: Sprite,
    pub show_rail_signal_states_in_map_view_black: Sprite,
    pub show_recipe_icons_in_map_view: Sprite,
    pub show_recipe_icons_in_map_view_black: Sprite,
    pub show_tags_in_map_view: Sprite,
    pub show_tags_in_map_view_black: Sprite,
    pub show_train_station_names_in_map_view: Sprite,
    pub show_train_station_names_in_map_view_black: Sprite,
    pub show_turret_range_in_map_view: Sprite,
    pub show_turret_range_in_map_view_black: Sprite,
    pub show_worker_robots_in_map_view: Sprite,
    pub show_worker_robots_in_map_view_black: Sprite,
    pub shuffle: Sprite,
    pub side_menu_achievements_hover_icon: Sprite,
    pub side_menu_achievements_icon: Sprite,
    pub side_menu_blueprint_library_hover_icon: Sprite,
    pub side_menu_blueprint_library_icon: Sprite,
    pub side_menu_bonus_hover_icon: Sprite,
    pub side_menu_bonus_icon: Sprite,
    pub side_menu_logistic_network_hover_icon: Sprite,
    pub side_menu_map_hover_icon: Sprite,
    pub side_menu_map_icon: Sprite,
    pub side_menu_menu_hover_icon: Sprite,
    pub side_menu_menu_icon: Sprite,
    pub side_menu_production_hover_icon: Sprite,
    pub side_menu_production_icon: Sprite,
    pub side_menu_technology_hover_icon: Sprite,
    pub side_menu_train_hover_icon: Sprite,
    pub side_menu_train_icon: Sprite,
    pub side_menu_tutorials_hover_icon: Sprite,
    pub side_menu_tutorials_icon: Sprite,
    pub slot: Sprite,
    pub slot_icon_ammo: Sprite,
    pub slot_icon_ammo_black: Sprite,
    pub slot_icon_armor: Sprite,
    pub slot_icon_armor_black: Sprite,
    pub slot_icon_fuel: Sprite,
    pub slot_icon_fuel_black: Sprite,
    pub slot_icon_gun: Sprite,
    pub slot_icon_gun_black: Sprite,
    pub slot_icon_inserter_hand: Sprite,
    pub slot_icon_inserter_hand_black: Sprite,
    pub slot_icon_module: Sprite,
    pub slot_icon_module_black: Sprite,
    pub slot_icon_resource: Sprite,
    pub slot_icon_resource_black: Sprite,
    pub slot_icon_result: Sprite,
    pub slot_icon_result_black: Sprite,
    pub slot_icon_robot: Sprite,
    pub slot_icon_robot_black: Sprite,
    pub slot_icon_robot_material: Sprite,
    pub slot_icon_robot_material_black: Sprite,
    pub small_gui_arrow: Sprite,
    pub spawn_flag: Sprite,
    pub speed_down: Sprite,
    pub speed_up: Sprite,
    pub spray_icon: Sprite,
    pub stack_inserter_capacity_bonus_modifier_constant: Option<Sprite>,
    pub stack_inserter_capacity_bonus_modifier_icon: Sprite,
    pub station_name: Sprite,
    pub status_not_working: Sprite,
    pub status_working: Sprite,
    pub status_yellow: Sprite,
    pub stop: Sprite,
    pub surface_editor_icon: Sprite,
    pub sync_mods: Sprite,
    pub technology_black: Sprite,
    pub technology_white: Sprite,
    pub tick_custom: Sprite,
    pub tick_once: Sprite,
    pub tick_sixty: Sprite,
    pub tile_editor_icon: Sprite,
    pub tile_ghost_cursor: Sprite,
    pub time_editor_icon: Sprite,
    pub too_far: Sprite,
    pub too_far_from_roboport_icon: Sprite,
    pub track_button: Sprite,
    pub train_braking_force_bonus_modifier_constant: Option<Sprite>,
    pub train_braking_force_bonus_modifier_icon: Sprite,
    pub train_stop_disabled_in_map_view: Sprite,
    pub train_stop_full_in_map_view: Sprite,
    pub train_stop_in_map_view: Sprite,
    pub train_stop_placement_indicator: Sprite,
    pub trash: Sprite,
    pub trash_white: Sprite,
    pub turret_attack_modifier_constant: Option<Sprite>,
    pub turret_attack_modifier_icon: Sprite,
    pub underground_pipe_connection: Sprite,
    pub underground_remove_belts: Sprite,
    pub underground_remove_pipes: Sprite,
    pub unlock_recipe_modifier_constant: Option<Sprite>,
    pub unlock_recipe_modifier_icon: Sprite,
    pub upgrade_blueprint: Sprite,
    pub upgrade_mark: Sprite,
    pub variations_tool_icon: Sprite,
    pub warning: Sprite,
    pub warning_icon: Sprite,
    pub warning_white: Sprite,
    pub white_mask: Sprite,
    pub white_square: Sprite,
    pub wire_shadow: Sprite,
    pub worker_robot_battery_modifier_constant: Option<Sprite>,
    pub worker_robot_battery_modifier_icon: Sprite,
    pub worker_robot_speed_modifier_constant: Option<Sprite>,
    pub worker_robot_speed_modifier_icon: Sprite,
    pub worker_robot_storage_modifier_constant: Option<Sprite>,
    pub worker_robot_storage_modifier_icon: Sprite,
    pub zoom_to_world_blueprint_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_blueprint_enabled_modifier_icon: Sprite,
    pub zoom_to_world_deconstruction_planner_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_deconstruction_planner_enabled_modifier_icon: Sprite,
    pub zoom_to_world_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_enabled_modifier_icon: Sprite,
    pub zoom_to_world_ghost_building_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_ghost_building_enabled_modifier_icon: Sprite,
    pub zoom_to_world_selection_tool_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_selection_tool_enabled_modifier_icon: Sprite,
    pub zoom_to_world_upgrade_planner_enabled_modifier_constant: Option<Sprite>,
    pub zoom_to_world_upgrade_planner_enabled_modifier_icon: Sprite,
    */
}

pub type UtilitySprites = BasePrototype<UtilitySpritesData>;

pub type PrototypeMap<T> = HashMap<String, T>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataRaw {
    pub accumulator: PrototypeMap<AccumulatorPrototype>,
    pub artillery_turret: PrototypeMap<ArtilleryTurretPrototype>,
    pub beacon: PrototypeMap<BeaconPrototype>,
    pub boiler: PrototypeMap<BoilerPrototype>,
    pub burner_generator: PrototypeMap<BurnerGeneratorPrototype>,

    pub arithmetic_combinator: PrototypeMap<ArithmeticCombinatorPrototype>,
    pub decider_combinator: PrototypeMap<DeciderCombinatorPrototype>,
    pub constant_combinator: PrototypeMap<ConstantCombinatorPrototype>,
    pub programmable_speaker: PrototypeMap<ProgrammableSpeakerPrototype>,

    pub container: PrototypeMap<ContainerPrototype>,
    pub logistic_container: PrototypeMap<LogisticContainerPrototype>,
    pub infinity_container: PrototypeMap<InfinityContainerPrototype>,
    pub linked_container: PrototypeMap<LinkedContainerPrototype>,

    pub assembling_machine: PrototypeMap<AssemblingMachinePrototype>,
    pub rocket_silo: PrototypeMap<RocketSiloPrototype>,
    pub furnace: PrototypeMap<FurnacePrototype>,

    pub electric_energy_interface: PrototypeMap<ElectricEnergyInterfacePrototype>,
    pub electric_pole: PrototypeMap<ElectricPolePrototype>,
    pub power_switch: PrototypeMap<PowerSwitchPrototype>,

    pub combat_robot: PrototypeMap<CombatRobotPrototype>,
    pub construction_robot: PrototypeMap<ConstructionRobotPrototype>,
    pub logistic_robot: PrototypeMap<LogisticRobotPrototype>,
    pub roboport: PrototypeMap<RoboportPrototype>,

    pub gate: PrototypeMap<GatePrototype>,
    pub wall: PrototypeMap<WallPrototype>,

    pub generator: PrototypeMap<GeneratorPrototype>,

    pub reactor: PrototypeMap<ReactorPrototype>,
    pub heat_interface: PrototypeMap<HeatInterfacePrototype>,
    pub heat_pipe: PrototypeMap<HeatPipePrototype>,

    pub inserter: PrototypeMap<InserterPrototype>,

    pub lab: PrototypeMap<LabPrototype>,

    pub lamp: PrototypeMap<LampPrototype>,

    pub land_mine: PrototypeMap<LandMinePrototype>,

    pub mining_drill: PrototypeMap<MiningDrillPrototype>,
    pub offshore_pump: PrototypeMap<OffshorePumpPrototype>,

    pub pipe: PrototypeMap<PipePrototype>,
    pub infinity_pipe: PrototypeMap<InfinityPipePrototype>,
    pub pipe_to_ground: PrototypeMap<PipeToGroundPrototype>,
    pub pump: PrototypeMap<PumpPrototype>,

    pub simple_entity_with_owner: PrototypeMap<SimpleEntityWithOwnerPrototype>,
    pub simple_entity_with_force: PrototypeMap<SimpleEntityWithForcePrototype>,

    pub solar_panel: PrototypeMap<SolarPanelPrototype>,

    pub storage_tank: PrototypeMap<StorageTankPrototype>,

    pub linked_belt: PrototypeMap<LinkedBeltPrototype>,
    pub loader_1x1: PrototypeMap<Loader1x1Prototype>,
    pub loader: PrototypeMap<Loader1x2Prototype>,
    pub splitter: PrototypeMap<SplitterPrototype>,
    pub transport_belt: PrototypeMap<TransportBeltPrototype>,
    pub underground_belt: PrototypeMap<UndergroundBeltPrototype>,

    pub radar: PrototypeMap<RadarPrototype>,
    pub turret: PrototypeMap<TurretPrototype>,
    pub ammo_turret: PrototypeMap<AmmoTurretPrototype>,
    pub electric_turret: PrototypeMap<ElectricTurretPrototype>,
    pub fluid_turret: PrototypeMap<FluidTurretPrototype>,

    pub car: PrototypeMap<CarPrototype>,

    pub curved_rail: PrototypeMap<CurvedRailPrototype>,
    pub straight_rail: PrototypeMap<StraightRailPrototype>,
    pub rail_signal: PrototypeMap<RailSignalPrototype>,
    pub rail_chain_signal: PrototypeMap<RailChainSignalPrototype>,
    pub train_stop: PrototypeMap<TrainStopPrototype>,
    pub locomotive: PrototypeMap<LocomotivePrototype>,
    pub cargo_wagon: PrototypeMap<CargoWagonPrototype>,
    pub fluid_wagon: PrototypeMap<FluidWagonPrototype>,
    pub artillery_wagon: PrototypeMap<ArtilleryWagonPrototype>,

    pub utility_sprites: PrototypeMap<UtilitySprites>,
    // not implemented
    // pub character: PrototypeMap<CharacterPrototype>,
    // pub unit_spawner: PrototypeMap<EnemySpawnerPrototype>,
    // pub player_port: PrototypeMap<PlayerPortPrototype>,
    // pub unit: PrototypeMap<UnitPrototype>,
    // pub spider_vehicle: PrototypeMap<SpiderVehiclePrototype>,
}

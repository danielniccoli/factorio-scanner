use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::blueprint::ItemStackIndex;

use super::{helper, Direction, FluidBox, FuelCategory, MapPosition, Sprite4Way};

/// [`Types/Energy`](https://lua-api.factorio.com/latest/types/Energy.html)
pub type Energy = String;

/// [`Types/BaseEnergySource`](https://lua-api.factorio.com/latest/types/BaseEnergySource.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseEnergySource<T> {
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub emissions_per_minute: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_no_power_icon: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_no_network_icon: bool,

    #[serde(flatten)]
    pub child: T,
}

/// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BurnerEnergySourceData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub fuel_inventory_size: ItemStackIndex,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub burnt_inventory_size: ItemStackIndex,

    // #[serde(default, skip_serializing_if = "Vec::is_empty"))]
    // smoke: Vec<SmokeSource>,
    // light_flicker: Option<LightFlickeringDefinition>,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,

    #[serde(flatten)]
    pub fuel: Option<FuelCategory>,
}

/// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
pub type BurnerEnergySource = BaseEnergySource<BurnerEnergySourceData>;

/// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricEnergySourceData {
    pub buffer_capacity: Option<Energy>,
    pub usage_priority: ElectricUsagePriority,
    pub input_flow_limit: Option<Energy>,
    pub output_flow_limit: Option<Energy>,
    pub drain: Option<Energy>,
}

/// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
pub type ElectricEnergySource = BaseEnergySource<ElectricEnergySourceData>;

/// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
//#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FluidEnergySourceData {
    pub fluid_box: FluidBox,

    // #[serde(default, skip_serializing_if = "Vec::is_empty"))]
    // pub smoke: Vec<SmokeSource>,
    // pub light_flicker: Option<LightFlickeringDefinition>,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub burns_fluid: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_fluid_usage: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub destroy_non_fuel_fluid: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub fluid_usage_per_tick: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub maximum_temperature: f64,
}

/// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
pub type FluidEnergySource = BaseEnergySource<FluidEnergySourceData>;

/// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatEnergySourceData {
    pub max_temperature: f64,
    pub specific_heat: Energy,
    pub max_transfer: Energy,

    #[serde(default = "helper::f64_15", skip_serializing_if = "helper::is_15_f64")]
    pub default_temperature: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub min_temperature_gradient: f64,

    #[serde(default = "helper::f64_15", skip_serializing_if = "helper::is_15_f64")]
    pub min_working_temperature: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub minimum_glow_temperature: f64,

    pub pipe_covers: Option<Sprite4Way>,
    pub heat_picture: Option<Sprite4Way>,
    pub heat_glow: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<HeatConnection>,
}

/// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
pub type HeatEnergySource = BaseEnergySource<HeatEnergySourceData>;

/// Enumeration of all `EnergySource` types.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AnyEnergySource {
    /// [`Types/BurnerEnergySource`](https://lua-api.factorio.com/latest/types/BurnerEnergySource.html)
    Burner {
        #[serde(flatten)]
        data: BurnerEnergySource,
    },

    /// [`Types/ElectricEnergySource`](https://lua-api.factorio.com/latest/types/ElectricEnergySource.html)
    Electric {
        #[serde(flatten)]
        data: ElectricEnergySource,
    },

    /// [`Types/FluidEnergySource`](https://lua-api.factorio.com/latest/types/FluidEnergySource.html)
    Fluid {
        #[serde(flatten)]
        data: FluidEnergySource,
    },

    /// [`Types/HeatEnergySource`](https://lua-api.factorio.com/latest/types/HeatEnergySource.html)
    Heat {
        #[serde(flatten)]
        data: HeatEnergySource,
    },

    /// [`Types/VoidEnergySource`](https://lua-api.factorio.com/latest/types/VoidEnergySource.html)
    Void,
}

/// [`Types/ElectricUsagePriority`](https://lua-api.factorio.com/latest/types/ElectricUsagePriority.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ElectricUsagePriority {
    PrimaryInput,
    PrimaryOutput,
    SecondaryInput,
    SecondaryOutput,
    Tertiary,
    Solar,
    Lamp,
}

/// [`Types/HeatConnection`](https://lua-api.factorio.com/latest/types/HeatConnection.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatConnection {
    pub position: MapPosition,
    pub direction: Direction,
}

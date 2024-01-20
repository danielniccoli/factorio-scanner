use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use types::{Animation, BoxSpecification, FactorioArray, Sprite};

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
pub type UtilitySprites = crate::BasePrototype<UtilitySpritesData>;

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct UtilitySpritesData {
    pub cursor_box: CursorBoxSpecification,
    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,

    #[serde(flatten)]
    pub sprites: HashMap<String, Sprite>,
}

/// [`Prototypes/UtilitySprites/CursorBoxSpecification`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorBoxSpecification {
    pub regular: FactorioArray<BoxSpecification>,
    pub not_allowed: FactorioArray<BoxSpecification>,
    pub copy: FactorioArray<BoxSpecification>,
    pub electricity: FactorioArray<BoxSpecification>,
    pub logistics: FactorioArray<BoxSpecification>,
    pub pair: FactorioArray<BoxSpecification>,
    pub train_visualization: FactorioArray<BoxSpecification>,
    pub blueprint_snap_rectangle: FactorioArray<BoxSpecification>,
}

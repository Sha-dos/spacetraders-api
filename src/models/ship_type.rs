/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ShipType : Type of ship
/// Type of ship
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipType {
    #[serde(rename = "SHIP_PROBE")]
    ShipProbe,
    #[serde(rename = "SHIP_MINING_DRONE")]
    ShipMiningDrone,
    #[serde(rename = "SHIP_SIPHON_DRONE")]
    ShipSiphonDrone,
    #[serde(rename = "SHIP_INTERCEPTOR")]
    ShipInterceptor,
    #[serde(rename = "SHIP_LIGHT_HAULER")]
    ShipLightHauler,
    #[serde(rename = "SHIP_COMMAND_FRIGATE")]
    ShipCommandFrigate,
    #[serde(rename = "SHIP_EXPLORER")]
    ShipExplorer,
    #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
    ShipHeavyFreighter,
    #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
    ShipLightShuttle,
    #[serde(rename = "SHIP_ORE_HOUND")]
    ShipOreHound,
    #[serde(rename = "SHIP_REFINING_FREIGHTER")]
    ShipRefiningFreighter,
    #[serde(rename = "SHIP_SURVEYOR")]
    ShipSurveyor,
}

impl std::fmt::Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ShipProbe => write!(f, "SHIP_PROBE"),
            Self::ShipMiningDrone => write!(f, "SHIP_MINING_DRONE"),
            Self::ShipSiphonDrone => write!(f, "SHIP_SIPHON_DRONE"),
            Self::ShipInterceptor => write!(f, "SHIP_INTERCEPTOR"),
            Self::ShipLightHauler => write!(f, "SHIP_LIGHT_HAULER"),
            Self::ShipCommandFrigate => write!(f, "SHIP_COMMAND_FRIGATE"),
            Self::ShipExplorer => write!(f, "SHIP_EXPLORER"),
            Self::ShipHeavyFreighter => write!(f, "SHIP_HEAVY_FREIGHTER"),
            Self::ShipLightShuttle => write!(f, "SHIP_LIGHT_SHUTTLE"),
            Self::ShipOreHound => write!(f, "SHIP_ORE_HOUND"),
            Self::ShipRefiningFreighter => write!(f, "SHIP_REFINING_FREIGHTER"),
            Self::ShipSurveyor => write!(f, "SHIP_SURVEYOR"),
        }
    }
}

impl Default for ShipType {
    fn default() -> ShipType {
        Self::ShipProbe
    }
}

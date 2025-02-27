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

/// ShipMount : A mount is installed on the exterier of a ship.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipMount {
    /// Symbo of this mount.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Name of this mount.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of this mount.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mounts that have this value, such as mining lasers, denote how powerful this mount's capabilities are.
    #[serde(rename = "strength", skip_serializing_if = "Option::is_none")]
    pub strength: Option<u32>,
    /// Mounts that have this value denote what goods can be produced from using the mount.
    #[serde(rename = "deposits", skip_serializing_if = "Option::is_none")]
    pub deposits: Option<Vec<Deposits>>,
    #[serde(rename = "requirements")]
    pub requirements: Box<models::ShipRequirements>,
}

impl ShipMount {
    /// A mount is installed on the exterier of a ship.
    pub fn new(symbol: Symbol, name: String, requirements: models::ShipRequirements) -> ShipMount {
        ShipMount {
            symbol,
            name,
            description: None,
            strength: None,
            deposits: None,
            requirements: Box::new(requirements),
        }
    }
}
/// Symbo of this mount.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Symbol {
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIii,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorIi,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIii,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayIi,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIii,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserIi,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIii,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    MountLaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MountMissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    MountTurretI,
}

impl Default for Symbol {
    fn default() -> Symbol {
        Self::MountGasSiphonI
    }
}
/// Mounts that have this value denote what goods can be produced from using the mount.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Deposits {
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
}

impl Default for Deposits {
    fn default() -> Deposits {
        Self::QuartzSand
    }
}

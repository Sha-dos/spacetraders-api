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

/// ShipCargoItem : The type of cargo item and the number of units.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipCargoItem {
    #[serde(rename = "symbol")]
    pub symbol: models::TradeSymbol,
    /// The name of the cargo item type.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the cargo item type.
    #[serde(rename = "description")]
    pub description: String,
    /// The number of units of the cargo item.
    #[serde(rename = "units")]
    pub units: i32,
}

impl ShipCargoItem {
    /// The type of cargo item and the number of units.
    pub fn new(
        symbol: models::TradeSymbol,
        name: String,
        description: String,
        units: i32,
    ) -> ShipCargoItem {
        ShipCargoItem {
            symbol,
            name,
            description,
            units,
        }
    }
}

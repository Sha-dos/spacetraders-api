/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`accept_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptContractError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deliver_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeliverContractError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fulfill_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FulfillContractError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContractError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_contracts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContractsError {
    UnknownValue(serde_json::Value),
}

/// Accept a contract by ID.   You can only accept contracts that were offered to you, were not accepted yet, and whose deadlines has not passed yet.
pub async fn accept_contract(
    configuration: &configuration::Configuration,
    contract_id: &str,
) -> Result<models::AcceptContract200Response, Error<AcceptContractError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_contract_id = contract_id;

    let uri_str = format!(
        "{}/my/contracts/{contractId}/accept",
        configuration.base_path,
        contractId = crate::apis::urlencode(p_contract_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<AcceptContractError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Deliver cargo to a contract.  In order to use this API, a ship must be at the delivery location (denoted in the delivery terms as `destinationSymbol` of a contract) and must have a number of units of a good required by this contract in its cargo.  Cargo that was delivered will be removed from the ship's cargo.
pub async fn deliver_contract(
    configuration: &configuration::Configuration,
    contract_id: &str,
    deliver_contract_request: Option<models::DeliverContractRequest>,
) -> Result<models::DeliverContract200Response, Error<DeliverContractError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_contract_id = contract_id;
    let p_deliver_contract_request = deliver_contract_request;

    let uri_str = format!(
        "{}/my/contracts/{contractId}/deliver",
        configuration.base_path,
        contractId = crate::apis::urlencode(p_contract_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_deliver_contract_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<DeliverContractError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Fulfill a contract. Can only be used on contracts that have all of their delivery terms fulfilled.
pub async fn fulfill_contract(
    configuration: &configuration::Configuration,
    contract_id: &str,
) -> Result<models::FulfillContract200Response, Error<FulfillContractError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_contract_id = contract_id;

    let uri_str = format!(
        "{}/my/contracts/{contractId}/fulfill",
        configuration.base_path,
        contractId = crate::apis::urlencode(p_contract_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FulfillContractError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get the details of a contract by ID.
pub async fn get_contract(
    configuration: &configuration::Configuration,
    contract_id: &str,
) -> Result<models::GetContract200Response, Error<GetContractError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_contract_id = contract_id;

    let uri_str = format!(
        "{}/my/contracts/{contractId}",
        configuration.base_path,
        contractId = crate::apis::urlencode(p_contract_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetContractError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Return a paginated list of all your contracts.
pub async fn get_contracts(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<models::GetContracts200Response, Error<GetContractsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_limit = limit;

    let uri_str = format!("{}/my/contracts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetContractsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

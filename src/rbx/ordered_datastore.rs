//! Low-level OrderedDataStore API operations.
//!
//! Typically, these operations should be consumed through the `RbxExperience`
//! struct, obtained through the `RbxCloud` struct.
//!

use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;

use crate::rbx::{
    ds_error::DataStoreErrorResponse, error::Error, util::QueryString, PageSize, UniverseId,
};

pub struct OrderedListEntriesParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub max_page_size: Option<PageSize>,
    pub page_token: Option<String>,
    pub order_by: Option<String>,
    pub filter: Option<String>,
}

pub struct OrderedCreateEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
}

pub struct OrderedUpdateEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
    pub allow_missing: Option<bool>,
}

pub struct OrderedIncrementEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub increment: i64,
}

#[derive(Deserialize, Debug)]
pub struct OrderedEntry {
    pub path: String,
    pub id: String,
    pub value: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedListEntriesResponse {
    pub entries: Vec<OrderedEntry>,
    pub next_page_token: Option<String>,
}

pub struct OrderedEntryParams {
    pub api_key: String,
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
}

async fn handle_res<T: DeserializeOwned>(res: Response) -> Result<T, Error> {
    match res.status().is_success() {
        true => {
            let body = res.json::<T>().await?;
            Ok(body)
        }
        false => {
            let err_res = res.json::<DataStoreErrorResponse>().await?;
            Err(Error::DataStoreError(err_res))
        }
    }
}

async fn handle_res_ok(res: Response) -> Result<(), Error> {
    match res.status().is_success() {
        true => Ok(()),
        false => {
            let err_res = res.json::<DataStoreErrorResponse>().await?;
            Err(Error::DataStoreError(err_res))
        }
    }
}

fn build_url(endpoint: &str, universe_id: UniverseId, scope: Option<&str>) -> String {
    let s = scope.unwrap_or("global");
    if endpoint.is_empty() {
        format!("https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/scopes/{s}")
    } else {
        format!(
			"https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/scopes/{s}{endpoint}",
		)
    }
}

/// List entries of an OrderedDataStore.
pub async fn list_entries(
    params: &OrderedListEntriesParams,
) -> Result<OrderedListEntriesResponse, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/entries", params.universe_id, params.scope.as_deref());
    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("max_page_size", max_page_size.to_string()));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("page_token", page_token.to_string()));
    }
    if let Some(order_by) = &params.order_by {
        query.push(("order_by", order_by.to_string()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.to_string()));
    }
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .send()
        .await?;
    handle_res::<OrderedListEntriesResponse>(res).await
}

/// Add a new entry to an OrderedDataStore.
pub async fn create_entry(params: &OrderedCreateEntryParams) -> Result<OrderedEntry, Error> {
    let client = reqwest::Client::new();
    let url = build_url("/entries", params.universe_id, params.scope.as_deref());
    let query: QueryString = vec![("id", params.id.to_string())];
    let body_json = json!({
        "value": &params.value,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = client
        .post(url)
        .header("x-api-key", &params.api_key)
        .query(&query)
        .body(body)
        .send()
        .await?;
    handle_res::<OrderedEntry>(res).await
}

pub async fn get_entry(params: &OrderedEntryParams) -> Result<OrderedEntry, Error> {
    let client = reqwest::Client::new();
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        params.scope.as_deref(),
    );
    let res = client
        .get(url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;
    handle_res::<OrderedEntry>(res).await
}

pub async fn delete_entry(params: &OrderedEntryParams) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        params.scope.as_deref(),
    );
    let res = client
        .delete(url)
        .header("x-api-key", &params.api_key)
        .send()
        .await?;
    handle_res_ok(res).await
}

pub async fn update_entry(params: &OrderedUpdateEntryParams) -> Result<OrderedEntry, Error> {
    let client = reqwest::Client::new();
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        params.scope.as_deref(),
    );
    let mut query: QueryString = vec![];
    if let Some(allow_missing) = &params.allow_missing {
        query.push(("allow_missing", allow_missing.to_string()));
    }
    let body_json = json!({
        "value": &params.value,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .body(body)
        .query(&query)
        .send()
        .await?;
    handle_res::<OrderedEntry>(res).await
}

pub async fn increment_entry(params: &OrderedIncrementEntryParams) -> Result<OrderedEntry, Error> {
    let client = reqwest::Client::new();
    let url = build_url(
        format!("/entries/{entry}:increment", entry = params.id).as_str(),
        params.universe_id,
        params.scope.as_deref(),
    );
    let body_json = json!({
        "amount": &params.increment,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = client
        .patch(url)
        .header("x-api-key", &params.api_key)
        .body(body)
        .send()
        .await?;
    handle_res::<OrderedEntry>(res).await
}

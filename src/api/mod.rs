//! Raw bindings for the discord API
#![allow(dead_code)]

/*
TODO:
Webhooks
Audit log
Voice
*/

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod permissions;
pub mod user;

#[cfg(test)]
mod tests;

use crate::errors;
use std::collections::HashMap;

fn set_headers<C: surf::middleware::HttpClient>(
    request: surf::Request<C>,
    token: &str,
) -> surf::Request<C> {
    request
        .set_header("Authorization", format!("Bot {}", token))
        .set_header("User-Agent", crate::discord::USER_AGENT.to_owned())
}

/// Makes an http GET request with a token and a url starting after `/api/v6/`
async fn api_get<T: serde::de::DeserializeOwned>(
    token: &str,
    url: &str,
    query: Option<HashMap<&str, String>>,
) -> crate::Result<T> {
    let response = set_headers(
        surf::get(format!(
            "{}{}{}",
            crate::discord::API_PATH,
            url,
            format_query(query)
        )),
        token,
    )
    .await?;
    handle_errors(response).await
}

/// Makes an http POST request with a token and a url starting after `/api/v6/`
async fn api_post<T: serde::de::DeserializeOwned, D: serde::Serialize>(
    token: &str,
    url: &str,
    query: Option<HashMap<&str, String>>,
    data: &D,
) -> crate::Result<T> {
    let response = set_headers(
        surf::post(format!(
            "{}{}{}",
            crate::discord::API_PATH,
            url,
            format_query(query)
        )),
        token,
    )
    .body_json(data)?
    .await?;
    handle_errors(response).await
}
/// Makes an http PUT request with a token and a url starting after `/api/v6/`
async fn api_put<T: serde::de::DeserializeOwned, D: serde::Serialize>(
    token: &str,
    url: &str,
    query: Option<HashMap<&str, String>>,
    data: &D,
) -> crate::Result<T> {
    let response = set_headers(
        surf::put(format!(
            "{}{}{}",
            crate::discord::API_PATH,
            url,
            format_query(query)
        )),
        token,
    )
    .body_json(data)?
    .await?;
    handle_errors(response).await
}
/// Makes an http PATCH request with a token and a url starting after `/api/v6/`
async fn api_patch<T: serde::de::DeserializeOwned, D: serde::Serialize>(
    token: &str,
    url: &str,
    query: Option<HashMap<&str, String>>,
    data: &D,
) -> crate::Result<T> {
    let response = set_headers(
        surf::patch(format!(
            "{}{}{}",
            crate::discord::API_PATH,
            url,
            format_query(query)
        )),
        token,
    )
    .body_json(data)?
    .await?;
    handle_errors(response).await
}
/// Makes an http DELETE request with a token and a url starting after `/api/v6/`
async fn api_delete<T: serde::de::DeserializeOwned>(
    token: &str,
    url: &str,
    query: Option<HashMap<&str, String>>,
) -> crate::Result<T> {
    let response = set_headers(
        surf::delete(format!(
            "{}{}{}",
            crate::discord::API_PATH,
            url,
            format_query(query)
        )),
        token,
    )
    .await?;
    handle_errors(response).await
}

fn format_query(query: Option<HashMap<&str, String>>) -> String {
    if let Some(query) = query {
        if query.is_empty() {
            return String::new();
        }
        let mut string = String::from("?");
        let mut iter = query.iter();
        if let Some((k, v)) = iter.next() {
            string.push_str(k);
            string.push('=');
            string.push_str(v);
        }
        for (k, v) in &query {
            string.push('&');
            string.push_str(k);
            string.push('=');
            string.push_str(v);
        }
        string
    } else {
        String::new()
    }
}

async fn handle_errors<T: serde::de::DeserializeOwned>(
    mut response: surf::Response,
) -> crate::Result<T> {
    if response.status().is_success() {
        Ok(response.body_json().await?)
    } else {
        Err(errors::DiscordError::ApiError(response.body_json::<ApiError>().await?))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    pub code: u64,
    pub message: String,
}

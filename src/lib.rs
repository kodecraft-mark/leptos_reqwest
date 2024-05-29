//! This module provides utilities for making HTTP requests with `reqwest` and handling responses
//! with custom error handling through the `LeptosReqwestError` trait. It includes support for
//! different HTTP methods and deserialization of responses into custom types.

use leptos::{Serializable, SerializationError};
use reqwest::{Error, StatusCode};

/// Enum representing different HTTP methods.
pub enum HttpMethod {
    /// HTTP GET method.
    Get,
    /// HTTP POST method.
    Post,
    /// HTTP PUT method.
    Put,
    /// HTTP DELETE method.
    Delete,
}

/// Trait for custom error handling in `reqwest` requests, requiring `Default` and `Serializable` implementations.
pub trait LeptosReqwestError: Default + Serializable {
    /// Creates an error instance from a deserialization error.
    fn deserialization_error(e: SerializationError) -> Self;
    /// Creates an error instance from a read error.
    fn read_error(e: Error) -> Self;
}

/// Sends an HTTP request and deserializes the response into a specified type.
///
/// # Parameters
///
/// - `request`: An optional request body to be serialized.
/// - `url`: The URL to send the request to.
/// - `headers`: The headers to include in the request.
/// - `method`: The HTTP method to use for the request.
///
/// # Returns
///
/// A `Result` containing either the deserialized response of type `U` or an error of type `E`.
///
/// # Type Parameters
///
/// - `T`: The type of the request body, which must implement `serde::Serialize`.
/// - `U`: The type of the successful response, which must implement `Serializable`.
/// - `E`: The type of the error response, which must implement `LeptosReqwestError`.

pub async fn send_and_parse<T, U, E>(
    request: Option<T>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<U, E>
where
    T: serde::Serialize,
    U: Serializable,
    E: LeptosReqwestError
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::Get => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        },
        HttpMethod::Post => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        },
        HttpMethod::Put => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        },
        HttpMethod::Delete => {
            client
                .delete(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    // log::info!("Response: {:?}", response);
    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                match res.text().await {
                    Ok(text) => {
                        match U::de(&text) {
                            Ok(result) => Ok(result),
                            Err(e) => {
                                // Fallback if U deserialization fails
                                Err(E::deserialization_error(e))
                            }
                        }
                    }
                    Err(e) => Err(E::read_error(e)), // Error getting text from response
                }
            } else {
                match res.text().await {
                    Ok(text) => {
                        log::error!("Error response: {}", text);
                        match E::de(&text) {
                            Ok(error_result) => Err(error_result),
                            Err(e) => Err(E::deserialization_error(e)), // Fallback if E deserialization fails
                        }
                    }
                    Err(e) => Err(E::read_error(e)), // Error getting text from response
                }
            }
        },
        Err(_) => Err(E::default()), // Network or other request error
    }
}

/// Sends an HTTP request and returns a boolean indicating success.
///
/// # Parameters
///
/// - `request`: An optional request body to be serialized.
/// - `url`: The URL to send the request to.
/// - `headers`: The headers to include in the request.
/// - `method`: The HTTP method to use for the request.
///
/// # Returns
///
/// A `Result` containing `true` if the request was successful, or an error of type `E` otherwise.
///
/// # Type Parameters
///
/// - `T`: The type of the request body, which must implement `serde::Serialize`.
/// - `E`: The type of the error response, which must implement `LeptosReqwestError`.

pub async fn send<T, E>(
    request: Option<T>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<bool, E>
where
    T: serde::Serialize,
    E: LeptosReqwestError
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::Get => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        },
        HttpMethod::Post => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        },
        HttpMethod::Put => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        },
        HttpMethod::Delete => {
            client
                .delete(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    match response {
        Ok(res) => {
            if res.status().is_success() {
                Ok(true)
            } else {
                match res.text().await {
                    Ok(text) => {
                        match E::de(&text) {
                            Ok(error_result) => Err(error_result),
                            Err(e) => Err(E::deserialization_error(e)), // Fallback if E deserialization fails
                        }
                    }
                    Err(e) => Err(E::read_error(e)), // Error getting text from response
                }
            }
        }
        Err(_) => Err(E::default()),
    }
}

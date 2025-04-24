use std::{error::Error, time::Duration};

use reqwest::blocking::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("generic fetch error: {0}")]
    Generic(Box<dyn Error>),
    #[error("HTTP error, code: {code}")]
    Http {
        code: u64,
        retry_after: Option<Duration>,
    },
}

impl From<reqwest::Error> for FetcherError {
    fn from(value: reqwest::Error) -> Self {
        Self::Generic(value.into())
    }
}

pub struct Fetcher {
    api_key: String,
    client: Client,
}

impl Fetcher {
    pub fn new(api_key: String) -> Result<Self, FetcherError> {
        let client = Client::builder().build()?;

        Ok(Self { api_key, client })
    }

    pub fn fetch_projects(&self) -> Result<String, FetcherError> {
        let request = self
            .client
            .get("https://app.asana.com/api/1.0/projects")
            .bearer_auth(&self.api_key)
            .build()?;

        self.client
            .execute(request)?
            .text()
            .map_err(FetcherError::from)
    }

    /// Experimental
    pub fn fetch_projects_paginated(&self, limit: u64, offset: Option<String>) -> Result<String, FetcherError> {
        let builder = self
            .client
            .get("https://app.asana.com/api/1.0/projects")
            .bearer_auth(&self.api_key)
            .query(&[("limit", limit)]);

        let request = if let Some(offset) = offset {
            builder.query(&[("offset", offset)]).build()?
        } else {
            builder.build()?
        };

        self.client
            .execute(request)?
            .text()
            .map_err(FetcherError::from)
    }

    pub fn fetch_users(&self) -> Result<String, FetcherError> {
        let request = self
            .client
            .get("https://app.asana.com/api/1.0/users")
            .bearer_auth(&self.api_key)
            .build()?;

        let response = self.client.execute(request)?;

        if let Err(error) = response.error_for_status_ref() {
            let retry_after: Option<u64> = response
                .headers()
                .get("Retry-After")
                .map(|v| v.to_str().unwrap_or("").parse().unwrap_or(0));
            let retry_after = match retry_after {
                Some(0) | None => None,
                Some(r) => Some(Duration::from_secs(r)),
            };

            return Err(FetcherError::Http {
                code: error
                    .status()
                    .map(|s| s.as_u16() as u64)
                    .unwrap_or_default(),
                retry_after,
            });
        }

        response.text().map_err(FetcherError::from)
    }

    /// Experimental
    pub fn fetch_users_paginated(&self, limit: u64, offset: Option<String>) -> Result<String, FetcherError> {
        let builder = self
            .client
            .get("https://app.asana.com/api/1.0/users")
            .bearer_auth(&self.api_key)
            .query(&[("limit", limit)]);

        let request = if let Some(offset) = offset {
            builder.query(&[("offset", offset)]).build()?
        } else {
            builder.build()?
        };

        let response = self.client.execute(request)?;

        if let Err(error) = response.error_for_status_ref() {
            let retry_after: Option<u64> = response
                .headers()
                .get("Retry-After")
                .map(|v| v.to_str().unwrap_or("").parse().unwrap_or(0));
            let retry_after = match retry_after {
                Some(0) | None => None,
                Some(r) => Some(Duration::from_secs(r)),
            };

            return Err(FetcherError::Http {
                code: error
                    .status()
                    .map(|s| s.as_u16() as u64)
                    .unwrap_or_default(),
                retry_after,
            });
        }

        response.text().map_err(FetcherError::from)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static AUTH_TOKEN: &str =
        "2/1210066817168544/1210066924811437:4751cc07fd5a98597a1c10db54d5c89a";

    #[test]
    fn test_fetch_projects() {
        let fetcher = Fetcher::new(AUTH_TOKEN.to_string()).unwrap();

        let projects = fetcher.fetch_projects().unwrap();
        print!("{projects}");
        assert!(!projects.is_empty())
    }

    #[test]
    fn test_fetch_users() {
        let fetcher = Fetcher::new(AUTH_TOKEN.to_string()).unwrap();

        let users = fetcher.fetch_users().unwrap();
        print!("{users}");
        assert!(!users.is_empty())
    }
}

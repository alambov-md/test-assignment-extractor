use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{entity::Entity, next_page::NextPage};

#[derive(Error, Debug)]
#[error("parse error: {0}")]
pub struct ParserError(serde_json::Error);

impl From<serde_json::Error> for ParserError {
    fn from(value: serde_json::Error) -> Self {
        Self(value)
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse_entities(&self, body: String) -> Result<Vec<Entity>, ParserError> {
        let body_parsed: Body = serde_json::from_str(&body)?;

        Ok(body_parsed.data)
    }

    /// Experimental
    pub fn parse_entities_paginated(&self, body: String) -> Result<PaginatedBody, ParserError> {
        let body_parsed: PaginatedBody = serde_json::from_str(&body)?;

        Ok(body_parsed)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    data: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedBody {
    pub data: Vec<Entity>,
    pub next_page: NextPage,
}
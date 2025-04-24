use std::{fs, path::PathBuf};

use std::error::Error;
use thiserror::Error;

use crate::entity::Entity;

#[derive(Error, Debug)]
#[error("repo error: {0}")]
pub struct RepoError(Box<dyn Error>);

impl From<serde_json::Error> for RepoError {
    fn from(value: serde_json::Error) -> Self {
        Self(value.into())
    }
}

impl From<std::io::Error> for RepoError {
    fn from(value: std::io::Error) -> Self {
        Self(value.into())
    }
}

pub struct Repository {
    save_path: PathBuf,
}

impl Repository {
    pub fn new(save_path: PathBuf) -> Result<Self, RepoError> {
        if !&save_path.is_dir() {
            fs::create_dir_all(&save_path)?;
        }

        Ok(Self { save_path })
    }

    pub fn save(&self, entity: Entity) -> Result<(), RepoError> {
        let filename = format!("{}_{}.json", entity.resource_type, entity.name);
        let file_path = self.save_path.join(&filename);

        let contents = serde_json::to_string(&entity)?;

        fs::write(file_path, contents).map_err(RepoError::from)
    }
}

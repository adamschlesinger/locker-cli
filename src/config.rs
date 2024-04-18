//! todo
//!

use std::any::type_name;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::debug;
use crate::defaults::*;

/// Describes a created workspace. Removed when the workspace is submitted for review or merged.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Name given to the workspace
    name: String,

    /// todo
    owner: Option<String>,

    /// All paths currently owned by this workspace
    owned_paths: Vec<String>,
}

/// Primary config and data for this repo's locker setup
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    /// When files or workspaces are released what branch are they merged in to?
    pub release_branch: String,

    /// eg: claim/*, feature/*, etc. Supports options such as claim/{file_name}
    /// If not specified then users will be have to specify their own branch on Claim
    pub workspace_branch_pattern: Option<String>,

    /// Do returned files require a review step before merging into release_branch?
    pub require_review: bool,

    /// Absolute local path to the repo
    pub path: String,
}

/// Global settings passed to the executed command(s) for this run
#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig {
    /// todo
    pub repo: RepoConfig,

    /// todo
    pub current_workspace: Option<WorkspaceConfig>,

    /// todo
    pub workspaces: Vec<WorkspaceConfig>,
}

impl WorkspaceConfig {
    /// Create a new workspace
    pub fn new<S: AsRef<str> + Display>(name: &S) -> Self {
        let ws = WorkspaceConfig {
            name: name.to_string(),
            owner: None,
            owned_paths: vec![],
        };

        // todo - write out here?

        ws
    }

    /// Load a workspace by name
    pub fn load<S: AsRef<str> + Display>(name: &S) -> Option<Self> {
        load_toml(&format!("{WORKSPACES_PATH}/{name}"))
    }

    /// Save workspace
    pub fn save(&self) {}
}

impl RepoConfig {
    /// todo
    pub fn load() -> Option<Self> {
        load_toml(&CONFIG_PATH)
    }
}

/// Load a toml file to the implementing type from a path
fn load_toml<S: AsRef<str> + Display, D: DeserializeOwned>(path: &S) -> Option<D> {
    debug!("Loading {path} as {}", type_name::<D>());
    let path = Path::new(path.as_ref());

    let exists = path.try_exists()
        .unwrap_or_else(|e| panic!("{e}"));

    if exists {
        return None;
    }

    let toml_str = read_to_string(path)
        .expect(&format!("Unable to read file at {:?}", path));

    let cfg: D = toml::from_str(&toml_str)
        .expect(&format!("Unable to convert file at {:?} to {}", path, type_name::<D>()));

    Some(cfg)
}
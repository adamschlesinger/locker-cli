use std::any::type_name;
use std::fmt::Debug;
use std::fs::read_to_string;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::debug;

/// Local file describing the created workspaces. Removed when the workspace is submitted for review.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Name given to the workspace
    pub name: String,

    /// All paths currently owned by this workspace
    pub owned_paths: Vec<String>,
}

/// Primary config for this repo's locker
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    /// When files or workspaces are released what branch are they merged in to?
    pub release_branch: String,

    /// eg: claim/*, feature/*, etc. Supports options such as claim/{file_name}
    /// If not specified then users will be have to specify their own branch on Claim
    pub workspace_branch_pattern: Option<String>,

    /// Do returned files require a review step before merging into release_branch?
    pub require_review: bool,
}

/// Global settings passed to the executed command(s) for this run
#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig {
    /// todo
    pub repo_path: String,

    /// todo
    pub locker_path: String,

    /// todo
    pub config_path: String,

    /// todo
    pub workspaces_path: String,
}

impl TOMLConfig for WorkspaceConfig {}

impl TOMLConfig for RepoConfig {}

pub trait TOMLConfig: DeserializeOwned {
    fn load<S: AsRef<str> + Debug>(path: &S) -> Self {
        debug!("Loading {:?} as {}", path, type_name::<Self>());
        let path: &str = path.as_ref();

        let toml_str = read_to_string(path)
            .expect(format!("Unable to read file for {}", type_name::<Self>()).as_str());

        let cfg: Self = toml::from_str(&toml_str)
            .expect(format!("Unable to read file for {}", type_name::<Self>()).as_str());

        cfg
    }
}
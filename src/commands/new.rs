use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::commands::{CLICommand, New};
use crate::{debug, header, RunConfig};
use crate::config::WorkspaceConfig;
use crate::defaults::WORKSPACES_PATH;

impl CLICommand for New {
    fn exec(self, run_config: &RunConfig) {
        header!("Creating new workspace");

        let workspace_config = WorkspaceConfig::new(&self.name);

        let workspaces_path = Path::new(WORKSPACES_PATH);
        if !workspaces_path.exists() {
            debug!("Creating new path ({}) for workspace files", WORKSPACES_PATH);
            let _ = fs::create_dir_all(workspaces_path);
        }

        let cfg_str = toml::to_string(&workspace_config).unwrap();

        let path_str = format!("{}/{}", WORKSPACES_PATH, self.name);
        let workspace_path = Path::new(&path_str);
        debug!("Creating new workspace file {}", path_str);

        let mut file = File::create(workspace_path).unwrap();
        let _ = file.write_all(cfg_str.as_bytes());
    }
}


use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::commands::{CLICommand, New};
use crate::{debug, header, RunConfig, WorkspaceConfig};

impl CLICommand for New {
    fn exec(&self, run_config: &RunConfig) {
        header!("Creating new workspace");

        let workspace_config = WorkspaceConfig {
            name: self.name.clone(),
            paths: vec![],
        };

        let workspaces_path = Path::new(&run_config.workspaces_path);
        if !workspaces_path.exists() {
            debug!("Creating new path ({}) for workspace files", run_config.workspaces_path);
            let _ = fs::create_dir_all(workspaces_path);
        }

        let cfg_str = toml::to_string(&workspace_config).unwrap();

        let path_str = format!("{}/{}", run_config.workspaces_path, self.name);
        let workspace_path = Path::new(&path_str);
        debug!("Creating new workspace file {}", path_str);

        let mut file = File::create(workspace_path).unwrap();
        let _ = file.write_all(cfg_str.as_bytes());
    }
}


use crate::commands::{CLICommand, Init};
use crate::question::Question;
use crate::*;
use std::io::Write;
use std::process::exit;
use crate::git::lfs;

impl CLICommand for Init {
    fn exec(&self, run_config: &RunConfig) {
        header!("Starting Locker Initialization");

        let locker_path = Path::new(&run_config.locker_path);
        if !locker_path.exists() {
            debug!("Creating new path ({:?}) for configuration files", locker_path);
            let _ = fs::create_dir_all(locker_path);
        } else {
            question!("Found existing locker config. Overwrite?" {
                "y" => { /* continue */},
                "n" => { exit(exitcode::OK) }
            });
        }

        let release_branch = question!(
            "Specify branch for released files and workspaces.",
            git::origin_default().as_str()
        );

        // LFS vTODO is required, make sure it's installed
        if let Ok(lfs_version) = lfs::version() {
            debug!("LFS version {}", lfs_version);
            // todo - check lfs version
        } else {
            error!("Git LFS not found. Please install before using locker");
            exit(exitcode::UNAVAILABLE);
        }

        let config = LockerConfig {
            release_branch,
            workspace_branch_pattern: None,
            require_review: false,
        };

        info!("Creating new config file @ {}", run_config.config_path);
        let config_path = Path::new(&run_config.config_path);
        let cfg_str = toml::to_string(&config).unwrap();
        let mut file = File::create(config_path).unwrap();
        let _ = file.write_all(cfg_str.as_bytes());

        info!("Adding locker to lfs");
        lfs::track_file(LOCKER_PATH)
            .expect("TODO: panic message");

        question!("Would you like to commit and push the changes?" {
            "y" => {
                git::add(vec![".gitattributes", ".locker/*"]);

                info!("Committing configuration");
                git::commit(
                    "Added initialized locker configuration",
                    vec![".gitattributes", ".locker/*"],
                );
            },
            "n" => { exit(exitcode::OK) }
        });
    }
}
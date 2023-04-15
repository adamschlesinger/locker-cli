use crate::commands::{CLICommand, Init};
use crate::*;
use std::io::Write;
use std::process::exit;

impl CLICommand for Init {
    fn exec(&self, settings: RunSettings) {
        header!("Starting Locker Initialization");

        if !settings.locker_path.exists() {
            debug!("Creating new path for configuration files");
            let _ = fs::create_dir_all(settings.locker_path);
        }

        let branch = question!("Specify branch for returned files.", git::origin_default());

        let config = LockerConfig {
            return_branch: branch,
            claim_branch_pattern: None,
            require_review: false,
        };

        info!("Creating new config file @ {:?}", settings.config_path);
        let cfg_str = toml::to_string(&config).unwrap();

        let mut file = File::create(settings.config_path).unwrap();
        let _ = file.write_all(cfg_str.as_bytes());

        question!("Make your first workspace?" {
            "y" => {},
            "n" => {}
        });

        if let Err(_) = lfs::version() {
            question!("git lfs not found. Would you like to install it?" {
                "y" => {}, // todo
                "n" => {
                    error!("Please install git lfs before using Locker");
                    exit(exitcode::UNAVAILABLE);
                }
            });
        }
    }
}

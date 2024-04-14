use crate::commands::{CLICommand, Init};
use crate::question::Question;
use crate::*;
use std::io::Write;
use std::process::exit;
use crate::git::lfs;

impl CLICommand for Init {
    fn exec(&self, settings: &RunSettings) {
        header!("Starting Locker Initialization");

        make_locker_path(&settings);
        let release_branch = ask_release_branch();
        lfs_check_and_install();

        let config = LockerConfig {
            release_branch,
            workspace_branch_pattern: None,
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
    }
}

fn ask_release_branch() -> String {
    question!(
        "Specify branch for released files and workspaces.",
        git::origin_default().as_str()
    )
}

fn make_locker_path(settings: &RunSettings) {
    if !settings.locker_path.exists() {
        debug!(
            "Creating new path ({:?}) for configuration files",
            settings.locker_path
        );
        let _ = fs::create_dir_all(settings.locker_path);
    }
}

fn lfs_check_and_install() {
    // LFS vTODO is required, make sure it's installed
    if let Ok(lfs_version) = lfs::version() {
        debug!("LFS version {}", lfs_version);
        // todo - check lfs version
    } else {
        question!("git lfs not found. Would you like to install it?" {
            "y" => {
                warn!("todo");
            },
            "n" => {
                error!("Please install git lfs before using locker");
                exit(exitcode::UNAVAILABLE);
            },
        });
    }
}

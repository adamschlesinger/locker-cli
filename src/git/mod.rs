use crate::sh_exit;

pub mod lfs;

/// Name of the default branch
/// https://stackoverflow.com/questions/28666357/git-how-to-get-default-branch
pub fn origin_default() -> String {
    // sh!("git symbolic-ref refs/remotes/origin/HEAD | sed 's@^refs/remotes/origin/@@'");
    sh_exit!("git rev-parse --abbrev-ref origin/HEAD")
}

/// Fully qualified path to the base folder of the repo
pub fn repo_absolute_path(input_path: Option<String>) -> String {
    match input_path {
        Some(input_path) => sh_exit!("git -C {input_path} rev-parse --show-toplevel"),
        None => sh_exit!("git rev-parse --show-toplevel"),
    }
}

/// todo
pub fn commit(message: &str, paths: Vec<&str>) {
    let paths_string = reduce_paths(paths);
    sh_exit!("git commit -m \"{message}\" {paths_string}");
}

/// todo
pub fn add(paths: Vec<&str>) {
    let paths_string = reduce_paths(paths);
    sh_exit!("git add -f {paths_string}");
}

fn reduce_paths(paths: Vec<&str>) -> String {
    paths.iter()
        .map(|&s| s.to_string())
        .reduce(|acc, s| {
            if acc.is_empty() { s } else { format!("{} {}", acc, s) }
        })
        .expect("Failed to reduce paths")
}
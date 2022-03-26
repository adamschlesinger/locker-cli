use clap::{Args};

#[derive(Args, Debug)]
pub struct Claim {
    /// Which file or path with multiple claimable files you want to claim
    #[clap(short, long)]
    path: Option<String>,
}
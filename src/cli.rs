use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rsst",
    about = "Save articles from RSS channels offline.",
    rename_all = "kebab-case"
)]
pub struct Opt {
    #[structopt(short = "d", long)]
    /// Prints the list of path that articles will be write into
    pub dry_run: bool,
    #[structopt(short = "-s", long)]
    /// Prints the list of sources that this app will retrieve articles from
    pub stdout: bool,
    #[structopt(short = "-c", long, parse(from_os_str))]
    /// Loads configuration file at the path
    pub config: Option<PathBuf>,
}

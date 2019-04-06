//! the bin for `RSSt`.

use rsst::cli;
use structopt::StructOpt;

/// Attempts to retrieve feeds. Prints errors if encounter any.
pub fn main() {
    if let Err(e) = cli::run(cli::Opt::from_args()) {
        eprintln!("{}", e);
    }
}

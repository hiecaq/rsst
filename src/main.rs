use rsst::cli;
use structopt::StructOpt;

pub fn main() {
    if let Err(e) = cli::run(cli::Opt::from_args()) {
        eprintln!("{}", e);
    }
}

//! The CLI interface for `RSSt`.

use crate::config;
use crate::downstream::HTML;
use crate::metadata;
use crate::upstream::{to_source, Article};
use crate::util::{self, get_metadata_dir, get_output_dir};
use std::error::Error;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rsst",
    about = "Save articles from RSS channels offline.",
    rename_all = "kebab-case"
)]
/// The options available by run `rsst`.
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

/// Try parse the file at `PathBuf` `p` into `metadata::Collection`.
fn get_collection(p: PathBuf) -> Result<metadata::Collection, util::Error> {
    match metadata::get(p) {
        Ok(v) => Ok(v),
        Err(util::Error::NotFound) | Err(util::Error::ParseFailed) => Ok(metadata::Collection {
            ..metadata::Collection::default()
        }),
        Err(e) => Err(e),
    }
}

/// Gets the exclusive upperbound index that marks the last article should dump.
fn get_bound(metadata: Option<metadata::Metadata>, article: &[Article]) -> usize {
    match metadata {
        Some(metadata) => match article.iter().position(|a| a.checksum == metadata.checksum) {
            Some(v) => v,
            None => article.len(),
        },
        None => article.len(),
    }
}

/// Run the given command in `opt`. Return an `Error` if failed at any point.
pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let config = config::get(opt.config)?;
    let metadata_dir = get_metadata_dir(config.setting.metadata_dir)?;
    create_dir_all(&metadata_dir)?;
    let mut collection = get_collection(metadata_dir.join("collections.json"))?;
    let output_dir: std::path::PathBuf = get_output_dir(config.setting.output_dir)?;
    create_dir_all(&output_dir)?;
    let output_format = match config.setting.output_format {
        Some(v) => v,
        None => String::from("html"),
    };
    for (alias, source) in &config.source {
        let dir = output_dir.join(alias);
        create_dir_all(&dir)?;
        if opt.dry_run {
            println!(
                "{} -> {}",
                dir.to_str().expect("failed to convert to path"),
                source
            );
        } else {
            let source = to_source(source)?;
            let bound = get_bound(
                collection
                    .metadata
                    .insert(String::clone(alias), source.metadata),
                &source.article,
            );
            let turn_into = if output_format == "html" {
                HTML::from
            } else {
                panic!("unsupported");
            };
            let output: Vec<_> = source.article[..bound]
                .iter()
                .map(|a| turn_into(a))
                .collect();
            if opt.stdout {
                println!("{:?}", collection.metadata[alias]);
                for o in output {
                    println!("{}", o.to_string());
                }
            } else {
                for o in output.iter().rev() {
                    let filepath = dir.join(o.filename());
                    println!("dumping {} ...", filepath.to_str().unwrap());
                    write(filepath, o.to_string())?;
                }
            }
        }
    }
    write(metadata_dir.join("collections.json"), collection.put()?)?;
    Ok(())
}

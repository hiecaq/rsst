use rsst::cli;
use rsst::config;
use rsst::downstream::HTML;
use rsst::metadata;
use rsst::upstream::to_source;
use rsst::util::{self, get_metadata_dir, get_output_dir};
use std::error::Error;
use std::fs::{create_dir_all, write};
use structopt::StructOpt;

fn run(opt: cli::Opt) -> Result<(), Box<dyn Error>> {
    let config = config::get(opt.config)?;
    let metadata_dir = get_metadata_dir(config.setting.metadata_dir)?;
    create_dir_all(&metadata_dir)?;
    let mut collection = match metadata::get(metadata_dir.join("collections.json")) {
        Ok(v) => v,
        Err(util::Error::NotFound) | Err(util::Error::ParseFailed) => metadata::Collection {
            ..metadata::Collection::default()
        },
        Err(e) => return Err(Box::new(e)),
    };
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
            let bound = match collection
                .metadata
                .insert(String::clone(alias), source.metadata)
            {
                Some(metadata) => {
                    match source
                        .article
                        .iter()
                        .position(|a| a.checksum == metadata.checksum)
                    {
                        Some(v) => v,
                        None => source.article.len(),
                    }
                }
                None => source.article.len(),
            };
            let turn_into = if output_format == "html" {
                HTML::from
            } else {
                panic!("un-supported");
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
                    write(dir.join(o.filename()), o.to_string())?;
                }
            }
        }
    }
    write(metadata_dir.join("collections.json"), collection.put()?)?;
    Ok(())
}

pub fn main() {
    if let Err(e) = run(cli::Opt::from_args()) {
        eprintln!("{}", e);
    }
}

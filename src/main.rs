use clap::Parser;
use evaporust::ProjectFinder;
use spinners::{Spinner, Spinners};
use std::{io, path::PathBuf};

#[derive(Debug, Parser)]
struct Options {
    #[arg(
        long,
        short = 'a',
        required = false,
        help = "Scan and clean all projects, even if they are already clean",
        default_value = "false"
    )]
    all_projects: bool,

    #[arg(
        long,
        short,
        required = false,
        help = "Directory from which to start scanning for projects",
        value_name = "DIRECTORY"
    )]
    base_dir: Option<String>,

    #[arg(
        long,
        short,
        required = false,
        help = "Scan for projects but don't actually run `cargo clean`",
        default_value = "false"
    )]
    dry_run: bool,

    #[arg(
        long,
        short,
        required = false,
        help = "Print total number of projects found",
        default_value = "false"
    )]
    total: bool,

    #[arg(
        long,
        short,
        required = false,
        help = "Print a list of all projects found",
        default_value = "false"
    )]
    list: bool,

    #[arg(
        long,
        short = 'x',
        required = false,
        help = "Comma-separated list of strings. Paths that contain any of these will be skipped",
        value_delimiter = ','
    )]
    exclude: Vec<String>,
}

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let options = Options::parse();

    let base_dir = {
        if let Some(s) = options.base_dir {
            PathBuf::from(s)
        } else {
            // *probably* won't ever fail ¯\_(ツ)_/¯
            std::env::current_dir().ok().unwrap()
        }
    };

    let mut walker = ProjectFinder::new(base_dir, options.exclude, options.all_projects);

    let mut spin = Spinner::new(Spinners::Dots2, "Scanning for projects...".into());
    _ = walker.walk()?;
    spin.stop();

    // Avoid clobbering the previous output
    println!();

    if options.list {
        walker.projects.sort();
        println!("{:#?}", walker.projects);
    }

    if options.total {
        println!("found {} projects", walker.projects.len());
    }

    if !options.dry_run {
        let mut spin = Spinner::new(Spinners::Dots2, "Cleaning up...".into());
        walker.clean();
        spin.stop();
        println!("done.")
    }

    Ok(())
}

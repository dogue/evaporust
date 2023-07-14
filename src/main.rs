use clap::Parser;
use evaporust::ProjectFinder;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Options {
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

fn main() {
    pretty_env_logger::init();

    let options = Options::parse();

    let base_dir = {
        if let Some(s) = options.base_dir {
            PathBuf::from(s)
        } else {
            // *probably* wonht ever fail ¯\_(ツ)_/¯
            std::env::current_dir().ok().unwrap()
        }
    };

    let mut walker = ProjectFinder::new(base_dir, options.exclude);

    _ = walker.walk();

    if options.total {
        println!("Found {} projects", walker.projects.len());
    }

    if options.list {
        println!("{:#?}", walker.projects);
    }

    if !options.dry_run {
        walker.clean();
    }
}

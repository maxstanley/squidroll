use std::{fs, path::PathBuf};

use clap::Parser;

use serde::Serialize;
use squidroll::{
    parser::{adblockplus::AdblockPlus, domainlist::DomainList, ParseFile},
    trie::Trie,
};

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Format {
    #[default]
    DomainList,
    // https://adblockplus.org/filter-cheatsheet
    AdblockPlus,
}

/// Program to parse blocklists into a Squid compatible format.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to parse
    #[arg(short, long)]
    filepath: PathBuf,

    /// Output file
    #[arg(short, long)]
    output_filepath: Option<PathBuf>,

    /// Mark all subdomains as wildcards
    #[arg(short, long, default_value_t = false)]
    wildcard_mark: bool,

    #[arg(long, default_value = "domain-list")]
    format: Format,
}

fn main() {
    let args = Args::parse();

    let input_lines = fs::read_to_string(args.filepath).unwrap();

    let parser: Box<dyn ParseFile> = match args.format {
        Format::AdblockPlus => Box::new(AdblockPlus {}),
        Format::DomainList => Box::new(DomainList {}),
    };

    let mut trie = Trie::new();
    for line in parser.parse_file(&input_lines) {
        if args.wildcard_mark && !line.starts_with(".") {
            trie.insert(&format!(".{}", line));
        } else {
            trie.insert(&line);
        }
    }

    if let Some(outfile) = args.output_filepath {
        fs::write(outfile, format!("{}", trie)).unwrap();
    } else {
        print!("{}", trie);
    }
}

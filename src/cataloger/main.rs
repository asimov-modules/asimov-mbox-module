// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-mbox-cataloger requires the 'std' feature");

use asimov_mbox_module::MboxReader;
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{Uri, UriScheme::File, UriValueParser};
use std::error::Error;

/// asimov-mbox-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The maximum number of messages to catalog.
    #[arg(short = 'n', long)]
    limit: Option<usize>,

    /// The output format.
    #[arg(short = 'o', long)]
    output: Option<String>,

    /// A `file:/path/to/messages.mbox` URL to the file to catalog.
    #[arg(id = "MBOX-FILE-URL", value_parser = UriValueParser::new(&[File]))]
    mbox_url: Uri<'static>,
}

fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Open the mbox file:
    let mbox = MboxReader::open(options.mbox_url.path())?;

    // Scan the mbox messages:
    for (index, entry) in mbox
        .iter()
        .take(options.limit.unwrap_or(usize::MAX))
        .enumerate()
    {
        let email = entry?;
        if index > 0 {
            println!();
        }
        print!("{}", email.headers.detailed());
    }

    Ok(EX_OK)
}

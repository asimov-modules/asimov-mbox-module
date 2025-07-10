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
    #[arg(value_name = "COUNT", short = 'n', long)]
    limit: Option<usize>,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    /// A `file:/path/to/messages.mbox` URL to the file to catalog.
    #[arg(value_name = "MBOX-FILE-URL", value_parser = UriValueParser::new(&[File]))]
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
    let messages = mbox.iter().take(options.limit.unwrap_or(usize::MAX));
    match options
        .output
        .as_ref()
        .unwrap_or(&String::default())
        .as_str()
    {
        "jsonld" | "json" => {
            use know::traits::ToJsonLd;
            let mut output = Vec::new();
            for message in messages {
                let message = message?;
                output.push(message.headers.to_jsonld()?);
            }
            if cfg!(feature = "pretty") {
                colored_json::write_colored_json(&output, &mut std::io::stdout())?;
                println!();
            } else {
                todo!() // TODO
            }
        },
        _ => {
            for (index, message) in messages.enumerate() {
                let message = message?;
                if index > 0 {
                    println!();
                }
                print!("{}", message.headers.detailed());
            }
        },
    }

    Ok(EX_OK)
}

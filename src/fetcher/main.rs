// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-mbox-fetcher requires the 'std' feature");

use asimov_mbox_module::MboxReader;
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{Uri, UriScheme::File, UriValueParser};
use know::datatypes::EmailMessageId;
use std::error::Error;

/// asimov-mbox-fetcher
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(id = "FORMAT", short = 'o', long)]
    output_format: Option<String>,

    /// A `file:/path/to/messages.mbox#mid` URL to the message to fetch.
    #[arg(id = "MBOX-MESSAGE-URL", value_parser = UriValueParser::new(&[File]))]
    message_url: Uri<'static>,
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
    let mbox = MboxReader::open(options.message_url.path())?;

    // Fetch the mbox message:
    let message_id: EmailMessageId = options
        .message_url
        .fragment_str()
        .expect("message ID should be given in the URL fragment")
        .into();
    match mbox.fetch(&message_id)? {
        Some(message) => {
            match options.output_format.unwrap_or_default().as_str() {
                "jsonld" | "json" => print!("{}", message.headers.jsonld()),
                "mime" | _ => {
                    print!("{}", message.headers.mime());
                    if let Some(body) = message.body {
                        println!();
                        print!("{}", body);
                    }
                },
            }
            Ok(EX_OK)
        },
        None => {
            eprintln!("message ID <{}> not found", message_id.as_str());
            Ok(EX_NOINPUT)
        },
    }
}

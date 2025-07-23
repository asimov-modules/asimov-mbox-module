# ASIMOV Mbox Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-mbox-module)](https://crates.io/crates/asimov-mbox-module)
[![Documentation](https://docs.rs/asimov-mbox-module/badge.svg)](https://docs.rs/asimov-mbox-module)

[ASIMOV] module for [Mbox] email import.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install mbox -v
```

### Installation from Source Code

```bash
cargo install asimov-mbox-module
```

## 👉 Examples

### Email Import from an Mbox File

#### Cataloging email messages in the mbox

```bash
asimov-mbox-cataloger file:/path/to/messages.mbox
```

#### Fetching a specific email message

```bash
asimov-mbox-fetcher file:/path/to/messages.mbox#mid
```

## ⚙ Configuration

This module requires no configuration.

## 📚 Reference

### `asimov-mbox-cataloger`

```
asimov-mbox-cataloger

Usage: asimov-mbox-cataloger [OPTIONS] <MBOX-FILE-URL>

Arguments:
  <MBOX-FILE-URL>  A `file:/path/to/messages.mbox` URL to the file to catalog

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -n, --limit <COUNT>    Limit the number of messages to catalog
  -o, --output <FORMAT>  Set the output format [default: cli] [possible values: cli, json, jsonld, jsonl]
  -h, --help             Print help
```

### `asimov-mbox-fetcher`

```
asimov-mbox-fetcher

Usage: asimov-mbox-fetcher [OPTIONS] <MBOX-MESSAGE-URL>

Arguments:
  <MBOX-MESSAGE-URL>  A `file:/path/to/messages.mbox#mid` URL to the message to fetch

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-mbox-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-mbox-module&text=asimov-mbox-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-mbox-module&title=asimov-mbox-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-mbox-module&t=asimov-mbox-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-mbox-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-mbox-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[Mbox]: https://en.wikipedia.org/wiki/Mbox
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org

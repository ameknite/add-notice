# add-notice

[![License](https://img.shields.io/badge/license-MPL2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![Crates.io](https://img.shields.io/crates/v/add-notice.svg)](https://crates.io/crates/add-notice)
![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.81.0+-red)
[![CI](https://github.com/ameknite/add-notice/workflows/CI/badge.svg)](https://github.com/ameknite/add-notice/actions?workflow=CI)

```bash
A cli tool to add headers notices to files

Usage: add-notice [OPTIONS]

Options:
  -n, --notice <NOTICE>
          path to the notice file [default: ./NOTICE]
      --dir <DIR>
          directory to apply the notice [default: .]
  -e, --extensions <EXTENSIONS>
          select files by extension, e.g. rs,js,kt [default: rs]
  -c, --comment-style <COMMENT_STYLE>
          comment style [default: //]
  -r, --remove
          remove notice in files
      --replace-with-string <REPLACE_WITH_STRING>
          replace notice with string
      --replace-with-file <REPLACE_WITH_FILE>
          replace notice with file content
  -h, --help
          Print help
  -V, --version
          Print version
```

## Installation

### Cargo [install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)

Compile the crate yourself with:

```rust
cargo install add-notice
```

### Cargo [binstall](https://github.com/cargo-bins/cargo-binstall)

Install a binary version:

```rust
cargo binstall add-notice
```

## Purpose

Makes the process of adding header notices to files easier.

Like those requested by licenses such as MPL2.0:

```txt
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
```

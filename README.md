<div align="center">


# psource
**p**rint **source** code

CLI tool to pretty print source code to stdout or directly to the clipboard.

The tool is created to quickly provide source code context to a large language model (LLM).

[![crates.io version](https://img.shields.io/crates/v/psource)](https://crates.io/crates/psource)
[![AUR version](https://img.shields.io/aur/version/psource-git)](https://aur.archlinux.org/packages/psource-git)

[![build and test](https://img.shields.io/github/actions/workflow/status/frederikstroem/psource/build_and_test.yml?label=build%20and%20test)](https://github.com/frederikstroem/psource/actions/workflows/build_and_test.yml)
[![docs.rs](https://img.shields.io/docsrs/psource)](https://docs.rs/psource)

</div>

---

⚠️ This tool is still in early development, expect breaking changes.

⏳️ See [CHANGELOG.md](CHANGELOG.md) for the latest changes and [roadmap](CHANGELOG.md#unreleased).

---

## Install
### Install using Cargo
Make sure to have Cargo installed and PATH configured. See [The Rust Programming Language - Installing Binaries with cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html).

#### Install from crates.io
```bash
cargo install psource
```

#### Install from GitHub source
```bash
cargo install --git https://github.com/frederikstroem/psource.git
```

#### Install from local source
```bash
cargo install --path .
```

### Arch Linux
#### Install via the Arch User Repository (AUR)
Install using your favorite AUR helper, e.g., [paru](https://github.com/Morganamilo/paru):
```bash
paru -S psource-git
```

## Configuration
The tool can be configured using a configuration file. The configuration file is a simple TOML file with the following structure:

```toml
# Copy the source code to the clipboard instead of printing it to stdout (default: false)
clipboard_is_default_output_target = false
```

psource will look for a configuration file in the following `$HOME/.config/psource/config.toml`.

## Get started
Get help:
```plaintext
psource --help
CLI tool to pretty print source code to stdout or directly to the clipboard. Skips binary files.

Usage: psource [OPTIONS] <INPUT>...

Arguments:
  <INPUT>...  Input files and directories

Options:
  -s, --stdout               Print the source code to stdout
  -c, --copy                 Copy the source code to the clipboard
  -a, --ancestry <ANCESTRY>  Display the file's ancestry in the output path by including the specified number of parent directories relative to the current working directory, or 0 to omit the ancestry [default: 1]
  -h, --help                 Print help
  -V, --version              Print version
```

### Example: Add a to_uppercase utils function to a Rust project
We have created a simple Rust project with a `lib.rs` file and a `main.rs` file.

**main.rs:**
```rust
mod lib;

fn main() {
    let message = lib::greet("Rustacean");
    println!("{}", message);
}
```

**lib.rs:**
```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

We want to add a `to_uppercase` function to a new `utils.rs` file and use it in `lib.rs`. We can use the `psource` tool to quickly copy the relevant code to the clipboard.

`cd` into the project root directory and run:
```bash
psource -c src
```

This command will place the following content onto your clipboard:
```plaintext
⚫ /simple_rust_program/src/main.rs
mod lib;

fn main() {
    let message = lib::greet("Rustacean");
    println!("{}", message);
}

⚫ /simple_rust_program/src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

```

You can now prompt the LLM with the following:
```plaintext
Please add a to_uppercase function to a new utils.rs file. Modify the greet function in lib.rs to use the to_uppercase function.

Source code:

⚫ /simple_rust_program/src/main.rs
mod lib;

fn main() {
    let message = lib::greet("Rustacean");
    println!("{}", message);
}

⚫ /simple_rust_program/src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

```

The LLM should be able to help you complete the task.

**Note:** This is a very simple example but scales really well to large projects. The limit largely depends on the LLM's capabilities and its context window. I've had great success with [OpenAI's GPT-4 Turbo](https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo), used with an alternative front-end utilizing the API, I can recommend [chatgpt-web](https://github.com/Niek/chatgpt-web). GPT-4 Turbo has a context window of 128,000 tokens, and I've also found setting a sample temperature of ±0.6 to work well for code development tasks.

**Tip:** A file tree structure can sometimes help the LLM to better understand the context of the code. For such a task, I recommend using [eza](https://github.com/eza-community/eza) with the `--tree` option. To pipe it to the clipboard, a tool like [xsel](https://github.com/kfish/xsel) can be used, e.g., `eza --tree | xsel -b`.


## Known issues
### Speeding up the copy to clipboard process
Due to a bug in the software supply chain, the `-c` option requires psource to wait for some time before exiting, else the clipboard will not be updated on some systems (discovered on KDE Plasma running X11).[[1]](https://github.com/1Password/arboard/issues/114)[[2]](https://github.com/sigoden/aichat/issues/160) To speed up the process, the `psource` stdout can be piped to a clipboard tool like [xsel](https://github.com/kfish/xsel), e.g., `psource src | xsel -b`.

//! Zed extension that runs [`ron-lsp`](https://github.com/jasonjmcghee/ron-lsp) for RON files.
//!
//! Zed only starts language servers that an extension has registered, so this crate does the
//! registration: `extension.toml` binds the language server id `ron-lsp` to the `RON` language, and
//! [`RonLspExtension::language_server_command`] tells Zed which binary to launch.
//!
//! Binary resolution order:
//! 1. `"lsp": { "ron-lsp": { "binary": { "path": ..., "arguments": [...], "env": {...} } } }` from Zed settings,
//! 2. `ron-lsp` found in `PATH`,
//! 3. an error naming both fixes.

use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

/// Binary installed via `cargo install ron-lsp` or overridden in Zed settings.
const BINARY_NAME: &str = "ron-lsp";

struct RonLspExtension;

impl zed::Extension for RonLspExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        // Explicit override, e.g.
        // "lsp": { "ron-lsp": { "binary": { "path": "C:\\...\\ron-lsp.exe", "arguments": [] } } }
        let settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .unwrap_or_default();
        if let Some(binary) = settings.binary {
            if let Some(path) = binary.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary.arguments.unwrap_or_default(),
                    env: binary.env.unwrap_or_default().into_iter().collect(),
                });
            }
        }

        match worktree.which(BINARY_NAME) {
            Some(command) => Ok(zed::Command {
                command,
                args: Vec::new(),
                env: Vec::new(),
            }),
            None => Err(format!(
                "`{BINARY_NAME}` not found in PATH. Install it with `cargo install ron-lsp`, \
                 or point Zed at the binary: \"lsp\": {{\"{language_server_id}\": \
                 {{\"binary\": {{\"path\": \"/absolute/path/to/{BINARY_NAME}\"}}}}}}"
            )),
        }
    }
}

zed::register_extension!(RonLspExtension);

# RON LSP for Zed

Zed extension that runs [ron-lsp](https://github.com/jasonjmcghee/ron-lsp) for `.ron` files: type-aware
diagnostics, completion, hover, go-to-definition, rename, code actions and formatting — all driven by the
Rust types the project deserializes those files into.

Zed ships no RON language server, and it [only starts language servers that an extension has
registered](https://github.com/zed-industries/zed/issues/52653) — declaring `lsp.ron-lsp.binary.path` in
`settings.json` alone does nothing. This extension is that registration: it binds the `RON` language to
the `ron-lsp` binary.

## Features

Provided by ron-lsp itself (0.1.x), based on your Rust type definitions:

- **Diagnostics** — unknown fields, missing required fields, type mismatches, RON syntax errors
  (honours `#[derive(Default)]`: omittable fields are not reported, and custom `Deserialize` impls are skipped)
- **Completion, hover** (field types and doc comments from Rust), **go to definition**, **rename**
- **Code actions** — insert required or missing fields
- **Formatting** — full document and range; the server can also act as your Zed formatter
  (`"languages": { "RON": { "formatter": "language_server" } }`)

The same binary works from the CLI without an editor: `ron-lsp check [path]`, `ron-lsp format [path]`.

## Requirements

| Piece | Why / how |
| --- | --- |
| Zed | any release whose extension API supports `0.7.0`; tested on Zed 1.24.2 (stable) |
| `ron-lsp` | the actual language server: `cargo install ron-lsp --locked`, must be on `PATH` |
| RON extension | [zed.dev/extensions/ron](https://zed.dev/extensions/ron) — provides the `RON` language (grammar) this server attaches to |

## Installation

The extension is not in the Zed extension registry yet, so install it as a **dev extension**:

```bash
# 1. the language server
cargo install ron-lsp --locked

# 2. this extension
git clone https://github.com/HoCoK31/zed-ron-lsp
```

3. In Zed, install the **RON** extension (command palette → `zed: extensions` → search `RON`).
4. In Zed, run `zed: install dev extension` and select the cloned `zed-ron-lsp` folder.
   Zed compiles it for `wasm32-wasip2` itself and installs the Rust target if it is missing.
5. Open any `.ron` file. The status bar shows `RON`, and the language server starts for the project.

Once published to the registry, this step becomes *Extensions → search “RON LSP” → Install*.

## Configuration

By default the extension resolves `ron-lsp` from `PATH`. To pin an explicit binary (different name,
local build, non-standard install location):

```json
{
  "lsp": {
    "ron-lsp": {
      "binary": {
        "path": "/absolute/path/to/ron-lsp"
      }
    }
  }
}
```

No `languages.language_servers` entry is needed — the server is not opt-in, so it starts automatically
whenever a `.ron` file is open.

### How the server learns your types

Either annotate the file:

```ron
/* @[crate::models::User] */

User(
    id: 1,
    name: "Alice",
)
```

…or map globs to types in a `ron.toml` next to your `Cargo.toml`:

```toml
[[types]]
glob = "**/*.post.ron"
type = "crate::models::Post"
```

The annotation in the file wins over `ron.toml`. Types are collected from the `.rs` files of the
**first workspace folder**, so open the project folder rather than a single file if you want type-aware
results — the annotation is resolved against that tree. See the
[upstream README](https://github.com/jasonjmcghee/ron-lsp#readme) for the full format.

## Limitations

- The root of the file must be a **named type** from your crate. A generic root such as `Vec<…>` cannot be
  annotated yet (`Could not find type: Vec<crate::models::User>`); wrap it in a newtype and point the
  annotation at the newtype.
- Rust types are scanned once per workspace root (a `ron.toml` change re-reads the config, but editing your
  `.rs` files requires reopening the project or restarting the server to refresh the type cache).
- Only files Zed identifies as language `RON` are attached, i.e. `*.ron` handled by the RON extension.
- No diagnostics at all is the normal state for a file without an annotation or a matching `ron.toml` entry —
  only syntax is checked then.

## Troubleshooting

| Symptom | Fix |
| --- | --- |
| Zed reports `` `ron-lsp` not found in PATH `` | `cargo install ron-lsp --locked`, reopen Zed so it inherits the updated `PATH`, or set `lsp.ron-lsp.binary.path` |
| Server never starts | make sure the **RON** extension is installed (language must be `RON`), then check `zed: open log` / the language server logs |
| No type errors shown | add `/* @[crate::…] */` to the file or a `ron.toml`; open the project folder, not a lone file |
| Diagnose the server outside Zed | `ron-lsp check path/to/file.ron` — the CLI accepts a file or a directory |

## Development

Requirements: Rust toolchain with the `wasm32-wasip2` target (`rustup target add wasm32-wasip2`).

```bash
cargo build --release --target wasm32-wasip2
# → target/wasm32-wasip2/release/zed_ron_lsp.wasm
```

Installing the folder via `zed: install dev extension` builds the same artifact and copies it to
`extension.wasm` next to `extension.toml`. After changing the sources, rebuild — Zed reloads a dev
extension when its `extension.toml` / `extension.wasm` changes, or run `zed: rebuild dev extension`.
Note that a reload *stops* the language server that was already running for open buffers; restart Zed
(or close and reopen the `.ron` file) to start it again.

The manifest is intentionally tiny:

```toml
[language_servers.ron-lsp]
languages = ["RON"]   # must match `name` in the RON grammar extension's languages/ron/config.toml
```

Everything else this extension does is `Extension::language_server_command` in `src/lib.rs`: read the
`lsp.ron-lsp.binary` override from Zed settings, otherwise `worktree.which("ron-lsp")`, otherwise return an
error explaining how to install it.

## Credits

- [ron-lsp](https://github.com/jasonjmcghee/ron-lsp) by Jason McGhee — the language server (MIT)
- [zed-ron](https://github.com/onbjerg/zed-ron) by onbjerg — RON grammar/language for Zed
- This extension is only the Zed glue; the protocol work happens upstream.

## License

MIT — see [LICENSE](LICENSE).

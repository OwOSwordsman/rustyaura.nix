# rustyaura.nix

A formatter that wraps around `rustfmt`, `prettierd` (with tailwind prettier plugin), and `leptosfmt`. The formatter reads from stdin and outputs formatted code to stdout.

## Usage

Configure your editor's formatter or rust-analyzer to use `rustyaura`. For example, in VS Code add the following to `.vscode/settings.json`.
```json
{
  "rust-analyzer.rustfmt.overrideCommand": ["rustyaura"]
}
```

Alternatively, you can invoke `rustyaura` from the command line.
```sh
cat main.rs | rustyaura > main.rs
```

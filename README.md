# painless-lsp

Language server for Elasticsearch's Painless scripting language.

## Features

- Syntax highlight
    - Semantic tokens (full) only

## Planned for future implementation

- Goto definition
- Find reference
- Hover
- Diagnostic
- Rename
- Signature Help
- Semantic Token (full_delta)
- Semantic Token (range)

[TODO](./TODO.md)

## Notice

This Language server has the following issues. These problems occurred during the process of porting the parser from Elasticsearch to Rust. We plan to address these issues in the future.

- dangling else
- (probably) confusing the slash used for division with the slash used in regular expressions

## For Development

### How to initialize

### How to Update Parser Code Based on Elasticsearch Definitions

Due to the constraints of the library used to utilize ANTLR definitions in Rust, some features need to be disabled or fixed.

1. Copy the contents from https://github.com/elastic/elasticsearch/tree/main/modules/lang-painless/src/main/antlr to `assets/antlr/`
2. Rewrite the section that uses `_input.LA(1)` to a format that does not use `_input.LA(1)`.
3. Run `make build_antlr`
4. For the generated `painlessparser.rs`:
    1. Rename `type` to `type_`
    2. Add `#![allow(unused_parens)]`
    3. Rename `PainlessParserParserContext` to `PainlessParserContext`
5. For the generated `painlesslexer.rs`:
    1. Modify `fn isShashRegex` to always return false

### How to Publish

1. `make build_release`
2. (For local debugging, run `code --install-extension dist/painless-lsp-x.y.z.vsix`)
3. Upload `dist/painless-lsp-x.y.z.vsix` file manually
    - [WEB UI](https://marketplace.visualstudio.com/) or command

### References

- LSP
    - [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
    - [VS Code Language Server Extension Guide](https://code.visualstudio.com/api/language-extensions/language-server-extension-guide)
- LSP sample
    - [rust-analyzer](https://github.com/rust-lang/rust-analyzer)
    - [haskell/lsp](https://github.com/haskell/lsp)
- LSP libraries
    - [lsp_types](https://docs.rs/lsp-types/latest/lsp_types/)
    - [tower-lsp](https://github.com/ebkalderon/tower-lsp)
- VSCode extension
    -[Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
- painless
    - [Painless Language Specification](https://www.elastic.co/guide/en/elasticsearch/painless/8.17/painless-lang-spec.html)
- antlr4rust
    - https://github.com/rrevenantt/antlr4rust

## Note

This repository is based on https://github.com/IWANABETHATGUY/tower-lsp-boilerplate.
And this is licensed under AGPLv3 as it includes parts of Elasticsearch code. (painless ANTLR parser)

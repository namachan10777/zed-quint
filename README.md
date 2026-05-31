# Quint for Zed

Quint syntax support for [Zed editor](https://zed.dev).

## Features

- Syntax highlighting, bracket matching, and indentation (via tree-sitter)
- Language server integration powered by the official
  [`@informalsystems/quint-language-server`](https://www.npmjs.com/package/@informalsystems/quint-language-server)
  (diagnostics, completion, hover, go-to-definition, etc.)

## Language Server

The extension looks for a `quint-language-server` executable on your `PATH` first.
If you already have it installed globally, that binary is used:

```sh
npm i -g @informalsystems/quint-language-server
```

If no executable is found on `PATH`, Zed automatically downloads and manages the
npm package for you (Node.js is required since the language server runs on Node).

## Dependency

- [tree-sitter-quint](https://github.com/gruhn/tree-sitter-quint)
  - copyright: Niklas Gruhn

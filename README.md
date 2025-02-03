# Domestic

A Rust project for local tasks and utilities - with LLM / RAG /ChatGPT


## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [License](#license)


# Introduction

Create a local ChatGPT - which could work on a single machine, without access to internet.

- [x] Basic functionality - PoC using existing tools: 
    - [x] ollama install with local models
    - [x] goose for integration between ollama and web, intellij/vscode, etc.
    - [ ] basic chat functionality
    - [ ] basic weg serving functionality
- [ ] move from ollama to local models served locally in rust/ native implementation (ollama is go)
- [ ] move from goos to local integrations // simplify integrations - quite opinionated
- [ ] integrate with local document processing / personal data -> knowledge extraction 
- [ ] extend to use neo4j =>  (PoC) if that will work at all 



## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## License

This project is licensed under the Apache License - see the LICENSE file for details.

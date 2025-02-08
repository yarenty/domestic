# Domestic

A Rust project for local tasks and utilities - with LLM / RAG /ChatGPT


## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [License](#license)


## Introduction


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
- CUDA (if you want to use nvidia models) and .... good luck with installation! ;-)
- ollama
- goose

### Rust

https://www.rust-lang.org/tools/install


```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### CUDA

https://developer.nvidia.com/cuda



### Ollama installation

https://ollama.com/download


_tip (2024/01): if you want to use DeepSeek model, after installation use: 'ollama run michaelneale/deepseek-r1-goose'_


Ollama should work by default on: http://localhost:11434 


### Goose instalation


https://block.github.io/goose/

or get git:
https://github.com/block/goose


then check providers:
https://block.github.io/goose/docs/getting-started/providers

and specify 

```bash
┌   goose-configure 
│
◇  What would you like to configure?
│  Configure Providers 
│
◇  Which model provider should we use?
│  Ollama 
│
◇   Provider Ollama requires OLLAMA_HOST, please enter a value
│  http://localhost:11434  
│    
◇  Enter a model from that provider:
│  michaelneale/deepseek-r1-goose
│
◇  Welcome! You're all set to explore and utilize my capabilities. Let's get started on solving your problems together!
│
└  Configuration saved successfully
```


Goose is CLI application, so you can run it with 'goose' command.

Additional configuration: TBD (all those extensions on [goose extenstions](https://block.github.io/goose/docs/getting-started/using-extensions))







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

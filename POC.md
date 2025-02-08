# POC

Proof of Concept: 
Can be domestic build and functional using existing tools and libraries?

Using manual steps.
Aka: low-level neardy solution.



## Manual steps

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





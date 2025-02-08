# Domestic

## GOAL

- **Personal ChatGPT(LLM,Co-pilot, ..) that works localy on ones machine without sharing data.**
- **Your personal information belongs to you.**
- **Simple "one-click" solution.**  No need to know anything about LLMs, RAG, or ChatGPT, or their implementations.


## Status


1. [PoC details](POC.md): Create a local ChatGPT - which could work on a single machine, without access to internet using manual (nerdy) 
2. [ ] One build
3. [ ] One delivery
4. [ ] One interface
5. [ ] Mac /Linux /Windows 



## Roadmap


- [x] Basic functionality - PoC using existing tools: 
    - [x] ollama install with local models
    - [x] goose for integration between ollama and web, intellij/vscode, etc.
    - [ ] basic chat functionality
    - [ ] basic web serving functionality
- [ ] move from ollama to local models served locally in rust/ native implementation (ollama is in go)
- [ ] move from goose to local integrations // simplify integrations - this is "quite opinionated" 
- [ ] integrate with local document processing / personal data -> knowledge extraction 
- [ ] extend to use neo4j =>  (another PoC: if that will work at all?) 




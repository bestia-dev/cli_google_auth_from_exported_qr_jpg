<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# code generator for protobuf

 ![License](https://img.shields.io/badge/license-MIT-blue.svg)

Protocol Buffers are a language-neutral, platform-neutral extensible mechanism for serializing structured data created by google. They claim it is performant and efficient, more than json and xml. Instead of a generic runtime coder/decoder, they prepare optimized source code for every specific proto message. So it is super performant after compiling. They created a code generator for 20 languages, but not for Rust (yet). There is the third party `prost` crate that looks it generates rust code from the `.proto` schema definition.  
The suggested way is to create a build.rs "script" that executes on every build. I personally don't like build.rs. It is "out of control" and frankly it does not need to be run on every build. Once is enough. I will rather make a on-time `code_generator` I can manually run and inspect the code before copying it inside my project.  

## install protoc

The `prost-build` needs `protoc` to parse the `.proto` file. `Protoc` is the official executable for code generation from the protobuf project.

<https://grpc.io/docs/protoc-installation/>

Download `protoc` from <https://github.com/protocolbuffers/protobuf/releases>.  
Unzip and drag and drop the file `protoc` into the VSCode project tree inside the folder `code_generator`. This copies the binary file from the Host OS into the rust_dev container.  
Make it executable with  
`cd code_generator`  
`chmod +x protoc`  
and then copy it into a directory that is in the `PATH` env variable  
`cp protoc ~/bin/protoc`  

## run the generator

Work in VSCode inside the `code_generator` directory.  
`cd code_generator`  
`code .`  
Run the generator simply with  
`cargo run`  
It will create a `_.rs` file.  
Copy the source code into the file `src/gauth_migration_proto_mod.rs`.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)

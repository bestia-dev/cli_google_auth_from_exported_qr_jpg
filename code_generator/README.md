# code generator for protobuf

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

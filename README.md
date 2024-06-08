#### rrpc

A rust library that implements RPC. Just a project to help me learn rust.

![Untitled-2024-02-03-1338](https://github.com/InfiniteVerma/rrpc/assets/45547198/52d11371-482c-4d4c-8d77-d03cb7d5b8a4)


#### Plan

V1
 - [x] Base bones (lib + test)
 - [x] Get it running
 - [x] String parsing

V2
 - [x] Pass port as a variable
 - [x] JSON serialization
 - [x] User can choose btw string/json at init time

V3
 - [x] Multiple clients. To speed up, server listens and spawns a short lived thread to execute each request from a pool?
 - [x] If sync, server main thread executes and returns. If async, dispatches a worker thread to do the job

V4
 - [x] Read from a .txt file and write to a .rs file
 - [x] Add test infra to test multiple scenarios
 - [x] Support enum
 - [x] Support struct with basic data types (int and string)

V5
 - [x] Support functions
 - [x] Use json to pack and unpack
 - [x] Test with dummy mains

V6
 - [x] Crate logic cleanup. Have a single crate?
 - [x] Import it and try using it to generate .rs files
 - [ ] Fix client packing

V7
 - [ ] ~~Build shared libraries and link them~~
 - [x] Using above .txt cons, signatures of txt functions will be available to client and definition in server
 - [x] How would a user pass this?
 - [ ] Horizontal and vertical scaling?

V8
 - [x] Example project using the rpc crate

V9
 - [ ] Proper error handling
 - [ ] Refactor. Try to understand more about designing code in rust.

V10
 - [ ] Parse a language instead of simply structs and enums?

V11
 - [ ] Do computation in parts and then aggregate?

#### Design

Overview: Functions that take basic data type/structs/enum as input

Rust translation:
 - Each type will be a type in rust
 - Pack and Unpack trait that all will implement

#### End Goal

 - user imports this crate as a dependency
 - make a .txt file with the specified IDL
 - make a small 'script.rs' to generate code
   - generates all the files
 - user codes their client + server implementation around it

#### References
 - https://doc.rust-lang.org/book/ch20-01-single-threaded.html
 - https://doc.rust-lang.org/book

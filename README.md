#### rrpc

A rust library that implements Remote Procedure Calls. RPC is used in distributed computing to execute a function in a different address space but written as if it's a normal function call.

Just a project to help me learn rust.

![Untitled-2024-02-03-1338](https://github.com/InfiniteVerma/rrpc/assets/45547198/52d11371-482c-4d4c-8d77-d03cb7d5b8a4)

#### Usage

1. Clone the repo

2. Build it to create `rrpc` binary
```
cargo build
```

3. Create your project
```
cargo new <proj>
```

4. Add rrpc as a build-dependency

5. Design your IDL and write it `input.txt` (syntax below)

6. Write build.rs using which call `rrpc` to generate `client_gen.rs` and `server_gen.rs`

7. Use it in your source code

Refer to example/ for a working solution.

#### Syntax

rrpc currently supports enum, struct and functions. Functions support just int and string as params (for now).

Example .txt file:

```
FUNCTION my_func
IN INT var1
IN INT var2
ENDFUNCTION

ENUM test_enum
val1 u32 1
val2 u32 2
ENDENUM

STRUCT structName
INT intvar 
STRING str
ENDSTRUCT
```

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
 - [x] Fix client packing

V7
 - [ ] ~~Build shared libraries and link them~~
 - [x] Using above .txt cons, signatures of txt functions will be available to client and definition in server
 - [x] How would a user pass this?

V8
 - [x] Example project using the rpc crate

V9
 - [ ] Pass enum, structs created in .txt file in the functions
 - [ ] Support synchronous functions
 - [ ] Server should use worker threads for async functions and main thread for sync functions

V10
 - [ ] Proper error handling
 - [ ] Refactor. Try to understand more about designing code in rust.

V11
 - [ ] Horizontal and vertical scaling?
 - [ ] Parse a language instead of simply structs and enums?
 - [ ] Do computation in parts and then aggregate?

#### End Goal

 - user imports this crate as a dependency
 - make a .txt file with the specified IDL
 - make a small 'build.rs' to generate code
   - generates all the files
 - user codes their client + server implementation around it

#### References
 - https://doc.rust-lang.org/book/ch20-01-single-threaded.html
 - https://doc.rust-lang.org/book

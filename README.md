#### rrpc

A rust library that implements RPC. Just a project to help me learn rust.

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

V3.5
 - [ ] Proper error handling
 - [ ] Refactor. Try to understand more about designing code in rust.

V4
 - [ ] Read a .txt file that contains custom functions signatures. Client will call them and in server, user will define them.
       - [ ] Parse comment and enum from .txt into a .rs file that gets imported with both client and server
 - [ ] They should implement pack and unpack trait

V5
 - [ ] Custom functions, signatures of which are available to client and definition in server
 - [ ] How would a user pass this?
 - [ ] Horizontal and vertical scaling?

V6
 - [ ] Parse a language instead of simply structs and enums?

V6
 - [ ] Do computation in parts and then aggregate?

V7
 - [ ] Example project using the rpc crate

#### Design

Overview: Functions that take basic data type/structs/enum as input

Rust translation:
 - Each type will be a type in rust
 - Pack and Unpack trait that all will implement

Example
-------

STRUCT CALC
int operand1 
int operand2
string operator
ENDSTRUCT

-->

trait RPC {
    fn pack(&self) -> String;
    fn unpack(&String) -> &self;
}

struct CALC {
    operand1: int,
    operant2: int,
    operator: string,
}

impl RPC for CALC {

    fn pack(&self) -> String {
        return in string
    }

    fn unpack(&self) -> Self {
        return in struct
    }
}

#### End Goal

client
 - 

server
 - 

#### References
 - https://doc.rust-lang.org/book/ch20-01-single-threaded.html
 - https://doc.rust-lang.org/book

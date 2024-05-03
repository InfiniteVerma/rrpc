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
 - [x] User can choose string/json as an option

V3
 - [ ] Multiple clients. To speed up, server listens and swaps a short lived thread to execute each request from a pool?
 - [ ] Proper error handling

V3.5
 - [ ] Refactor. Try to understand more about designing code in rust.

V4
 - [ ] Custom functions, signatures of which are available to client and definition in server
 - [ ] How would a user pass this?
 - [ ] Horizontal and vertical scaling?

V5
 - [ ] Do computation in parts and then aggregate?

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

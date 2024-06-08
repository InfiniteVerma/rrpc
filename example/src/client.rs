use generated::RpcFunction;
use std::env;

mod generated {
    include!("generated/client_gen.rs");
}

fn main() {
    env::set_var("RUST_LOG", "info");

    let client = generated::Client::new(8000);
    <() as RpcFunction>::my_func(client, 1);
}

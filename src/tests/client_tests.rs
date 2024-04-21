use rrpc::client;

#[test]
fn test_rpc_client() {
    match client::connect() {
        Ok(stream) => {
        }
        Err(err) => {
            panic!("Failed {}", err);
        }
    }
}

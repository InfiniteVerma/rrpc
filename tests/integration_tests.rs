#[cfg(test)]
mod tests {

    use rrpc::{server, client}; #[test]

    #[test] 
    fn test_rpc_integration() {
    
        let _server_handle = std::thread::spawn(|| {
            server::start().unwrap();
        });
    
        std::thread::sleep(std::time::Duration::from_secs(1));
    
    
        let expression = "2+4";
        let resp = client::send(expression).unwrap();

        assert_eq!(resp, "6");
    }
}

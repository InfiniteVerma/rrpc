#[cfg(test)]
mod tests {

    use rrpc::shared::InputType;
    use rrpc::{client::Client, server::Server};

    #[test]
    fn test_rpc_integration() {
        let test_cases = vec![
            ("2+4", "6"),
            ("2+5", "7"),
            ("22+15", "37"),
            // subtract
            ("5-2", "3"),
            // multiply
            ("5*2", "10"),
            // divide
            ("5/2", "2"),
            ("5/0", "Division by zero"),
        ];

        let _server_handle = std::thread::spawn(|| {
            let server = Server::new(InputType::STR, 8080);
            server.start().unwrap();
        });

        std::thread::sleep(std::time::Duration::from_secs(1));

        let client = Client::new(InputType::STR, 8080);

        for (input, expected_output) in test_cases {
            let resp = client.send_sync(input).unwrap();

            assert_eq!(resp, expected_output);
        }
    }
}

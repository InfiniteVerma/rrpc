#[cfg(test)]
mod tests {

    use rrpc::{client, server};
    #[test]

    fn setup_server() {
        let _server_handle = std::thread::spawn(|| {
            server::start().unwrap();
        });

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

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

        setup_server();

        for (input, expected_output) in test_cases {
            let resp = client::send(input).unwrap();

            assert_eq!(resp, expected_output);
        }
    }
}

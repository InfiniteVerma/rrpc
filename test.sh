if [ $# != 0 ]; then
RUST_BACKTRACE=1 cargo test --test generate_tests -- --nocapture
else
RUST_BACKTRACE=1 cargo test --test generate_tests
fi

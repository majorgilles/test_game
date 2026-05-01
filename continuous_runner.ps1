#cargo watch -x check -x "nextest run"
$env:RUST_BACKTRACE = "0"
cargo watch -x "nextest run"

# Spamrurs

A quickly written program to spam Russian Government websites and related websites, use at your own risk, might be illegal depending on the country you are from

Written as a response to https://stop-russian-desinformation.near.page/, these sites suffer from a certain limitation in web browsers which limits the number of active connections to 4.

# How to run

1. Install Rust: https://www.rust-lang.org/tools/install
2. Run `$ cargo  run --release -- (parameters)` in the base directory

# Usage

Just run it, use the -c parameter to control how many requests are sent to site per batch, after each batch, amount of requests and failed requests are printed.

# Todo

Rewrite how requests created to focus on sites still up

# get-req-rust

This Rust project demonstrates how to make a synchronous HTTP GET request using the `reqwest` crate and handle errors using the `error-chain` crate. It fetches data from a remote API and prints the response status, headers, and body.

## Dependencies

- [reqwest](https://docs.rs/reqwest/latest/reqwest/): The `reqwest` crate provides a convenient, higher-level HTTP client for Rust. It allows you to make HTTP requests and work with responses.

- [error-chain](https://docs.rs/error-chain/latest/error_chain/): The `error-chain` crate simplifies error handling in Rust, making it easy to define and convert custom error types.

## Importance

This project showcases how to perform HTTP requests and handle errors efficiently in a Rust application, making it a useful starting point for developers who need to interact with web services.

## Usage

### Installation

1. Ensure you have Rust and Cargo installed on your system. You can install them from [Rust's official website](https://www.rust-lang.org/learn/get-started).

2. Clone the repository using Git:
   ```bash
   git clone https://github.com/codescalper/get-req-rust.git
   ```
3. Navigate to the project directory:

   ```bash
   cd get-req-rust
   ```

4. Build and run the project with Cargo:

   ```bash
   cargo run
   ```

### Examples

1.  **API Data Retrieval**: This project demonstrates how to make an HTTP GET request to a remote API and retrieve data, which can be used as a reference for integrating external data sources into your Rust application.
2.  **Error Handling**: The use of the `error-chain` crate showcases best practices for error handling, which can be helpful for Rust developers looking to manage errors in their projects effectively.

---

If you have any questions or suggestions, feel free to reach out on Twitter: [@mayanks_tw](https://twitter.com/mayanks_tw).

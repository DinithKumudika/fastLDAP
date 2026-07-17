# FastLDAP

A high-performance, in-memory LDAP server built with Rust and async/await (`tokio`). Designed for speed, safety, and RFC 4511 compliance, this solution serves as a highly scalable identity provider, particularly suited for Keycloak federation.

## Features

- **Asynchronous I/O**: Fully built on `tokio` for maximum concurrency.
- **Zero-Copy Parsing**: Utilizes `nom` and `bytes` for streaming BER decoding and encoding.
- **Lock-Free Concurrency**: Uses `DashMap` to provide a blazingly fast in-memory store.
- **LDIF Bootstrapping**: Built-in LDIF parser to quickly seed users and groups at startup.
- **Security Ready**: Built-in support for LDAPS (TLS over port 636) using `tokio-rustls`.
- **Search Capabilities**: Supports complex RFC 4515 LDAP search filters for group and user enumeration.

## Prerequisites

- **Rust Toolchain**: 1.75 or higher.
- **C++ Build Tools**: Required for compiling procedural macros on Windows. Ensure you have the Visual Studio Build Tools with C++ extensions installed.

## Building and Running

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Run the server**:
   ```bash
   cargo run --release
   ```
   By default, the server will load seed data from the `initial_ldif` configuration and bind to port `3893` to accept LDAP connections.

## Keycloak Integration

FastLDAP is specifically designed to handle Keycloak's User Federation. 
To connect Keycloak:
1. Navigate to Keycloak Admin Console -> User Federation -> Add LDAP.
2. Set **Vendor** to `Other`.
3. Set **Connection URL** to `ldap://127.0.0.1:3893` (or `ldaps://...`).
4. Set **Users DN** to `dc=example,dc=com` (matching the seed data).
5. Set **Bind Type** to `simple`.
6. Configure the Bind DN and password to match your administrative account.

## Project Structure

- `src/ber/`: Zero-copy BER streaming decoder and encoder.
- `src/protocol/`: LDAP message structures and response codes.
- `src/store/`: `DashMap`-backed in-memory store and LDIF loader.
- `src/operations/`: Request handlers for Bind, Search, etc.
- `src/filter/`: RFC 4515 compliant filter parser and evaluator.
- `src/server.rs` & `src/connection.rs`: Tokio TCP/TLS connection management.

## Future Roadmap

- [x] Support for LDAP Add, Delete, Modify, and ModifyDN operations.
- [ ] Persistent storage backends (e.g., RocksDB).
- [ ] Advanced SASL authentication (GSSAPI, SCRAM).
- [ ] Strict LDAP schema validation.

## License

This project is licensed under the MIT License.

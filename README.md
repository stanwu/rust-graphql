# Rust GraphQL API Server

This project is a simple GraphQL API server implemented in Rust using the Actix-web framework and Juniper library. The server includes an `echo` query that returns the input message.

## Features
- **GraphQL API**: Built with Juniper for defining and handling GraphQL queries.
- **Actix-web Integration**: Uses Actix-web for handling HTTP requests and routing.
- **Echo Query**: A simple query that echoes back the input message.

## Setup and Run

### Prerequisites
- Rust (latest stable version)
- Cargo (Rust's package manager)

### Steps
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-graphql
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the server:
   ```bash
   cargo run
   ```
4. Access the GraphQL endpoint at:
   ```
   http://127.0.0.1:8080/graphql
   ```

### Example Query
You can test the `echo` query using `curl`:
```bash
curl -X POST http://127.0.0.1:8080/graphql \
-H "Content-Type: application/json" \
-d '{"query": "{ echo(message: \"Hello, world!\") }"}'
```
Expected response:
```json
{
  "data": {
    "echo": "Hello, world!"
  }
}
```

## Development Process Summary

During the development of this project, several challenges were encountered and resolved:

1. **Dependency Version Conflicts**:
   - The `juniper` crate had conflicting versions due to dependencies from `juniper_actix`.
   - Resolved by aligning all dependencies to use `juniper v0.16.1`.

2. **Type Mismatch Issues**:
   - The `post_graphql_handler` function required specific context types.
   - Updated `EmptyMutation` and `EmptySubscription` to use `()` as the context type.

3. **Thread Safety**:
   - Used `Arc` to ensure thread-safe shared ownership of the GraphQL schema.

4. **Error Handling**:
   - Addressed errors related to missing `Cargo.toml` by ensuring the correct working directory.

5. **Detailed Documentation**:
   - Added comprehensive comments to the code for better understanding and maintainability.

## Future Improvements
- Add support for mutations and subscriptions.
- Implement authentication and authorization.
- Enhance error handling and logging.

## License
This project is licensed under the MIT License.
# Connect Database by ODBC

A Rust-based ODBC database client that provides a simple interface for connecting to and querying databases using ODBC.

## Requirements

- Rust 1.56 or later
- ODBC Driver Manager (unixODBC on Linux)
- Appropriate ODBC driver for your target database

## Installation

1. Install the ODBC Driver Manager:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install unixodbc unixodbc-dev
   
   # CentOS/RHEL
   sudo yum install unixODBC unixODBC-devel
   ```

2. Install the appropriate ODBC driver for your database (e.g., MySQL, PostgreSQL, SQL Server)

3. Clone and build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Configure your ODBC connection string in `src/main.rs`:
   ```rust
   let connection_string = "Driver={Your ODBC Driver};Server=your_server;Database=your_db;UID=username;PWD=password";
   ```

2. Run the client:
   ```bash
   cargo run
   ```

## Features

- Simple connection management
- Query execution
- Result set handling
- Error handling with context

## Example

```rust
let client = OdbcClient::new()?;
let connection = client.connect(connection_string)?;
client.execute_query(&connection, "SELECT * FROM your_table")?;
```

## License

MIT

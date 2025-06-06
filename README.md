# Rust PostgreSQL Client

A simple Rust application demonstrating how to connect to a PostgreSQL database to manage key-value data. Designed to work seamlessly with both local and cloud-hosted PostgreSQL instances, enabling easy remote access.

## Features

- Connect to PostgreSQL using the official `postgres` crate
- Execute SQL queries like `INSERT` and `SELECT`
- Manage simple key-value pairs in a database table
- Easily adaptable for cloud deployment and remote usage

## Getting Started

### Prerequisites

- Rust (recommended latest stable)
- PostgreSQL server running locally or on the cloud
- `postgres` crate (included in `Cargo.toml`)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/mishalturkane/rust-postgres.git
   cd rust-postgres

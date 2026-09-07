# Rust URL Shortener

A simple URL shortener API built with Rust, Axum, SQLx, and PostgreSQL.

This project generates a random 8-character short code for a given URL,
stores the mapping in PostgreSQL, and redirects requests using the generated
short code.

## Features

- Generate random 8-character short codes
- Shorten URLs through a JSON API
- URL validation for HTTP/HTTPS URLs
- PostgreSQL persistent storage
- Short-code uniqueness enforced by database constraint
- Redirect to the original URL
- Automated tests for short-code generation

## Tech Stack

- **Rust**
- **Axum** — HTTP web framework
- **Tokio** — asynchronous runtime
- **SQLx** — async SQL toolkit
- **PostgreSQL** — persistent database storage
- **Serde** — JSON serialization and deserialization
- **Rand** — random short-code generation
- **URL** — URL parsing and validation

## How It Works

The application stores URL mappings in PostgreSQL:
```text
short_code → original_url
```

## Setup

### Prerequisites

- Rust
- PostgreSQL
- SQLx CLI

### Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://postgres:PASSWORD@localhost:5432/url_shortener
```

### Database Migration
Run the migrations:
```bash
sqlx migrate run
```
Run
```bash
cargo run
```
The server will start at:
```text
http://127.0.0.1:8080
```


For example:  
NYLBWgPu → https://docs.rs/tokio/latest/tokio/

## Creating a Short URL

A client sends the original URL to:
```bash
POST /post_url
```
with a JSON body:
```json
{
  "url": "https://example.com"
}
```

The server then:

1. Generates a random 8-character short code.  
2. Stores the short code and original URL in the HashMap.  
3. Returns the generated short code.  

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Check whether the server is running |
| POST | `/post_url` | Generate a short code for a URL |
| GET | `/get_url/{short_code}` | Redirect to the original URL |


## Example Workflow
1. Create a short URL
```text
POST http://127.0.0.1:8080/post_url
Content-Type: application/json
```
```json
{
  "url": "https://docs.rs/tokio/latest/tokio/"
}
```
Response:
```json
{
  "short_code": "v2iywVAR"
}
```

2. Use the short code
```text
GET http://127.0.0.1:8080/get_url/v2iywVAR
```
The server redirects to:
```text
https://docs.rs/tokio/latest/tokio/
```

## Storage

The application uses PostgreSQL for persistent URL storage.

SQLx migrations are used to manage the database schema.

The `urls` table contains:

- `id` — primary key
- `short_code` — unique 8-character identifier
- `original_url` — original destination URL

Data persists across application restarts.

> Old version : Data is lost when the application restarts.


# Current Status

The core URL shortening and redirect functionality is implemented.

## Planned Improvements
- Handle non-existent short codes without unwrap() ✅
- Validate submitted URLs ✅
- Handle short-code collisions ✅
- Add more automated tests ✅
- Add persistent database storage✅
- Add a web UI
- Deploy the application
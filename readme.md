# Rust URL Shortener

A simple URL shortener API built with Rust and Axum.

This project generates a random 8-character short code for a given URL,
stores the mapping in memory, and redirects requests using the generated
short code.

## Features

- Generate random 8-character short codes
- Shorten URLs through a JSON API
- In-memory URL storage using `HashMap`
- Shared application state using `Arc<Mutex<_>>`
- Redirect to the original URL
- Basic random short-code testing

## Tech Stack

- **Rust**
- **Axum** — HTTP web framework
- **Tokio** — asynchronous runtime
- **Serde** — JSON serialization and deserialization
- **Rand** — random short-code generation
- **HashMap** — in-memory URL storage
- **Arc + Mutex** — shared mutable application state

## How It Works

The application stores URLs using the following mapping:

```text
short_code → original_url
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
  "short_code": "NYLBWgPu"
}
```

2. Use the short code
```text
GET http://127.0.0.1:8080/get_url/NYLBWgPu
```
The server redirects to:
```text
https://docs.rs/tokio/latest/tokio/
```

## Storage

The application currently uses an in-memory HashMap:

short_code → original_url

The HashMap is shared between handlers using Arc and Mutex.

> Note: Data is lost when the application restarts.


# Current Status

The core URL shortening and redirect functionality is implemented.

## Planned Improvements
- Handle non-existent short codes without unwrap() ✅
- Validate submitted URLs ✅
- Handle short-code collisions ✅
- Add more automated tests ✅
- Add persistent database storage
- Add a web UI
- Deploy the application
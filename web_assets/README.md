# Web Assets

This directory contains web assets (HTML, CSS, etc.) used by the Rust learning examples.

## Chapter 20 Web Server Assets

### `ch20_web_server/`

Contains HTML pages served by the Chapter 20 web server examples. These files demonstrate a complete web server implementation with educational content.

#### Files:

- **`graceful.html`** - Main landing page for the graceful shutdown server
  - Used by: `ch20_03_graceful_shutdown.rs`
  - Route: `http://localhost:7880/`
  - Purpose: Introduces graceful shutdown concepts with interactive navigation

- **`about.html`** - Comprehensive project documentation
  - Used by: All Chapter 20 examples
  - Route: `http://localhost:7880/about`
  - Purpose: Complete timeline of the web server journey from single-threaded to graceful shutdown

- **`shutdown.html`** - Technical deep-dive into graceful shutdown
  - Used by: `ch20_03_graceful_shutdown.rs`
  - Route: `http://localhost:7880/shutdown`
  - Purpose: Detailed explanation of Drop trait implementation and shutdown process

- **`404.html`** - Custom error page
  - Used by: All Chapter 20 examples
  - Route: Any invalid route
  - Purpose: Demonstrates proper HTTP error handling

#### Usage:

```bash
# Run the web servers to see these pages in action:
cargo run --example ch20_01_single_threaded    # http://localhost:7878
cargo run --example ch20_02_multithreaded      # http://localhost:7879  
cargo run --example ch20_03_graceful_shutdown  # http://localhost:7880
```

#### Educational Value:

These HTML files serve multiple learning purposes:

1. **Real Web Server Functionality** - Your Rust code serves actual web pages
2. **Interactive Documentation** - Learn concepts while experiencing them
3. **HTTP Protocol Demonstration** - Shows request/response handling
4. **Static File Serving** - Demonstrates file system integration
5. **Routing Examples** - Shows how web servers handle different URLs

The pages include:
- Styled CSS for professional appearance
- Navigation between different routes
- Technical explanations of Rust concepts
- Code examples and implementation details
- Production considerations and next steps

These assets transform the Chapter 20 examples from simple console applications into fully functional web applications that you can interact with in your browser!
# Chapter 12.6: Writing Error Messages to Standard Error Instead of Standard Output

## Key Takeaways

### Standard Streams
- **stdout**: Standard output for program results
- **stderr**: Standard error for error messages
- **Separation**: Allows redirecting output and errors separately
- **Unix Convention**: Errors go to stderr, results to stdout

### Standard Stream Usage
```rust
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

### eprintln! vs println!
- **println!()**: Writes to stdout
- **eprintln!()**: Writes to stderr
- **Usage**: Error messages should use eprintln!
- **Redirection**: Allows separating output from errors

### Testing Standard Error
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn stderr_output() {
        let mut cmd = Command::cargo_bin("minigrep").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Problem parsing arguments"));
    }
}
```

### Stream Redirection Examples
```bash
# Redirect stdout to file, stderr to terminal
cargo run -- to poem.txt > output.txt

# Redirect both stdout and stderr to file
cargo run -- to poem.txt > output.txt 2>&1

# Redirect stderr to file, stdout to terminal
cargo run -- to poem.txt 2> errors.txt

# Redirect stdout to /dev/null, show only errors
cargo run -- to poem.txt > /dev/null
```

### Manual Stream Writing
```rust
use std::io::{self, Write};

// Write to stdout explicitly
io::stdout().write_all(b"Hello, world!\n").unwrap();

// Write to stderr explicitly
io::stderr().write_all(b"Error occurred!\n").unwrap();

// Buffered writing
let mut stdout = io::stdout();
writeln!(stdout, "Buffered output").unwrap();
stdout.flush().unwrap();
```

### Error Handling Best Practices
- **User-Friendly Messages**: Clear, actionable error descriptions
- **Consistent Format**: Standardized error message format
- **Exit Codes**: Use appropriate exit codes for different errors
- **Logging**: Consider logging for debugging vs user errors

### Program Structure
- **Parse Arguments**: Validate and parse command line arguments
- **Handle Errors**: Use stderr for all error messages
- **Output Results**: Use stdout for program output only
- **Exit Gracefully**: Use appropriate exit codes

### Integration Testing
- **Command Testing**: Test complete program behavior
- **Stream Separation**: Verify stdout and stderr separately
- **Exit Code Testing**: Test that program exits with correct codes
- **Error Message Testing**: Verify error message content

Official Chapter: https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html

---
*Completed: ✓*
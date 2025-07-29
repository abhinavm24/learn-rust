# Chapter 12.5: Working with Environment Variables

## Key Takeaways

### Environment Variable Usage
- **Configuration**: Use env vars for optional configuration
- **Runtime Behavior**: Change program behavior without recompilation
- **Case Sensitivity**: Add case-insensitive search option
- **std::env::var()**: Function to read environment variables

### Environment Variable Implementation
```rust
use std::env;

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

### Case-Insensitive Search
```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}
```

### Updated Run Function
```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
    
    Ok(())
}
```

### Environment Variable Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### Environment Variable Patterns
- **is_ok()**: Check if env var is set (any value)
- **unwrap_or()**: Provide default value if not set
- **parse()**: Convert string values to other types
- **Validation**: Validate env var values for correctness

### Usage Examples
```bash
# Case-sensitive search (default)
cargo run -- searchstring example.txt

# Case-insensitive search
IGNORE_CASE=1 cargo run -- searchstring example.txt

# Setting environment variable
export IGNORE_CASE=1
cargo run -- searchstring example.txt
```

### Best Practices
- **Optional Features**: Use env vars for optional behavior
- **Clear Names**: Use descriptive environment variable names
- **Documentation**: Document expected environment variables
- **Defaults**: Provide sensible default behavior
- **Validation**: Check env var values for correctness

### Cross-Platform Considerations
- **Shell Differences**: Different syntax on Windows/Unix
- **Case Sensitivity**: Environment variable names may be case-sensitive
- **Persistence**: Environment variables persist for session
- **Security**: Avoid sensitive data in environment variables

Official Chapter: https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html

---
*Completed: ✓*
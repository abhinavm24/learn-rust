# Chapter 13.3: Improving Our I/O Project

## Key Takeaways

### Refactoring with Iterators
- **Replace Loops**: Use iterator methods instead of manual loops
- **Functional Style**: More expressive and concise code
- **Error Handling**: Better integration with Result types
- **Performance**: Zero-cost abstractions maintain performance

### Improving Argument Parsing
```rust
// Before: Manual indexing
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

// After: Using iterator
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // Skip program name
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

### Updated Main Function
```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

### Improving Search Function
```rust
// Before: Manual loop
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

// After: Using iterator
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

### Case-Insensitive Search with Iterators
```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

### Benefits of Iterator Refactoring
- **Conciseness**: Fewer lines of code
- **Readability**: More declarative, less imperative
- **Safety**: Less manual index management
- **Composability**: Easy to add more filtering/mapping steps

### Ownership Considerations
```rust
// Taking ownership vs borrowing
impl Config {
    // Takes ownership of iterator
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // Can consume the iterator
    }
}

// Alternative: Working with references
impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &'static str> {
        let mut iter = args.iter().skip(1); // Skip program name
        
        let query = iter.next().ok_or("Missing query")?.clone();
        let file_path = iter.next().ok_or("Missing file path")?.clone();
        
        // ...
    }
}
```

### Error Handling Improvements
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ConfigError {
    message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration error: {}", self.message)
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, ConfigError> {
        args.next(); // Skip program name
        
        let query = args.next().ok_or_else(|| ConfigError {
            message: "Missing query argument".to_string(),
        })?;
        
        let file_path = args.next().ok_or_else(|| ConfigError {
            message: "Missing file path argument".to_string(),
        })?;
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

### Testing Iterator-Based Code
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn config_from_args() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        
        let config = Config::build(args.into_iter()).unwrap();
        
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }
}
```

### Performance Considerations
- **Zero-Cost**: Iterator chains compile to efficient loops
- **Memory**: No intermediate collections unless explicitly collected
- **Lazy Evaluation**: Work only done when needed
- **Optimization**: Compiler can optimize iterator chains effectively

Official Chapter: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html

---
*Completed: ✓*
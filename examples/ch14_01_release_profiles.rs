use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 14.1", "Customizing Builds with Release Profiles");

    println!("🚀 Release Profiles in Cargo");
    println!();
    
    println!("Cargo has two main profiles:");
    println!("• dev profile - used when running `cargo build`");
    println!("• release profile - used when running `cargo build --release`");
    println!();
    
    println!("📊 Profile Settings (in Cargo.toml):");
    println!();
    println!("[profile.dev]");
    println!("opt-level = 0      # No optimizations (fast compile, slow runtime)");
    println!("debug = true       # Include debug symbols");
    println!("panic = 'unwind'   # Stack unwinding on panic");
    println!();
    println!("[profile.release]");
    println!("opt-level = 3      # Maximum optimizations (slow compile, fast runtime)");
    println!("debug = false      # No debug symbols");
    println!("panic = 'unwind'   # Can be changed to 'abort' for smaller binaries");
    println!();
    
    println!("🔧 Custom Profile Example:");
    println!();
    println!("[profile.dev]");
    println!("opt-level = 1      # Light optimization for faster debug builds");
    println!();
    println!("[profile.release]");
    println!("lto = true         # Link Time Optimization");
    println!("codegen-units = 1  # Better optimization at cost of compile time");
    println!("panic = 'abort'    # Smaller binary size");
    println!();
    
    #[cfg(debug_assertions)]
    println!("✅ This binary was built with the DEV profile (debug_assertions enabled)");
    
    #[cfg(not(debug_assertions))]
    println!("🚀 This binary was built with the RELEASE profile (debug_assertions disabled)");
    
    println!();
    println!("💡 Key Takeaways:");
    println!("• dev profile prioritizes fast compilation");
    println!("• release profile prioritizes runtime performance");
    println!("• You can customize both profiles in Cargo.toml");
    println!("• Use conditional compilation (#[cfg]) to adapt to different profiles");
    
    demonstrate_performance_difference();
}

fn demonstrate_performance_difference() {
    println!();
    println!("⚡ Performance Demonstration:");
    
    let start = std::time::Instant::now();
    
    // Simple computation that benefits from optimization
    let mut sum = 0u64;
    for i in 0..1_000_000 {
        sum += i * i;
    }
    
    let duration = start.elapsed();
    
    println!("Computed sum of squares: {}", sum);
    println!("Time taken: {:?}", duration);
    
    #[cfg(debug_assertions)]
    println!("💡 Try running with --release to see the performance difference!");
    
    #[cfg(not(debug_assertions))]
    println!("🚀 This is running with optimizations enabled!");
}
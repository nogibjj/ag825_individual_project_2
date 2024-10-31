fn main() {
    println!("cargo:rustc-link-lib=dylib=sqlite3"); // Links to the SQLite library dynamically
    // Optional: Add path to SQLite if it's not in a standard location
    // println!("cargo:rustc-link-search=native=/path/to/sqlite");
}
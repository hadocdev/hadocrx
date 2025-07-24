fn main() {
    println!("cargo:rustc-link-lib=dylib=hadocrx");
    println!("cargo:rustc-link-search=native=./target/debug");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    println!("cargo:rustc-link-arg=-Wl,-z,origin");
}

fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-lSystem");
}

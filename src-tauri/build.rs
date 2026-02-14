fn main() {
    #[cfg(target_os = "macos")]
    {
        
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    println!("cargo:rustc-link-lib=c++_shared");

    tauri_build::build();
}
fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++"); // macOS uses libc++

    #[cfg(any(target_os = "linux", target_os = "android"))]
    println!("cargo:rustc-link-lib=c++_shared"); // Linux/Android

    tauri_build::build();
}

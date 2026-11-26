# Breve

Breve is a local-first AI chat application built with Rust and Dioxus. It gives you a cross-platform conversational interface for creating, managing, and continuing chats while keeping data on-device in SQLite.

The project is designed around a simple idea: run models locally, keep conversations private, and make the experience feel fast and familiar across desktop and mobile targets.

## Why Breve?

- Run LLMs locally without requiring an internet connection
- Estimate usable context based on device resources
- Manage conversations with local persistence
- Keep the app fast and privacy-friendly through a Rust backend
- Build once and target multiple platforms with Dioxus

## Features

- Local model execution and inference configuration
- Conversation creation, editing, and deletion
- Persistent chat history stored in SQLite
- Cross-platform UI built with Dioxus
- Device-aware model and context handling
- Fast local development workflow with hot reloading

## Tech Stack

- Rust
- Dioxus for the UI
- SQLite for local storage
- Rust-backed service layer for configuration, inference, and conversation management

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/cli)

### Install

```bash
git clone https://github.com/divy-sh/breve.git
cd breve
```

### Run the app

On macOS:

```bash
dx serve --macos
```

You can also use the Dioxus CLI for other targets depending on your platform and setup.

## Android release (Google Play)

### Prerequisites

- Rust stable and Dioxus CLI 0.7.10, matching the Dioxus version resolved in `Cargo.lock`:

```bash
cargo install dioxus-cli --version 0.7.10 --locked
```

- Android Studio with Android SDK Platform 36, Android SDK Build-Tools, Android SDK Command-line Tools, NDK (Side by side), and CMake installed
- Java 17 (Android Studio's bundled runtime works)
- The Rust Android targets:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

On macOS, set the Android environment variables in the shell used to build (adjust the NDK version to the one installed by Android Studio):

```bash
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/<installed-ndk-version>"
export PATH="$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin"
```

### Build the Play Store bundle

From the repository root, build a release Android App Bundle:

```bash
dx bundle --platform android --release --package-types aab --out-dir dist/android
```

The Android App Bundle will be written to `dist/android/Breve-aarch64-linux-android.aab`. The app's Android application ID is `com.breve.ai`; it is configured in `Dioxus.toml`. Keep this ID stable after publishing, because Google Play treats a different ID as a different app.

### Sign the bundle with your upload key

Dioxus produces an unsigned release bundle, so signing it is a separate required step. Do not add your keystore or passwords to the repository. Keep the keystore backed up securely and outside version control. To sign the generated bundle, use the alias in your existing keystore; `jarsigner` prompts for passwords if they are not supplied:

```bash
jarsigner -keystore /secure/path/to/upload-key.jks \
  -signedjar dist/android/Breve-release.aab \
  dist/android/Breve-aarch64-linux-android.aab YOUR_KEY_ALIAS
```

Verify the signed output before uploading:

```bash
jarsigner -verify -verbose -certs dist/android/Breve-release.aab
```

Upload `Breve-release.aab` in the Google Play Console. For a new Play app, enroll in Play App Signing and use the key designated as the **upload key** to sign uploads; Google Play manages the app-signing key used for delivery. For an existing app, use its registered upload key. Do not use `apksigner` to sign an `.aab`; it is for APK files.

For a locally installable APK instead of a Play upload, build with `--package-types apk`. APK signing and installation are separate from the Play App Bundle flow above.

## Project Structure

```text
breve/
├── src/
│   ├── core/         # configuration, inference, conversation, and storage logic
│   ├── ui/           # Dioxus components and panels
│   ├── types/        # shared type definitions
│   └── main.rs       # application entry point
├── assets/           # static frontend assets
├── Cargo.toml        # Rust project configuration
├── Dioxus.toml       # Dioxus application configuration
├── README.md
├── tailwind.css
└── privacy_policy/
```

## Development Notes

- The app keeps conversation and settings data local instead of sending it to a remote backend.
- Model availability and backend setup are handled through the configuration and inference layers in `src/core`.
- The UI is organized into reusable Dioxus components and app panels under `src/ui`.

## Contributing

Contributions are welcome. If you want to improve the app, please open an issue or submit a pull request with a clear description of the change.

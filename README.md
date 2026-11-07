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

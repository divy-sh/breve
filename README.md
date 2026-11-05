# Breve

Breve is a cross-platform application built using DIoxus. It provides a conversational interface that allows users to create, manage, and continue conversations, storing all data locally using SQLite through a Rust backend.

## Features

- Run LLMs locally on any device, without internet connection
- Calculate maximum context length and query length based on Device memory
- Create, update, and delete conversations
- Save conversations locally in SQLite
- Rust backend for secure, high-performance operations
- Hot-reloading and easy development experience

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Dioxus](https://dioxuslabs.com)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/divy-sh/breve.git
   cd breve
   ```

2. Run using:
   ```sh
   dx serve --macos
   ```

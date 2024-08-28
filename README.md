# RsGen
A Rust Generative AI Co-pilot

## Development Environment

To streamline the development and testing of the Rust CLI tool, you can use a setup that involves `cargo watch` for automatic recompilation and a separate terminal for executing the latest build. This setup ensures efficient iteration without the need for repetitive installation steps.

### Setup Instructions

1. **Run `cargo watch` for Automatic Recompilation**:
   In your first terminal, start `cargo watch` to monitor the project for any changes and automatically recompile the code:
   ```bash
   cargo watch -x build
   ```
   This command will continuously rebuild the project whenever you modify the source files.

2. **Execute the Latest Build**:
   In a second terminal, you can run the compiled binary using `cargo run`:
   ```bash
   cargo run -- <arguments>
   ```
   This command will execute the most recent build. Since `cargo watch` is already handling the recompilation, `cargo run` will always run the latest version of the CLI tool.

This setup allows for efficient development cycles, reducing the need to manually reinstall the CLI tool after every change.

---

## Installation

To install `rsgen` so that you can run it from anywhere on your system, follow these steps:

1. **Build and Install**:
   In your project directory, use the following command to install `rsgen` globally:
   ```bash
   cargo install --path .
   ```
   This command compiles and installs the binary in Cargo's bin directory (typically `~/.cargo/bin`), making it accessible from anywhere on your system.

2. **Add to PATH**:
   Ensure that Cargo's bin directory is in your system's `PATH` environment variable. If it's not already, you can add it by placing the following line in your shell's configuration file (e.g., `~/.bashrc`):
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   ```
   After adding this line, reload your shell configuration by running:
   ```bash
   source ~/.bashrc
   ```

Once installed, you can run `rsgen` from anywhere in your terminal.

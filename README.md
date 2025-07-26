# 🔗 Rust + C Interoperability Demo

This is a **demo project** showcasing how to create a **single executable** using code written in **two different languages**: **Rust** and **C**.

The project demonstrates how **Rust can call a C function** using FFI (Foreign Function Interface), a powerful feature of Rust that enables seamless integration with low-level system libraries.

---

## 📦 Build Instructions

Make sure you have Rust and Cargo installed:  
👉 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

multi_lang_demo/
├── Cargo.toml       # Project metadata and dependencies
├── build.rs         # Compiles C code into a static library
├── sum.c            # C source file containing `sum` function
└── src/
    └── main.rs      # Rust main file calling C function


To build and run:

```bash
cargo build
cargo run



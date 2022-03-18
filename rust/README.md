# JSON API Benchmark: Rust

## Installing Rust
If Rust not already installed on your system, download `Rustup`, which is a Rust installer and version management tool.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Otherwise, make sure it is up-to-date.
```
rustup update
```

For additional information about Rust and Cargo, see the [Get Started](https://www.rust-lang.org/learn/get-started) guide.

## Running the benchmark
Compile an optimized version of the code and run the server
```
cd [PROJECT ROOT]/rust
cargo run --release
```

In a separate console window, run the benchmarking tool. For example
```
wrk -t3 -c100 -d30s http://localhost:8080/species
```

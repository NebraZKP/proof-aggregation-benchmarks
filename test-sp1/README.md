# SP1 Aggregation Benchmark

Simulates the aggregation of $N$ Groth16 proofs using the SP1 zkVM. The guest code simply reads a single Groth16 proofs from the host and repeatedly verifies this proof $N$ times, asserting its validity each time. No public values are committed. This is not a fully functional proof aggregation program, but will perform approximately the same work as one.

Currently this benchmark uses no cryptographic precompiles to enable a fair comparison with other zkVMs. We note that cryptographic precompiles will significantly improve the performance.

## Instructions
First follow the SP1 [installation instructions](https://docs.succinct.xyz/getting-started/install.html).

From the `script` directory, build the project with `cargo build --release`. Then, run the benchmark for a given batch size with
```sh
RUST_LOG=info ./target/release/prove --n <batch-size>
```

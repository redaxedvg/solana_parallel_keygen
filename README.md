# Solana Parallel Keypair Generator

This repository contains a high-performance Solana keypair generator that leverages parallel processing for faster keypair generation. The program is designed to quickly generate keypairs that match a desired prefix by utilizing parallel execution, which speeds up the process significantly compared to sequential generation.

## Features

- **Parallel Processing**: Generates keypairs in parallel using the Rayon library for optimized performance.
- **Prefix Matching**: Generate a Solana keypair where the public key starts with a desired prefix.
- **Efficient Batch Generation**: Processes keypairs in batches of 100,000 to maximize throughput.
- **Customizable Prefix**: Easily change the prefix to any string to generate keypairs that start with your desired sequence.

## Requirements

- **Rust**: The program is written in Rust. Make sure you have the latest version of Rust installed.
- **Dependencies**:
  - `solana_sdk` for Solana keypair generation.
  - `base58` for encoding the public key in Base58.
  - `rayon` for parallel processing.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/redaxedvg/solana-parallel-keygen.git
   cd solana-parallel-keygen
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the program:
   ```bash
   cargo run
   ```

## Usage

To generate keypairs that match a specific prefix, modify the `prefix` variable in the code. By default, the program is set to look for keypairs that start with `"desiredPrefix"`.

### Steps to Modify the Prefix:
1. Open `src/main.rs`.
2. Locate the line with the `prefix` variable:
   ```rust
   let prefix = "desiredPrefix";
   ```
3. Replace `"desiredPrefix"` with your desired prefix string, for example:
   ```rust
   let prefix = "CustomPrefix";
   ```
4. Save the file and run the program again.

### Example Output:

When the program finds a keypair that matches the prefix, it will display:

```text
Found matching public key: CustomPrefix1234567890
Private key: 5J7xv9dG...
```

## How Parallel Generation Works

The program generates keypairs in parallel using the Rayon library, which divides the work across multiple threads for faster execution. Instead of generating keypairs one by one, the keypair generation process is split into batches of 100,000 keypairs, and each batch is processed in parallel. This results in a significant speedup, especially when looking for a specific prefix.

The program will continue to generate batches of keypairs until one of them matches the desired prefix. It prints progress every 100,000 keypairs, so you can track how much work has been done and how much time has passed.

### Performance Considerations

- The speed of keypair generation depends on your system's hardware. For multi-core CPUs, this parallel approach can speed up the search time dramatically compared to a sequential search.
- The more cores your system has, the faster the process will be, as Rayon will distribute the tasks accordingly.
- Be sure to monitor your system’s performance (CPU usage) to ensure that it isn't overloaded.

## Example Use Case

### Searching for `CustomPrefix`:
To search for keypairs that start with `CustomPrefix`, modify the code as follows:

```rust
let prefix = "CustomPrefix";
```

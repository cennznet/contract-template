# Contract Template
A minimal contract intended as a template for new projects.  
```bash
> tree
.
├── Cargo.toml
├── README.md
├── build.sh
└── src
    └── lib.rs
```

## Usage
```bash
# Setup `cargo generate` command
cargo install cargo-generate

# Create a new project from this template
cargo generate --git https://github.com/cennznet/contract-template --name my-new-contract
```

__Build__  
```bash
./build.sh 
# outputs:
# - wasm binary: `./target/my-new-contract.wasm`
# - JSON ABI:    `./target/my-new-contract.json`
```

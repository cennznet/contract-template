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

### Build
```bash
./build.sh 
# outputs:
# - wasm binary: `./target/my-new-contract.wasm`
# - JSON ABI:    `./target/my-new-contract.json`
```

### Deploy
install [@cennznet/cli](https://github.com/cennznet/cli) if not.

```bash
# sync latest scripts
$ cennz-cli script:update

$ cennz-cli script:run [OPTIONS] contract-deploy PATH_TO_WASM ACCOUNT

OPTIONS
  -c, --endpoint=endpoint  [default: ws://localhost:9944] cennznet node endpoint
  -f, --path=path          [default: /Users/moge/.cennz_cli/wallet.json] path to wallet.json
  -p, --passphrase         if a passphrase is needed
  --noApi                  pass true if the script doesn't need to connect to the network

# outputs:
# contract code hash: 0x...
# ExtrinsicSuccess
```

### Calculate code hash off-chain
```bash
$ cennz-cli script:run --noApi contract-hashcode PATH_TO_WASM
```

### Instantiate
```bash
# sync latest scripts
$ cennz-cli script:update

$ cennz-cli script:run [OPTIONS] contract-instantiate ACCOUND_ID CODE_HASH PATH_TO_ABI ENDOWMENT GAS_LIMIT

OPTIONS
  -c, --endpoint=endpoint  [default: ws://localhost:9944] cennznet node endpoint
  -f, --path=path          [default: /Users/moge/.cennz_cli/wallet.json] path to wallet.json
  -p, --passphrase         if a passphrase is needed
  --noApi                  pass true if the script doesn't need to connect to the network

# outputs:
# contract addr: 5H39JxdvKhUPyV7p48gFzSsCSMVnZfXZmhsfgHy6AjpMFyUw
# or ExtrinsicFail
```

### Calculate contract address off-chain
```bash
$ cennz-cli script:run --noApi contract-addr OWNER_ACCOUNT CODE_HASH PATH_TO_ABI
```

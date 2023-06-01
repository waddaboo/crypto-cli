# Crypto CLI

A Rust CLI tool to generate RSA key pair, encrypt and decrypt message.

## Features

- Generate private and public RSA key pairs.
- Encrypt message using public key with the option of hex encoding.
- Decrypt message using private key with the option of hex decoding.

## Getting Started

1. Run the line below to get more information on each command.

```
cargo run -- --help
```

2. Run a specific command.

```
cargo run -- <COMMAND>
```

## Commands

1. You can get more specific information about any commands by using `--help` with a specific command. For example, you can run any of the following commands for varying level of detail:
   - ```
     cargo run -- --help
     ```
   - ```
     cargo run -- encrypt --help
     ```
   - ```
     cargo run -- decrypt --help
     ```
2. More information on shorthand commands like `--text` (shortcut `-t`) can be found in `--help`.
3. Currently available commands:
   - ```
     cargo run -- generate
     ```
   - ```
     cargo run -- encrypt --text <TEXT> --public_exponent <PUBLIC_EXPONENT> --modulus <MODULUS>
     ```
   - ```
     cargo run -- decrypt --text <TEXT> --private_exponent <PRIVATE_EXPONENT> --public_exponent <PUBLIC_EXPONENT> --modulus <MODULUS>
     ```
   - ```
     cargo run -- encrypt-encode --text <TEXT> --public_exponent <PUBLIC_EXPONENT> --modulus <MODULUS>
     ```
   - ```
     cargo run -- decrypt-decode --text <TEXT> --private_exponent <PRIVATE_EXPONENT> --public_exponent <PUBLIC_EXPONENT> --modulus <MODULUS>
     ```
4. Test commands for testing RSA encryption decryption cases:

```
cargo test
```

hal-elements -- an Elements extension of hal
============================================

An extension of [hal](https://github.com/stevenroose/hal).

# Installation

```
$ cargo install --locked hal hal-elements
```


# Summary of commands:

- address
    - create: create addresses
    - inspect: inspect addresses

- block
	- create: create a binary block from JSON
	- decode: decode a binary block to JSON

- tx
	- create: create a binary transaction from JSON
	- decode: decode a transaction to JSON

# Examples

```bash
# Create addresses from public key
hal-elements elements address create --pubkey <pubkey-hex> --elementsregtest

# Inspect an address
hal-elements elements address inspect <address>

# Decode a transaction (JSON output)
hal-elements elements tx decode <tx-hex>

# Decode a transaction (YAML output)
hal-elements elements tx decode --yaml <tx-hex>

# Decode a block
hal-elements elements block decode <block-hex>
```


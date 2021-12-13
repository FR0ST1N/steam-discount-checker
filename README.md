# steam-discount-checker
Steam discount checker for CLI.

## Install
```bash
cargo install --git https://github.com/FR0ST1N/steam-discount-checker.git
```

## Usage

### Check
```bash
steamdc
```

### Add
Add games to the list to check.
`steamdc add id`
`id` can be found in the url of the steam game.
```bash
steamdc add 524220 638970 1091500
```
The above command will add `524220`, `638970` and `1091500` to the list.

### Remove
`steamdc remove id`
```bash
steamdc remove 638970 1091500
```

## License
- [MIT](LICENSE)

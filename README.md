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
Remove games from the list.
`steamdc remove id`
```bash
steamdc remove 638970 1091500
```

### List
List all games from the list.
```bash
steamdc list
```

## License
- [MIT](LICENSE)

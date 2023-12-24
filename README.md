<h1 align="center">Decim</h1>
<p align="center">
    <img src="assets/logo.png" alt="Poorly drawn decim" />
</p>

<h3>
Decim is a simple game manager written in Rust.
It allows you to manage your game, store information about them,
manage backups and launch them with a single command.
</h3>


## Installation

### From source

```bash
git clone
cd decim
cargo install --path .
```

## Usage

```bash
# Get help
decim --help

# Create a new game
decim create <name>

# Edit a game
decim edit <name>

# Delete a game
decim delete <name>

# List all games
decim list

# Launch a game
decim launch <name>

# Backup a game
decim backup <name>

# Backup all games
decim backup-all

# Search for a game
decim search <name>

# Search by tags (multiple tags)
decim search-tags <tag1> <tag2> <tag3>
```
### TODO :
- [ ] Auto-completion of commands
- [X] Remove cli because it's useless
- [ ] Add restore for backups
- [X] Add version history when editing a game
- [ ] Write some tests
- [X] Improve search with levensthein distance
- [X] Add sub args to create command

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
Decim is licensed under the [MIT](https://choosealicense.com/licenses/mit/) license.
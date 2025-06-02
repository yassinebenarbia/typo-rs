<h1 align="center">Typo-rs</h1>

Fix your typos by repetition.
> [!NOTE]  
> This is still a very WIP.

# Showcase
||
|------|
|![ShowCase](./assets/typo-rs.png "showcase")|

# Usage
Clone this repo, `cd` to it's directory then 
```bash
cargo install --path ./
```
after that you can run typo-rs and provide config path as an optional argument or make sure that the config is in the directory you are running it from
```bash
typo-rs # option config.toml path
```

# Config
Check [config.example.toml](./config.example.toml)

> [!IMPORTANT]  
> Don't forget to add an extra space at the end of each line in your `paragraph`s

# Keybinds
|key|behavior|
|---|--------|
|`Tab`|cycle to next paragraph|
|`Shift + Tab`|cycle to previous paragraph|

# TODOs
- [ ] Show typing stats
    - [ ] Calculate WPM
    - [ ] Calculate accuracy
    - [ ] Show learned words
    - [ ] Show typing accuracy over time (days/weeks/etc)
- [ ] Add a time limit
___
> **_NOTE:_** Huge thanks to [museun](https://github.com/museun) for making this possible

# Taguchi table generator

In Rust

Taguchi tables, or orthogonal arrays, are a way of efficiently running experiments with many
combinations of multiple variables.

Some light reading
[here](https://www.me.psu.edu/cimbala/me345/Lectures/Taguchi_orthogonal_arrays.pdf) for you.

I originally found about this method from the excellent NightHawkInLight on YouTube, specifically
[this video](https://www.youtube.com/watch?v=5oULEuOoRd0).

## Setup

NodeJS. I used [`nvm`](https://github.com/nvm-sh/nvm#install--update-script).

```bash
# NodeJS (I'm using `nvm`)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.zshrc
cargo install wasm-pack
```

## Running web server

```bash
just serve
```

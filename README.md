# onport

[![CI](https://github.com/lucasnevespereira/onport/actions/workflows/ci.yml/badge.svg?style=flat-square)](https://github.com/lucasnevespereira/onport/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/onport-cli?style=flat-square)](https://crates.io/crates/onport-cli)
[![License](https://img.shields.io/crates/l/onport-cli?style=flat-square)](LICENSE)

A fast port inspector for developers. See what's running, kill what's stuck.

## Install

```bash
cargo install onport-cli
```

## Usage

```bash
onport              # list all listening ports
onport 3000         # what's on port 3000?
onport 3000 -k      # kill the process on port 3000
```

### Alias

Add to your shell config for a shorter command:

```bash
alias op="onport"
```

Then:

```bash
op 3000             # what's on port 3000?
op 3000 -k          # kill it
```

## Example output

```
PORT   PID    PROCESS     USER     UPTIME   CPU    MEM
3000   12847  node        lucas    2h 13m   1.2%   84MB
5432   491    postgres    lucas    3d 4h    0.1%   32MB
8080   13102  java        lucas    12m      4.8%   512MB
```

## License

[MIT](LICENSE)

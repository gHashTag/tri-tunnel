# tri-tunnel — Quick Start

## Install

```bash
git clone https://github.com/gHashTag/tri-tunnel.git
cd tri-tunnel
cargo install --path .
```

## Basic Commands

```bash
# Start
tri-tunnel start

# Status
tri-tunnel status

# Stop
tri-tunnel stop

# Open in browser
tri-tunnel open
```

## URLs After Start

```
Main:    https://<device>.ts.net:443/
Health:  https://<device>.ts.net:443/health
API:     https://<device>.ts.net:443/api/status
```

## Test Results

All 9 tests passed ✅

See [TESTING.md](TESTING.md) for details.

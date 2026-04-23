# tri-tunnel

**Tailscale Funnel management for trios-server**

One command to make your trios-server accessible from anywhere via Tailscale.

## Installation

```bash
git clone https://github.com/gHashTag/tri-tunnel.git
cd tri-tunnel
cargo install --path .
```

## Usage

```bash
# Start funnel (trios-server must be running on port 9005)
tri-tunnel start

# Show status
tri-tunnel status

# Stop funnel
tri-tunnel stop

# Open dashboard in browser
tri-tunnel open
```

## Requirements

- Tailscale installed from [App Store](https://apps.apple.com/app/tailscale/id1475387142)
- Tailscale logged in and connected

## Example

```bash
$ tri-tunnel start

╔═══════════════════════════════════════════════════════════════╗
║     tri-tunnel — Tailscale Funnel              
║                 for trios-server ║
╚═══════════════════════════════════════════════════════════════╝

⠋ Starting Tailscale Funnel...

✅ Funnel started successfully!

┌─────────────────────────────────────────────────────────────┐
│  STATUS                    │
├─────────────────────────────────────────────────────────────┤
│  Device:  playra's MacBook Pro                             │
│  Funnel:  ACTIVE ✅                                          │
│  URL:     playras-macbook-pro-1.tail01804b.ts.net          │
└─────────────────────────────────────────────────────────────┘

🌐 Your trios-server is accessible at: https://playras-macbook-pro-1.tail01804b.ts.net/
🌐 Health check: https://playras-macbook-pro-1.tail01804b.ts.net/health
🌐 API status: https://playras-macbook-pro-1.tail01804b.ts.net/api/status
```

## License

MIT

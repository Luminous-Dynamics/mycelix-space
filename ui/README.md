# Mycelix Space Traffic Control Dashboard

A real-time space traffic control interface built with SvelteKit and CesiumJS, connected to a Holochain-based decentralized backend.

## Features

- **Live Orbital Map**: 3D globe visualization of tracked objects using CesiumJS
- **Conjunction Monitoring**: Real-time collision warnings with risk assessment
- **Object Tracking**: Search and browse all tracked orbital objects
- **Debris Bounties**: Crowdfunded bounties for debris removal and tracking

## Tech Stack

- **Frontend**: SvelteKit 2.x, TypeScript
- **3D Visualization**: CesiumJS for globe and satellite rendering
- **Backend**: Holochain (via @holochain/client)
- **Styling**: Custom CSS with dark theme

## Getting Started

### Prerequisites

- Node.js 18+
- npm or pnpm
- Holochain conductor running with mycelix-space hApp installed

### Installation

```bash
cd ui
npm install
```

### Development

```bash
npm run dev
```

The dashboard will be available at `http://localhost:5173`.

### Build

```bash
npm run build
npm run preview
```

## Project Structure

```
ui/
├── src/
│   ├── lib/
│   │   ├── holochain/      # Holochain client wrapper
│   │   ├── stores/         # Svelte stores for state management
│   │   └── components/     # Reusable UI components
│   └── routes/
│       ├── +layout.svelte  # Main layout with sidebar
│       ├── +page.svelte    # Live map dashboard
│       ├── conjunctions/   # Conjunction monitoring
│       ├── objects/        # Object browser
│       └── bounties/       # Debris bounties
├── static/
│   └── cesium/            # CesiumJS assets (copy from node_modules)
└── package.json
```

## Configuration

### Holochain Connection

By default, the dashboard connects to the Holochain conductor at `ws://localhost:8888`. To change this, modify the `DEFAULT_CONDUCTOR_URL` in `src/lib/holochain/client.ts`.

### CesiumJS Token

For production use, get your own Cesium Ion access token at [cesium.com](https://cesium.com/ion/signup) and update it in `src/routes/+page.svelte`.

## Features Roadmap

- [x] Dashboard layout with navigation
- [x] Live orbital map with CesiumJS
- [x] Conjunction monitoring page
- [x] Object browser with search
- [x] Debris bounties page
- [ ] Real Holochain integration (currently using mock data)
- [ ] TLE propagation for satellite positions
- [ ] Real-time WebSocket updates
- [ ] User authentication
- [ ] CDM export functionality

## License

MIT

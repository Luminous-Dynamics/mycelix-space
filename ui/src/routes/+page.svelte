<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let cesiumContainer: HTMLDivElement;
	let viewer: any;
	let objectCount = 0;
	let activeConjunctions = 0;
	let trackedDebris = 0;
	let networkPeers = 0;

	// Mock data for demo - will be replaced with Holochain queries
	const mockStats = {
		objects: 12847,
		conjunctions: 23,
		debris: 4521,
		peers: 7
	};

	onMount(async () => {
		if (!browser) return;

		// Update stats (mock for now)
		objectCount = mockStats.objects;
		activeConjunctions = mockStats.conjunctions;
		trackedDebris = mockStats.debris;
		networkPeers = mockStats.peers;

		// Dynamically import Cesium (client-side only)
		const Cesium = await import('cesium');

		// Configure Cesium Ion token (use demo token or get your own)
		Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJlYWE1OWUxNy1mMWZiLTQzYjYtYTQ0OS1kMWFjYmFkNjc5YzciLCJpZCI6NTc4Miwic2NvcGVzIjpbImFzciIsImdjIl0sImlhdCI6MTU0MzIzNjgwOH0.xMxJLVwHftcblRjFTHI6hM2q7pLz8Yfe36pohquFuKQ';

		viewer = new Cesium.Viewer(cesiumContainer, {
			terrainProvider: await Cesium.createWorldTerrainAsync(),
			baseLayerPicker: false,
			geocoder: false,
			homeButton: false,
			sceneModePicker: false,
			navigationHelpButton: false,
			animation: true,
			timeline: true,
			fullscreenButton: false,
			vrButton: false,
			infoBox: true,
			selectionIndicator: true,
			shadows: false,
			shouldAnimate: true
		});

		// Set initial camera to show Earth from space
		viewer.camera.setView({
			destination: Cesium.Cartesian3.fromDegrees(-98.0, 30.0, 25000000),
			orientation: {
				heading: 0.0,
				pitch: -Cesium.Math.PI_OVER_TWO,
				roll: 0.0
			}
		});

		// Add some demo satellites
		addDemoSatellites(Cesium);

		return () => {
			if (viewer) {
				viewer.destroy();
			}
		};
	});

	function addDemoSatellites(Cesium: any) {
		// ISS (approximate position)
		viewer.entities.add({
			name: 'ISS (ZARYA)',
			position: Cesium.Cartesian3.fromDegrees(-95.4, 29.5, 408000),
			point: {
				pixelSize: 8,
				color: Cesium.Color.LIME,
				outlineColor: Cesium.Color.WHITE,
				outlineWidth: 2
			},
			label: {
				text: 'ISS',
				font: '12px sans-serif',
				fillColor: Cesium.Color.WHITE,
				style: Cesium.LabelStyle.FILL_AND_OUTLINE,
				outlineWidth: 2,
				verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
				pixelOffset: new Cesium.Cartesian2(0, -12)
			}
		});

		// Add some debris markers (demo)
		const debrisPositions = [
			{ lon: 45, lat: 20, alt: 600000, name: 'Debris Field Alpha' },
			{ lon: -120, lat: 45, alt: 800000, name: 'COSMOS 2251 Fragment' },
			{ lon: 80, lat: -30, alt: 700000, name: 'Fengyun-1C Fragment' }
		];

		debrisPositions.forEach(d => {
			viewer.entities.add({
				name: d.name,
				position: Cesium.Cartesian3.fromDegrees(d.lon, d.lat, d.alt),
				point: {
					pixelSize: 5,
					color: Cesium.Color.ORANGE
				}
			});
		});

		// Add conjunction warning marker
		viewer.entities.add({
			name: 'CONJUNCTION WARNING: Starlink-1234 / Debris',
			position: Cesium.Cartesian3.fromDegrees(-45, 35, 550000),
			point: {
				pixelSize: 12,
				color: Cesium.Color.RED,
				outlineColor: Cesium.Color.YELLOW,
				outlineWidth: 3
			},
			label: {
				text: '⚠️ CONJUNCTION',
				font: '14px sans-serif',
				fillColor: Cesium.Color.RED,
				style: Cesium.LabelStyle.FILL_AND_OUTLINE,
				outlineWidth: 2,
				verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
				pixelOffset: new Cesium.Cartesian2(0, -15)
			}
		});
	}
</script>

<div class="dashboard">
	<header class="dashboard-header">
		<h1>Space Traffic Control</h1>
		<div class="header-actions">
			<button class="btn btn-primary">+ Register Object</button>
			<button class="btn">Submit TLE</button>
		</div>
	</header>

	<div class="stats-bar">
		<div class="stat">
			<span class="stat-value">{objectCount.toLocaleString()}</span>
			<span class="stat-label">Tracked Objects</span>
		</div>
		<div class="stat warning">
			<span class="stat-value">{activeConjunctions}</span>
			<span class="stat-label">Active Conjunctions</span>
		</div>
		<div class="stat">
			<span class="stat-value">{trackedDebris.toLocaleString()}</span>
			<span class="stat-label">Debris Pieces</span>
		</div>
		<div class="stat">
			<span class="stat-value">{networkPeers}</span>
			<span class="stat-label">Network Peers</span>
		</div>
	</div>

	<div class="globe-container">
		<div bind:this={cesiumContainer} id="cesiumContainer"></div>

		<div class="globe-overlay">
			<div class="legend">
				<h4>Legend</h4>
				<div class="legend-item">
					<span class="dot active"></span> Active Satellite
				</div>
				<div class="legend-item">
					<span class="dot debris"></span> Tracked Debris
				</div>
				<div class="legend-item">
					<span class="dot warning"></span> Conjunction Warning
				</div>
			</div>
		</div>
	</div>

	<div class="alerts-panel">
		<h3>Recent Alerts</h3>
		<div class="alert-list">
			<div class="alert-item high">
				<span class="alert-time">2 min ago</span>
				<span class="alert-message">HIGH: Conjunction detected - Starlink-1234 / Cosmos Debris</span>
				<span class="risk-badge risk-high">HIGH</span>
			</div>
			<div class="alert-item medium">
				<span class="alert-time">15 min ago</span>
				<span class="alert-message">MEDIUM: Close approach - OneWeb-0456 / Unknown Object</span>
				<span class="risk-badge risk-medium">MEDIUM</span>
			</div>
			<div class="alert-item low">
				<span class="alert-time">1 hr ago</span>
				<span class="alert-message">LOW: Routine screening complete - 847 objects analyzed</span>
				<span class="risk-badge risk-low">LOW</span>
			</div>
		</div>
	</div>
</div>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 1.5rem;
		gap: 1rem;
	}

	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard-header h1 {
		font-size: 1.5rem;
		color: #00d4ff;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.stats-bar {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.stats-bar .stat {
		background: #12121a;
		border: 1px solid #2a2a3a;
		border-radius: 12px;
		padding: 1rem;
		text-align: center;
	}

	.stats-bar .stat.warning .stat-value {
		color: #ff8844;
	}

	.globe-container {
		flex: 1;
		position: relative;
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid #2a2a3a;
		min-height: 400px;
	}

	#cesiumContainer {
		width: 100%;
		height: 100%;
	}

	.globe-overlay {
		position: absolute;
		top: 1rem;
		right: 1rem;
		z-index: 10;
	}

	.legend {
		background: rgba(18, 18, 26, 0.9);
		border: 1px solid #2a2a3a;
		border-radius: 8px;
		padding: 1rem;
	}

	.legend h4 {
		margin: 0 0 0.75rem 0;
		color: #888;
		font-size: 0.75rem;
		text-transform: uppercase;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		color: #ccc;
		margin: 0.5rem 0;
	}

	.dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.dot.active { background: #44ff88; }
	.dot.debris { background: #ff8844; }
	.dot.warning { background: #ff4444; }

	.alerts-panel {
		background: #12121a;
		border: 1px solid #2a2a3a;
		border-radius: 12px;
		padding: 1rem;
	}

	.alerts-panel h3 {
		margin: 0 0 1rem 0;
		font-size: 1rem;
		color: #e0e0e0;
	}

	.alert-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.alert-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		border-radius: 8px;
		background: #1a1a2a;
	}

	.alert-time {
		font-size: 0.75rem;
		color: #666;
		min-width: 80px;
	}

	.alert-message {
		flex: 1;
		font-size: 0.85rem;
	}

	.alert-item.high { border-left: 3px solid #ff4444; }
	.alert-item.medium { border-left: 3px solid #ffcc00; }
	.alert-item.low { border-left: 3px solid #44ff88; }
</style>

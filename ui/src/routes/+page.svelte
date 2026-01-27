<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import {
		propagateTLE,
		generateOrbitalPath,
		SAMPLE_TLES,
		DEBRIS_TLES,
		type TLE,
		type SatellitePosition
	} from '$lib/orbital/propagator';

	let cesiumContainer: HTMLDivElement;
	let viewer: any;
	let Cesium: any;
	let updateInterval: ReturnType<typeof setInterval>;

	// Stats
	let objectCount = 0;
	let activeConjunctions = 0;
	let trackedDebris = 0;
	let networkPeers = 0;

	// Track entities for updates
	let satelliteEntities: Map<number, any> = new Map();

	// All TLEs to track
	const allTles = [...SAMPLE_TLES, ...DEBRIS_TLES];

	onMount(async () => {
		if (!browser) return;

		// Update stats
		objectCount = SAMPLE_TLES.length;
		trackedDebris = DEBRIS_TLES.length;
		activeConjunctions = 2; // Demo value
		networkPeers = 1; // Single node for now

		// Dynamically import Cesium
		Cesium = await import('cesium');

		// Configure Cesium Ion token (demo token - get your own for production)
		Cesium.Ion.defaultAccessToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJlYWE1OWUxNy1mMWZiLTQzYjYtYTQ0OS1kMWFjYmFkNjc5YzciLCJpZCI6NTc4Miwic2NvcGVzIjpbImFzciIsImdjIl0sImlhdCI6MTU0MzIzNjgwOH0.xMxJLVwHftcblRjFTHI6hM2q7pLz8Yfe36pohquFuKQ';

		viewer = new Cesium.Viewer(cesiumContainer, {
			terrainProvider: await Cesium.createWorldTerrainAsync(),
			baseLayerPicker: false,
			geocoder: false,
			homeButton: false,
			sceneModePicker: true,
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

		// Set clock to real-time
		viewer.clock.shouldAnimate = true;
		viewer.clock.multiplier = 1;

		// Set initial camera to show Earth from space
		viewer.camera.setView({
			destination: Cesium.Cartesian3.fromDegrees(-98.0, 30.0, 20000000),
			orientation: {
				heading: 0.0,
				pitch: -Cesium.Math.PI_OVER_TWO,
				roll: 0.0
			}
		});

		// Initialize all satellites
		initializeSatellites();

		// Update positions every second
		updateInterval = setInterval(() => {
			updateSatellitePositions();
		}, 1000);

		return () => {
			if (updateInterval) clearInterval(updateInterval);
			if (viewer) viewer.destroy();
		};
	});

	onDestroy(() => {
		if (updateInterval) clearInterval(updateInterval);
	});

	function initializeSatellites() {
		const now = new Date();

		// Add active satellites
		for (const tle of SAMPLE_TLES) {
			const pos = propagateTLE(tle, now);
			if (!pos) continue;

			// Generate orbital path (90 minutes = ~1 orbit for LEO)
			const path = generateOrbitalPath(tle, now, 90, 2);

			// Create orbit path
			const pathPositions = path.positions.map(p =>
				Cesium.Cartesian3.fromDegrees(p.longitude, p.latitude, p.altitude * 1000)
			);

			// Add orbital path polyline
			viewer.entities.add({
				name: `${tle.name} Orbit`,
				polyline: {
					positions: pathPositions,
					width: 1,
					material: new Cesium.PolylineGlowMaterialProperty({
						glowPower: 0.2,
						color: Cesium.Color.CYAN.withAlpha(0.5)
					})
				}
			});

			// Add satellite point
			const entity = viewer.entities.add({
				id: `sat-${tle.noradId}`,
				name: tle.name,
				position: Cesium.Cartesian3.fromDegrees(pos.longitude, pos.latitude, pos.altitude * 1000),
				point: {
					pixelSize: 8,
					color: Cesium.Color.LIME,
					outlineColor: Cesium.Color.WHITE,
					outlineWidth: 2
				},
				label: {
					text: tle.name,
					font: '12px sans-serif',
					fillColor: Cesium.Color.WHITE,
					style: Cesium.LabelStyle.FILL_AND_OUTLINE,
					outlineWidth: 2,
					verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
					pixelOffset: new Cesium.Cartesian2(0, -12),
					distanceDisplayCondition: new Cesium.DistanceDisplayCondition(0, 15000000)
				},
				description: `
					<h3>${tle.name}</h3>
					<p><b>NORAD ID:</b> ${tle.noradId}</p>
					<p><b>Altitude:</b> ${pos.altitude.toFixed(1)} km</p>
					<p><b>Latitude:</b> ${pos.latitude.toFixed(4)}°</p>
					<p><b>Longitude:</b> ${pos.longitude.toFixed(4)}°</p>
					<p><b>Type:</b> Active Satellite</p>
				`
			});

			satelliteEntities.set(tle.noradId, entity);
		}

		// Add debris
		for (const tle of DEBRIS_TLES) {
			const pos = propagateTLE(tle, now);
			if (!pos) continue;

			const entity = viewer.entities.add({
				id: `debris-${tle.noradId}`,
				name: tle.name,
				position: Cesium.Cartesian3.fromDegrees(pos.longitude, pos.latitude, pos.altitude * 1000),
				point: {
					pixelSize: 5,
					color: Cesium.Color.ORANGE,
					outlineColor: Cesium.Color.RED,
					outlineWidth: 1
				},
				label: {
					text: tle.name,
					font: '10px sans-serif',
					fillColor: Cesium.Color.ORANGE,
					style: Cesium.LabelStyle.FILL_AND_OUTLINE,
					outlineWidth: 1,
					verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
					pixelOffset: new Cesium.Cartesian2(0, -8),
					distanceDisplayCondition: new Cesium.DistanceDisplayCondition(0, 10000000)
				},
				description: `
					<h3>${tle.name}</h3>
					<p><b>NORAD ID:</b> ${tle.noradId}</p>
					<p><b>Altitude:</b> ${pos.altitude.toFixed(1)} km</p>
					<p><b>Type:</b> DEBRIS</p>
					<p style="color: orange;"><b>⚠️ Tracked Debris Object</b></p>
				`
			});

			satelliteEntities.set(tle.noradId, entity);
		}

		// Add conjunction warning marker (demo)
		addConjunctionWarning();
	}

	function updateSatellitePositions() {
		if (!viewer || !Cesium) return;

		const now = new Date();

		for (const tle of allTles) {
			const pos = propagateTLE(tle, now);
			if (!pos) continue;

			const entity = satelliteEntities.get(tle.noradId);
			if (entity) {
				entity.position = Cesium.Cartesian3.fromDegrees(
					pos.longitude,
					pos.latitude,
					pos.altitude * 1000
				);

				// Update description with current data
				const isDebris = DEBRIS_TLES.some(d => d.noradId === tle.noradId);
				entity.description = `
					<h3>${tle.name}</h3>
					<p><b>NORAD ID:</b> ${tle.noradId}</p>
					<p><b>Altitude:</b> ${pos.altitude.toFixed(1)} km</p>
					<p><b>Latitude:</b> ${pos.latitude.toFixed(4)}°</p>
					<p><b>Longitude:</b> ${pos.longitude.toFixed(4)}°</p>
					<p><b>Velocity:</b> ${Math.sqrt(pos.velocity.x**2 + pos.velocity.y**2 + pos.velocity.z**2).toFixed(2)} km/s</p>
					<p><b>Type:</b> ${isDebris ? 'DEBRIS' : 'Active Satellite'}</p>
					<p><i>Updated: ${now.toISOString()}</i></p>
				`;
			}
		}
	}

	function addConjunctionWarning() {
		// Add a demo conjunction warning between two objects
		const primary = SAMPLE_TLES[1]; // Starlink
		const secondary = DEBRIS_TLES[0]; // Cosmos debris

		const primaryPos = propagateTLE(primary, new Date());
		if (!primaryPos) return;

		viewer.entities.add({
			name: `⚠️ CONJUNCTION: ${primary.name} / ${secondary.name}`,
			position: Cesium.Cartesian3.fromDegrees(
				primaryPos.longitude + 5,
				primaryPos.latitude + 5,
				primaryPos.altitude * 1000
			),
			billboard: {
				image: createWarningIcon(),
				width: 32,
				height: 32
			},
			label: {
				text: '⚠️ CONJUNCTION',
				font: 'bold 14px sans-serif',
				fillColor: Cesium.Color.RED,
				style: Cesium.LabelStyle.FILL_AND_OUTLINE,
				outlineWidth: 2,
				outlineColor: Cesium.Color.BLACK,
				verticalOrigin: Cesium.VerticalOrigin.BOTTOM,
				pixelOffset: new Cesium.Cartesian2(0, -20)
			},
			description: `
				<h3 style="color: red;">⚠️ CONJUNCTION WARNING</h3>
				<p><b>Primary:</b> ${primary.name} (${primary.noradId})</p>
				<p><b>Secondary:</b> ${secondary.name} (${secondary.noradId})</p>
				<p><b>TCA:</b> ${new Date(Date.now() + 7200000).toISOString()}</p>
				<p><b>Miss Distance:</b> 0.15 km</p>
				<p><b>Collision Probability:</b> 3.4×10⁻³</p>
				<p style="color: red;"><b>Risk Level: HIGH</b></p>
			`
		});
	}

	function createWarningIcon(): string {
		// Create a simple warning icon as data URL
		const canvas = document.createElement('canvas');
		canvas.width = 32;
		canvas.height = 32;
		const ctx = canvas.getContext('2d')!;

		// Draw warning triangle
		ctx.fillStyle = '#ff4444';
		ctx.beginPath();
		ctx.moveTo(16, 2);
		ctx.lineTo(30, 28);
		ctx.lineTo(2, 28);
		ctx.closePath();
		ctx.fill();

		// Draw exclamation mark
		ctx.fillStyle = '#ffffff';
		ctx.font = 'bold 18px sans-serif';
		ctx.textAlign = 'center';
		ctx.fillText('!', 16, 24);

		return canvas.toDataURL();
	}
</script>

<div class="dashboard">
	<header class="dashboard-header">
		<h1>Space Traffic Control</h1>
		<div class="live-indicator">
			<span class="pulse"></span>
			LIVE - Real-time SGP4 Propagation
		</div>
		<div class="header-actions">
			<button class="btn btn-primary">+ Register Object</button>
			<button class="btn">Submit TLE</button>
		</div>
	</header>

	<div class="stats-bar">
		<div class="stat">
			<span class="stat-value">{objectCount}</span>
			<span class="stat-label">Active Satellites</span>
		</div>
		<div class="stat warning">
			<span class="stat-value">{activeConjunctions}</span>
			<span class="stat-label">Active Conjunctions</span>
		</div>
		<div class="stat debris">
			<span class="stat-value">{trackedDebris}</span>
			<span class="stat-label">Tracked Debris</span>
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
				<div class="legend-item">
					<span class="line orbit"></span> Orbital Path
				</div>
			</div>

			<div class="controls">
				<h4>View Controls</h4>
				<button class="control-btn" on:click={() => viewer?.camera.flyHome()}>
					🌍 Reset View
				</button>
				<button class="control-btn" on:click={() => {
					if (viewer) viewer.clock.multiplier = viewer.clock.multiplier === 1 ? 60 : 1;
				}}>
					⏱️ Toggle Speed
				</button>
			</div>
		</div>
	</div>

	<div class="alerts-panel">
		<h3>Recent Alerts</h3>
		<div class="alert-list">
			<div class="alert-item high">
				<span class="alert-time">2 min ago</span>
				<span class="alert-message">HIGH: Conjunction detected - STARLINK-1007 / COSMOS 2251 DEB</span>
				<span class="risk-badge risk-high">HIGH</span>
			</div>
			<div class="alert-item medium">
				<span class="alert-time">15 min ago</span>
				<span class="alert-message">MEDIUM: Close approach - ISS / FENGYUN 1C DEB</span>
				<span class="risk-badge risk-medium">MEDIUM</span>
			</div>
			<div class="alert-item low">
				<span class="alert-time">1 hr ago</span>
				<span class="alert-message">LOW: Routine screening complete - {objectCount + trackedDebris} objects analyzed</span>
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

	.live-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		color: #44ff88;
		font-weight: 600;
	}

	.pulse {
		width: 8px;
		height: 8px;
		background: #44ff88;
		border-radius: 50%;
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.5; transform: scale(1.2); }
		100% { opacity: 1; transform: scale(1); }
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

	.stats-bar .stat.debris .stat-value {
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
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.legend, .controls {
		background: rgba(18, 18, 26, 0.9);
		border: 1px solid #2a2a3a;
		border-radius: 8px;
		padding: 1rem;
	}

	.legend h4, .controls h4 {
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

	.line {
		width: 20px;
		height: 2px;
	}

	.line.orbit {
		background: linear-gradient(90deg, transparent, #00d4ff, transparent);
	}

	.control-btn {
		display: block;
		width: 100%;
		padding: 0.5rem;
		margin-top: 0.5rem;
		background: #1a1a2a;
		border: 1px solid #2a2a3a;
		border-radius: 6px;
		color: #ccc;
		cursor: pointer;
		font-size: 0.8rem;
	}

	.control-btn:hover {
		background: #2a2a3a;
		color: #fff;
	}

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

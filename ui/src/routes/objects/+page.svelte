<script lang="ts">
	import { objects, objectStats, searchQuery, filteredObjects } from '$lib/stores/objects';

	// Mock data for demo
	const mockObjects = [
		{ norad_id: 25544, name: 'ISS (ZARYA)', object_type: 'PAYLOAD', country_code: 'ISS', launch_date: 915148800000, decay_date: null, rcs_size: 'LARGE' },
		{ norad_id: 48274, name: 'STARLINK-1234', object_type: 'PAYLOAD', country_code: 'US', launch_date: 1609459200000, decay_date: null, rcs_size: 'MEDIUM' },
		{ norad_id: 43205, name: 'COSMOS 2251 DEB', object_type: 'DEBRIS', country_code: 'CIS', launch_date: 1234567890000, decay_date: null, rcs_size: 'SMALL' },
		{ norad_id: 51234, name: 'ONEWEB-0456', object_type: 'PAYLOAD', country_code: 'UK', launch_date: 1625097600000, decay_date: null, rcs_size: 'MEDIUM' },
		{ norad_id: 99999, name: 'FENGYUN 1C DEB', object_type: 'DEBRIS', country_code: 'PRC', launch_date: 1167609600000, decay_date: null, rcs_size: 'SMALL' },
		{ norad_id: 55123, name: 'ASTRA 3B', object_type: 'PAYLOAD', country_code: 'LUX', launch_date: 1275350400000, decay_date: null, rcs_size: 'LARGE' },
		{ norad_id: 12345, name: 'COSMOS 1408 DEB', object_type: 'DEBRIS', country_code: 'CIS', launch_date: 1636934400000, decay_date: null, rcs_size: 'SMALL' },
		{ norad_id: 88234, name: 'SL-16 R/B', object_type: 'ROCKET_BODY', country_code: 'CIS', launch_date: 946684800000, decay_date: null, rcs_size: 'LARGE' },
	];

	// Initialize mock data
	objects.set(mockObjects);

	function formatDate(timestamp: number | null): string {
		if (!timestamp) return '-';
		return new Date(timestamp).toLocaleDateString();
	}

	function getTypeColor(type: string): string {
		switch (type) {
			case 'PAYLOAD': return '#44ff88';
			case 'DEBRIS': return '#ff8844';
			case 'ROCKET_BODY': return '#ffcc00';
			default: return '#888';
		}
	}
</script>

<div class="objects-page">
	<header class="page-header">
		<h1>Tracked Objects</h1>
		<div class="header-actions">
			<button class="btn btn-primary">+ Register Object</button>
			<button class="btn">Import TLE</button>
			<button class="btn">Bulk Upload</button>
		</div>
	</header>

	<div class="stats-cards">
		<div class="stat-card">
			<span class="stat-value">{$objectStats.total.toLocaleString()}</span>
			<span class="stat-label">Total Objects</span>
		</div>
		<div class="stat-card satellites">
			<span class="stat-value">{$objectStats.satellites.toLocaleString()}</span>
			<span class="stat-label">Satellites</span>
		</div>
		<div class="stat-card debris">
			<span class="stat-value">{$objectStats.debris.toLocaleString()}</span>
			<span class="stat-label">Debris</span>
		</div>
		<div class="stat-card rocket">
			<span class="stat-value">{$objectStats.rocketBodies.toLocaleString()}</span>
			<span class="stat-label">Rocket Bodies</span>
		</div>
	</div>

	<div class="search-bar">
		<input
			type="text"
			placeholder="Search by name, NORAD ID, type, or country..."
			bind:value={$searchQuery}
		/>
		<span class="result-count">{$filteredObjects.length} results</span>
	</div>

	<div class="card">
		<table>
			<thead>
				<tr>
					<th>NORAD ID</th>
					<th>Name</th>
					<th>Type</th>
					<th>Country</th>
					<th>Launch Date</th>
					<th>RCS Size</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each $filteredObjects as obj}
					<tr>
						<td>
							<span class="norad-id">{obj.norad_id}</span>
						</td>
						<td class="object-name">{obj.name}</td>
						<td>
							<span class="type-badge" style="color: {getTypeColor(obj.object_type)}">
								{obj.object_type.replace('_', ' ')}
							</span>
						</td>
						<td>{obj.country_code}</td>
						<td>{formatDate(obj.launch_date)}</td>
						<td>{obj.rcs_size || '-'}</td>
						<td>
							<button class="btn-small">TLE</button>
							<button class="btn-small">Track</button>
							<button class="btn-small">Details</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<div class="pagination">
		<button class="btn-small" disabled>Previous</button>
		<span class="page-info">Page 1 of 1</span>
		<button class="btn-small" disabled>Next</button>
	</div>
</div>

<style>
	.objects-page {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.page-header h1 {
		color: #00d4ff;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.stats-cards {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.stat-card {
		background: #12121a;
		border: 1px solid #2a2a3a;
		border-radius: 12px;
		padding: 1rem;
		text-align: center;
	}

	.stat-card .stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #00d4ff;
		display: block;
	}

	.stat-card .stat-label {
		font-size: 0.75rem;
		color: #888;
		text-transform: uppercase;
	}

	.stat-card.satellites .stat-value { color: #44ff88; }
	.stat-card.debris .stat-value { color: #ff8844; }
	.stat-card.rocket .stat-value { color: #ffcc00; }

	.search-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.search-bar input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: 1px solid #2a2a3a;
		border-radius: 8px;
		background: #12121a;
		color: #e0e0e0;
		font-size: 0.9rem;
	}

	.search-bar input:focus {
		outline: none;
		border-color: #00d4ff;
	}

	.search-bar input::placeholder {
		color: #666;
	}

	.result-count {
		color: #888;
		font-size: 0.85rem;
	}

	.norad-id {
		font-family: monospace;
		color: #00d4ff;
	}

	.object-name {
		font-weight: 500;
	}

	.type-badge {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.btn-small {
		padding: 0.3rem 0.6rem;
		font-size: 0.75rem;
		border: 1px solid #2a2a3a;
		border-radius: 4px;
		background: transparent;
		color: #888;
		cursor: pointer;
		margin-right: 0.25rem;
	}

	.btn-small:hover {
		background: #1a1a2a;
		color: #e0e0e0;
	}

	.btn-small:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}

	.page-info {
		color: #888;
		font-size: 0.85rem;
	}
</style>

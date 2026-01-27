<script lang="ts">
	import { conjunctions, conjunctionStats, riskFilter, filteredConjunctions } from '$lib/stores/conjunctions';

	// Mock data for demo
	const mockConjunctions = [
		{
			primary_norad_id: 48274,
			secondary_norad_id: 25544,
			tca: Date.now() + 3600000 * 2, // 2 hours from now
			miss_distance_km: 0.15,
			collision_probability: 0.0034,
			risk_level: 'HIGH'
		},
		{
			primary_norad_id: 51234,
			secondary_norad_id: 99999,
			tca: Date.now() + 3600000 * 8, // 8 hours from now
			miss_distance_km: 0.45,
			collision_probability: 0.00089,
			risk_level: 'MEDIUM'
		},
		{
			primary_norad_id: 43205,
			secondary_norad_id: 88234,
			tca: Date.now() + 3600000 * 24, // 24 hours from now
			miss_distance_km: 1.2,
			collision_probability: 0.00012,
			risk_level: 'LOW'
		},
		{
			primary_norad_id: 55123,
			secondary_norad_id: 12345,
			tca: Date.now() + 3600000 * 0.5, // 30 min from now
			miss_distance_km: 0.05,
			collision_probability: 0.0156,
			risk_level: 'EMERGENCY'
		}
	];

	// Initialize mock data
	conjunctions.set(mockConjunctions);

	function formatTca(timestamp: number): string {
		const diff = timestamp - Date.now();
		const hours = Math.floor(diff / 3600000);
		const mins = Math.floor((diff % 3600000) / 60000);
		if (hours > 0) return `${hours}h ${mins}m`;
		return `${mins}m`;
	}

	function formatProbability(prob: number): string {
		if (prob >= 0.01) return (prob * 100).toFixed(2) + '%';
		return prob.toExponential(2);
	}

	function setFilter(level: string | null) {
		riskFilter.set(level);
	}
</script>

<div class="conjunctions-page">
	<header class="page-header">
		<h1>Active Conjunctions</h1>
		<div class="header-actions">
			<button class="btn btn-primary">+ Report Conjunction</button>
			<button class="btn">Generate CDM</button>
		</div>
	</header>

	<div class="stats-cards">
		<div class="stat-card">
			<span class="stat-value">{$conjunctionStats.total}</span>
			<span class="stat-label">Total Active</span>
		</div>
		<div class="stat-card emergency">
			<span class="stat-value">{$conjunctionStats.emergency}</span>
			<span class="stat-label">Emergency</span>
		</div>
		<div class="stat-card high">
			<span class="stat-value">{$conjunctionStats.high}</span>
			<span class="stat-label">High Risk</span>
		</div>
		<div class="stat-card medium">
			<span class="stat-value">{$conjunctionStats.medium}</span>
			<span class="stat-label">Medium Risk</span>
		</div>
		<div class="stat-card low">
			<span class="stat-value">{$conjunctionStats.low}</span>
			<span class="stat-label">Low Risk</span>
		</div>
	</div>

	<div class="filters">
		<span class="filter-label">Filter by risk:</span>
		<button class="filter-btn" class:active={$riskFilter === null} on:click={() => setFilter(null)}>All</button>
		<button class="filter-btn emergency" class:active={$riskFilter === 'EMERGENCY'} on:click={() => setFilter('EMERGENCY')}>Emergency</button>
		<button class="filter-btn high" class:active={$riskFilter === 'HIGH'} on:click={() => setFilter('HIGH')}>High</button>
		<button class="filter-btn medium" class:active={$riskFilter === 'MEDIUM'} on:click={() => setFilter('MEDIUM')}>Medium</button>
		<button class="filter-btn low" class:active={$riskFilter === 'LOW'} on:click={() => setFilter('LOW')}>Low</button>
	</div>

	<div class="card">
		<table>
			<thead>
				<tr>
					<th>Risk</th>
					<th>Primary Object</th>
					<th>Secondary Object</th>
					<th>Time to TCA</th>
					<th>Miss Distance</th>
					<th>Probability</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each $filteredConjunctions as conj}
					<tr class="conjunction-row {conj.risk_level.toLowerCase()}">
						<td>
							<span class="risk-badge risk-{conj.risk_level.toLowerCase()}">{conj.risk_level}</span>
						</td>
						<td>
							<span class="norad-id">{conj.primary_norad_id}</span>
						</td>
						<td>
							<span class="norad-id">{conj.secondary_norad_id}</span>
						</td>
						<td>
							<span class="tca" class:urgent={conj.tca - Date.now() < 3600000}>
								{formatTca(conj.tca)}
							</span>
						</td>
						<td>{conj.miss_distance_km.toFixed(3)} km</td>
						<td>{formatProbability(conj.collision_probability)}</td>
						<td>
							<button class="btn-small">View CDM</button>
							<button class="btn-small">Details</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style>
	.conjunctions-page {
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
		grid-template-columns: repeat(5, 1fr);
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

	.stat-card.emergency .stat-value { color: #ff4444; }
	.stat-card.high .stat-value { color: #ff8844; }
	.stat-card.medium .stat-value { color: #ffcc00; }
	.stat-card.low .stat-value { color: #44ff88; }

	.filters {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.filter-label {
		color: #888;
		font-size: 0.85rem;
	}

	.filter-btn {
		padding: 0.4rem 0.8rem;
		border: 1px solid #2a2a3a;
		border-radius: 6px;
		background: transparent;
		color: #888;
		cursor: pointer;
		font-size: 0.8rem;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: #1a1a2a;
		color: #e0e0e0;
	}

	.filter-btn.active {
		background: #00d4ff20;
		border-color: #00d4ff;
		color: #00d4ff;
	}

	.filter-btn.emergency.active { background: #ff444420; border-color: #ff4444; color: #ff4444; }
	.filter-btn.high.active { background: #ff884420; border-color: #ff8844; color: #ff8844; }
	.filter-btn.medium.active { background: #ffcc0020; border-color: #ffcc00; color: #ffcc00; }
	.filter-btn.low.active { background: #44ff8820; border-color: #44ff88; color: #44ff88; }

	.conjunction-row.emergency { background: #ff444410; }
	.conjunction-row.high { background: #ff884410; }

	.norad-id {
		font-family: monospace;
		color: #00d4ff;
	}

	.tca {
		font-weight: 600;
	}

	.tca.urgent {
		color: #ff4444;
		animation: pulse 1s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.6; }
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
</style>

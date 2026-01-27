<script lang="ts">
	import { writable, derived } from 'svelte/store';

	interface Bounty {
		id: string;
		target_norad_id: number;
		target_name: string;
		title: string;
		description: string;
		reward_amount: number;
		funded_amount: number;
		deadline: number | null;
		status: string;
		contributors: number;
	}

	// Mock bounty data
	const bounties = writable<Bounty[]>([
		{
			id: 'bounty-001',
			target_norad_id: 43205,
			target_name: 'COSMOS 2251 DEB',
			title: 'Remove Large Debris Fragment',
			description: 'This debris fragment from the 2009 collision poses significant risk to operational satellites in LEO.',
			reward_amount: 50000,
			funded_amount: 32500,
			deadline: Date.now() + 86400000 * 90,
			status: 'ACTIVE',
			contributors: 47
		},
		{
			id: 'bounty-002',
			target_norad_id: 99999,
			target_name: 'FENGYUN 1C DEB',
			title: 'Track & Catalog Debris Cloud',
			description: 'Improve tracking accuracy for the Fengyun-1C debris cloud. Over 3,000 trackable fragments.',
			reward_amount: 25000,
			funded_amount: 25000,
			deadline: Date.now() + 86400000 * 30,
			status: 'FUNDED',
			contributors: 128
		},
		{
			id: 'bounty-003',
			target_norad_id: 88234,
			target_name: 'SL-16 R/B',
			title: 'Deorbit Rocket Body',
			description: 'Large rocket body in congested orbit. High priority for active debris removal mission.',
			reward_amount: 100000,
			funded_amount: 15000,
			deadline: Date.now() + 86400000 * 180,
			status: 'ACTIVE',
			contributors: 23
		},
		{
			id: 'bounty-004',
			target_norad_id: 12345,
			target_name: 'COSMOS 1408 DEB',
			title: 'Emergency Tracking Enhancement',
			description: 'Recent ASAT test debris. Urgent need for improved tracking to protect ISS and crewed missions.',
			reward_amount: 75000,
			funded_amount: 68000,
			deadline: Date.now() + 86400000 * 14,
			status: 'URGENT',
			contributors: 312
		}
	]);

	const bountyStats = derived(bounties, ($bounties) => {
		const total = $bounties.length;
		const totalReward = $bounties.reduce((sum, b) => sum + b.reward_amount, 0);
		const totalFunded = $bounties.reduce((sum, b) => sum + b.funded_amount, 0);
		const totalContributors = $bounties.reduce((sum, b) => sum + b.contributors, 0);
		return { total, totalReward, totalFunded, totalContributors };
	});

	function formatCurrency(amount: number): string {
		return '$' + amount.toLocaleString();
	}

	function formatDeadline(timestamp: number | null): string {
		if (!timestamp) return 'No deadline';
		const days = Math.ceil((timestamp - Date.now()) / 86400000);
		if (days < 0) return 'Expired';
		if (days === 0) return 'Today';
		if (days === 1) return 'Tomorrow';
		return `${days} days`;
	}

	function getProgress(funded: number, total: number): number {
		return Math.min(100, (funded / total) * 100);
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'ACTIVE': return '#00d4ff';
			case 'FUNDED': return '#44ff88';
			case 'URGENT': return '#ff4444';
			case 'COMPLETED': return '#888';
			default: return '#888';
		}
	}
</script>

<div class="bounties-page">
	<header class="page-header">
		<h1>Debris Bounties</h1>
		<div class="header-actions">
			<button class="btn btn-primary">+ Create Bounty</button>
			<button class="btn">My Contributions</button>
		</div>
	</header>

	<div class="stats-cards">
		<div class="stat-card">
			<span class="stat-value">{$bountyStats.total}</span>
			<span class="stat-label">Active Bounties</span>
		</div>
		<div class="stat-card reward">
			<span class="stat-value">{formatCurrency($bountyStats.totalReward)}</span>
			<span class="stat-label">Total Rewards</span>
		</div>
		<div class="stat-card funded">
			<span class="stat-value">{formatCurrency($bountyStats.totalFunded)}</span>
			<span class="stat-label">Funded</span>
		</div>
		<div class="stat-card contributors">
			<span class="stat-value">{$bountyStats.totalContributors.toLocaleString()}</span>
			<span class="stat-label">Contributors</span>
		</div>
	</div>

	<div class="bounty-grid">
		{#each $bounties as bounty}
			<div class="bounty-card">
				<div class="bounty-header">
					<span class="status-badge" style="background: {getStatusColor(bounty.status)}20; color: {getStatusColor(bounty.status)}; border-color: {getStatusColor(bounty.status)}">
						{bounty.status}
					</span>
					<span class="deadline" class:urgent={bounty.status === 'URGENT'}>
						{formatDeadline(bounty.deadline)}
					</span>
				</div>

				<h3 class="bounty-title">{bounty.title}</h3>

				<div class="target-info">
					<span class="target-label">Target:</span>
					<span class="norad-id">{bounty.target_norad_id}</span>
					<span class="target-name">{bounty.target_name}</span>
				</div>

				<p class="bounty-description">{bounty.description}</p>

				<div class="funding-progress">
					<div class="progress-header">
						<span class="funded-amount">{formatCurrency(bounty.funded_amount)}</span>
						<span class="reward-amount">of {formatCurrency(bounty.reward_amount)}</span>
					</div>
					<div class="progress-bar">
						<div class="progress-fill" style="width: {getProgress(bounty.funded_amount, bounty.reward_amount)}%"></div>
					</div>
					<div class="progress-footer">
						<span class="contributors">{bounty.contributors} contributors</span>
						<span class="percentage">{getProgress(bounty.funded_amount, bounty.reward_amount).toFixed(0)}%</span>
					</div>
				</div>

				<div class="bounty-actions">
					<button class="btn btn-primary">Fund Bounty</button>
					<button class="btn">View Details</button>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.bounties-page {
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
		font-size: 1.5rem;
		font-weight: 700;
		color: #00d4ff;
		display: block;
	}

	.stat-card .stat-label {
		font-size: 0.75rem;
		color: #888;
		text-transform: uppercase;
	}

	.stat-card.reward .stat-value { color: #ffcc00; }
	.stat-card.funded .stat-value { color: #44ff88; }
	.stat-card.contributors .stat-value { color: #00d4ff; }

	.bounty-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.bounty-card {
		background: #12121a;
		border: 1px solid #2a2a3a;
		border-radius: 12px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.bounty-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.status-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		border: 1px solid;
	}

	.deadline {
		font-size: 0.8rem;
		color: #888;
	}

	.deadline.urgent {
		color: #ff4444;
		font-weight: 600;
	}

	.bounty-title {
		margin: 0;
		font-size: 1.1rem;
		color: #e0e0e0;
	}

	.target-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
	}

	.target-label {
		color: #666;
	}

	.norad-id {
		font-family: monospace;
		color: #00d4ff;
	}

	.target-name {
		color: #888;
	}

	.bounty-description {
		margin: 0;
		font-size: 0.85rem;
		color: #888;
		line-height: 1.5;
	}

	.funding-progress {
		background: #0a0a0f;
		border-radius: 8px;
		padding: 1rem;
	}

	.progress-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.funded-amount {
		font-weight: 700;
		color: #44ff88;
	}

	.reward-amount {
		color: #888;
		font-size: 0.85rem;
	}

	.progress-bar {
		height: 8px;
		background: #2a2a3a;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #00d4ff, #44ff88);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-footer {
		display: flex;
		justify-content: space-between;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #666;
	}

	.bounty-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: auto;
	}

	.bounty-actions .btn {
		flex: 1;
	}
</style>

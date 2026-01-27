<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fetchTLEs, fetchDemoData, CATALOG_GROUPS, type CatalogGroup } from '$lib/orbital/celestrak';
	import type { TLE } from '$lib/orbital/propagator';

	const dispatch = createEventDispatcher<{
		load: { tles: TLE[] };
		error: { message: string };
	}>();

	let isLoading = false;
	let loadedCount = 0;
	let selectedGroup: CatalogGroup = CATALOG_GROUPS.STATIONS;
	let error: string | null = null;

	const groupOptions: { value: CatalogGroup; label: string }[] = [
		{ value: CATALOG_GROUPS.STATIONS, label: 'Space Stations' },
		{ value: CATALOG_GROUPS.ACTIVE, label: 'All Active Satellites' },
		{ value: CATALOG_GROUPS.STARLINK, label: 'Starlink Constellation' },
		{ value: CATALOG_GROUPS.ONEWEB, label: 'OneWeb Constellation' },
		{ value: CATALOG_GROUPS.WEATHER, label: 'Weather Satellites' },
		{ value: CATALOG_GROUPS.GPS, label: 'GPS Satellites' },
		{ value: CATALOG_GROUPS.VISUAL, label: 'Visually Bright' },
		{ value: CATALOG_GROUPS.LAST_30_DAYS, label: 'Recently Launched' },
		{ value: CATALOG_GROUPS.COSMOS_2251_DEBRIS, label: 'Cosmos 2251 Debris' },
		{ value: CATALOG_GROUPS.IRIDIUM_33_DEBRIS, label: 'Iridium 33 Debris' },
	];

	async function loadData() {
		isLoading = true;
		error = null;

		try {
			const tles = await fetchTLEs(selectedGroup);
			loadedCount = tles.length;
			dispatch('load', { tles });
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch data';
			dispatch('error', { message: error });
		} finally {
			isLoading = false;
		}
	}

	async function loadDemoData() {
		isLoading = true;
		error = null;

		try {
			const tles = await fetchDemoData();
			loadedCount = tles.length;
			dispatch('load', { tles });
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch demo data';
			dispatch('error', { message: error });
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="data-loader">
	<h4>Load CelesTrak Data</h4>

	<div class="loader-controls">
		<select bind:value={selectedGroup} disabled={isLoading}>
			{#each groupOptions as opt}
				<option value={opt.value}>{opt.label}</option>
			{/each}
		</select>

		<button
			class="btn btn-primary"
			on:click={loadData}
			disabled={isLoading}
		>
			{#if isLoading}
				Loading...
			{:else}
				Load TLEs
			{/if}
		</button>

		<button
			class="btn"
			on:click={loadDemoData}
			disabled={isLoading}
		>
			Quick Demo
		</button>
	</div>

	{#if error}
		<div class="error-message">
			⚠️ {error}
		</div>
	{/if}

	{#if loadedCount > 0 && !error}
		<div class="success-message">
			✓ Loaded {loadedCount} objects
		</div>
	{/if}

	<p class="note">
		Data from <a href="https://celestrak.org" target="_blank" rel="noopener">CelesTrak</a>
	</p>
</div>

<style>
	.data-loader {
		background: rgba(18, 18, 26, 0.9);
		border: 1px solid #2a2a3a;
		border-radius: 8px;
		padding: 1rem;
	}

	h4 {
		margin: 0 0 0.75rem 0;
		color: #888;
		font-size: 0.75rem;
		text-transform: uppercase;
	}

	.loader-controls {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	select {
		padding: 0.5rem;
		border: 1px solid #2a2a3a;
		border-radius: 6px;
		background: #1a1a2a;
		color: #e0e0e0;
		font-size: 0.8rem;
	}

	select:focus {
		outline: none;
		border-color: #00d4ff;
	}

	.btn {
		padding: 0.5rem;
		font-size: 0.8rem;
	}

	.error-message {
		margin-top: 0.5rem;
		padding: 0.5rem;
		background: #ff444420;
		border: 1px solid #ff4444;
		border-radius: 4px;
		color: #ff8888;
		font-size: 0.75rem;
	}

	.success-message {
		margin-top: 0.5rem;
		padding: 0.5rem;
		background: #44ff8820;
		border: 1px solid #44ff88;
		border-radius: 4px;
		color: #88ff88;
		font-size: 0.75rem;
	}

	.note {
		margin: 0.75rem 0 0 0;
		font-size: 0.7rem;
		color: #666;
	}

	.note a {
		color: #00d4ff;
	}
</style>

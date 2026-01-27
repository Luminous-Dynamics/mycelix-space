<script lang="ts">
	import { connectionStatus, errorMessage, connect, disconnect } from '$lib/holochain/client';
	import { onMount } from 'svelte';

	let conductorUrl = 'ws://localhost:8888';
	let autoConnect = true;

	onMount(() => {
		if (autoConnect) {
			attemptConnect();
		}
	});

	async function attemptConnect() {
		try {
			await connect(conductorUrl);
		} catch (e) {
			// Error is already handled in the client
			console.log('Connection failed, will retry or use mock data');
		}
	}

	function handleDisconnect() {
		disconnect();
	}
</script>

<div class="connection-status">
	{#if $connectionStatus === 'connected'}
		<span class="status-dot connected"></span>
		<span class="status-text">Holochain: Connected</span>
		<button class="disconnect-btn" on:click={handleDisconnect} title="Disconnect">
			✕
		</button>
	{:else if $connectionStatus === 'connecting'}
		<span class="status-dot connecting"></span>
		<span class="status-text">Connecting...</span>
	{:else if $connectionStatus === 'error'}
		<span class="status-dot error"></span>
		<div class="status-details">
			<span class="status-text">Holochain: Offline</span>
			<span class="error-hint" title={$errorMessage || 'Connection failed'}>
				(Using demo data)
			</span>
		</div>
		<button class="retry-btn" on:click={attemptConnect} title="Retry connection">
			↻
		</button>
	{:else}
		<span class="status-dot disconnected"></span>
		<span class="status-text">Holochain: Disconnected</span>
		<button class="connect-btn" on:click={attemptConnect} title="Connect">
			→
		</button>
	{/if}
</div>

<style>
	.connection-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		font-size: 0.8rem;
		color: #888;
		border-top: 1px solid #2a2a3a;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.status-dot.connected {
		background: #44ff88;
		box-shadow: 0 0 6px #44ff88;
	}

	.status-dot.connecting {
		background: #ffcc00;
		animation: pulse 1s infinite;
	}

	.status-dot.error {
		background: #ff8844;
	}

	.status-dot.disconnected {
		background: #ff4444;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.4; }
	}

	.status-text {
		flex: 1;
	}

	.status-details {
		display: flex;
		flex-direction: column;
		flex: 1;
		gap: 0.1rem;
	}

	.error-hint {
		font-size: 0.7rem;
		color: #666;
		cursor: help;
	}

	.connect-btn,
	.disconnect-btn,
	.retry-btn {
		width: 20px;
		height: 20px;
		border: none;
		border-radius: 4px;
		background: #2a2a3a;
		color: #888;
		cursor: pointer;
		font-size: 0.7rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.connect-btn:hover,
	.retry-btn:hover {
		background: #3a3a4a;
		color: #44ff88;
	}

	.disconnect-btn:hover {
		background: #3a3a4a;
		color: #ff4444;
	}
</style>

<script lang="ts">
	import '../app.css';
</script>

<div class="app">
	<nav class="sidebar">
		<div class="logo">
			<h1>MYCELIX</h1>
			<span class="subtitle">Space Traffic Control</span>
		</div>

		<ul class="nav-links">
			<li><a href="/" class:active={$page?.url?.pathname === '/'}>
				<span class="icon">🌍</span> Live Map
			</a></li>
			<li><a href="/conjunctions" class:active={$page?.url?.pathname === '/conjunctions'}>
				<span class="icon">⚠️</span> Conjunctions
			</a></li>
			<li><a href="/objects" class:active={$page?.url?.pathname === '/objects'}>
				<span class="icon">🛰️</span> Objects
			</a></li>
			<li><a href="/bounties" class:active={$page?.url?.pathname === '/bounties'}>
				<span class="icon">💰</span> Bounties
			</a></li>
		</ul>

		<div class="connection-status">
			<span class="status-dot" class:connected={true}></span>
			<span>Holochain: Connected</span>
		</div>
	</nav>

	<main class="content">
		<slot />
	</main>
</div>

<script context="module" lang="ts">
	import { page } from '$app/stores';
</script>

<style>
	.app {
		display: flex;
		height: 100vh;
		background: #0a0a0f;
		color: #e0e0e0;
	}

	.sidebar {
		width: 240px;
		background: #12121a;
		border-right: 1px solid #2a2a3a;
		display: flex;
		flex-direction: column;
		padding: 1rem;
	}

	.logo {
		padding: 1rem 0;
		border-bottom: 1px solid #2a2a3a;
		margin-bottom: 1rem;
	}

	.logo h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #00d4ff;
		margin: 0;
		letter-spacing: 2px;
	}

	.logo .subtitle {
		font-size: 0.75rem;
		color: #888;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.nav-links {
		list-style: none;
		padding: 0;
		margin: 0;
		flex: 1;
	}

	.nav-links li {
		margin: 0.25rem 0;
	}

	.nav-links a {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: #888;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.nav-links a:hover {
		background: #1a1a2a;
		color: #e0e0e0;
	}

	.nav-links a.active {
		background: #00d4ff20;
		color: #00d4ff;
	}

	.icon {
		font-size: 1.25rem;
	}

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
		background: #ff4444;
	}

	.status-dot.connected {
		background: #44ff88;
	}

	.content {
		flex: 1;
		overflow: auto;
	}
</style>

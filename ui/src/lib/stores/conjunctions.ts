import { writable, derived } from 'svelte/store';
import type { Conjunction, Alert } from '$lib/holochain/client';

// Store for active conjunctions
export const conjunctions = writable<Conjunction[]>([]);

// Store for alerts
export const alerts = writable<Alert[]>([]);

// Loading states
export const isLoading = writable(false);

// Risk level filter
export const riskFilter = writable<string | null>(null);

// Filtered conjunctions by risk level
export const filteredConjunctions = derived(
	[conjunctions, riskFilter],
	([$conjunctions, $filter]) => {
		if (!$filter) return $conjunctions;
		return $conjunctions.filter(c => c.risk_level === $filter);
	}
);

// Conjunctions sorted by urgency (TCA closest first)
export const urgentConjunctions = derived(conjunctions, ($conjunctions) => {
	return [...$conjunctions]
		.sort((a, b) => a.tca - b.tca)
		.slice(0, 10);
});

// Stats derived from conjunctions
export const conjunctionStats = derived(conjunctions, ($conjunctions) => {
	const total = $conjunctions.length;
	const emergency = $conjunctions.filter(c => c.risk_level === 'EMERGENCY').length;
	const high = $conjunctions.filter(c => c.risk_level === 'HIGH').length;
	const medium = $conjunctions.filter(c => c.risk_level === 'MEDIUM').length;
	const low = $conjunctions.filter(c => c.risk_level === 'LOW').length;

	return { total, emergency, high, medium, low };
});

// Unread alerts count
export const unreadAlertCount = derived(alerts, ($alerts) => {
	// For now, count all as unread - would track read state in real app
	return $alerts.length;
});

// Actions
export function addConjunction(conj: Conjunction) {
	conjunctions.update(list => {
		const key = `${conj.primary_norad_id}-${conj.secondary_norad_id}`;
		const existsIdx = list.findIndex(c =>
			`${c.primary_norad_id}-${c.secondary_norad_id}` === key
		);
		if (existsIdx >= 0) {
			const updated = [...list];
			updated[existsIdx] = conj;
			return updated;
		}
		return [...list, conj];
	});
}

export function addAlert(alert: Alert) {
	alerts.update(list => {
		if (list.find(a => a.id === alert.id)) return list;
		return [alert, ...list].slice(0, 100); // Keep last 100
	});
}

export function clearConjunctions() {
	conjunctions.set([]);
}

export function clearAlerts() {
	alerts.set([]);
}

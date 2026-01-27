import { writable, derived } from 'svelte/store';
import type { OrbitalObject, TleData } from '$lib/holochain/client';

// Store for all tracked orbital objects
export const objects = writable<OrbitalObject[]>([]);

// Store for TLE data (keyed by NORAD ID)
export const tleData = writable<Map<number, TleData>>(new Map());

// Loading state
export const isLoading = writable(false);

// Search query
export const searchQuery = writable('');

// Filtered objects based on search
export const filteredObjects = derived(
	[objects, searchQuery],
	([$objects, $query]) => {
		if (!$query.trim()) return $objects;
		const q = $query.toLowerCase();
		return $objects.filter(obj =>
			obj.name.toLowerCase().includes(q) ||
			obj.norad_id.toString().includes(q) ||
			obj.object_type.toLowerCase().includes(q) ||
			obj.country_code.toLowerCase().includes(q)
		);
	}
);

// Stats derived from objects
export const objectStats = derived(objects, ($objects) => {
	const total = $objects.length;
	const satellites = $objects.filter(o => o.object_type === 'PAYLOAD').length;
	const debris = $objects.filter(o => o.object_type === 'DEBRIS').length;
	const rocketBodies = $objects.filter(o => o.object_type === 'ROCKET_BODY').length;

	return { total, satellites, debris, rocketBodies };
});

// Actions
export function addObject(obj: OrbitalObject) {
	objects.update(list => {
		const exists = list.find(o => o.norad_id === obj.norad_id);
		if (exists) {
			return list.map(o => o.norad_id === obj.norad_id ? obj : o);
		}
		return [...list, obj];
	});
}

export function updateTle(noradId: number, tle: TleData) {
	tleData.update(map => {
		map.set(noradId, tle);
		return new Map(map);
	});
}

export function clearObjects() {
	objects.set([]);
	tleData.set(new Map());
}

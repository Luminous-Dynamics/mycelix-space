/**
 * CelesTrak Data Fetcher
 * Fetches real TLE data from CelesTrak's public API
 */

import type { TLE } from './propagator';

// CelesTrak GP (General Perturbations) API endpoints
const CELESTRAK_BASE = 'https://celestrak.org/NORAD/elements/gp.php';

// Common catalog groups
export const CATALOG_GROUPS = {
	// Active satellites
	STATIONS: 'stations',           // Space stations
	ACTIVE: 'active',               // All active satellites
	STARLINK: 'starlink',           // Starlink constellation
	ONEWEB: 'oneweb',               // OneWeb constellation
	WEATHER: 'weather',             // Weather satellites
	GPS: 'gps-ops',                 // GPS operational
	GALILEO: 'galileo',             // Galileo constellation
	GLONASS: 'glo-ops',             // GLONASS operational

	// Special interest
	LAST_30_DAYS: 'last-30-days',   // Recently launched
	VISUAL: 'visual',               // Visually bright satellites
	SCIENCE: 'science',             // Science satellites
	AMATEUR: 'amateur',             // Amateur radio satellites

	// Debris and defunct
	COSMOS_2251_DEBRIS: 'cosmos-2251-debris',
	IRIDIUM_33_DEBRIS: 'iridium-33-debris',
	FENGYUN_1C_DEBRIS: '1999-025',  // Fengyun-1C debris (by international designator)
} as const;

export type CatalogGroup = typeof CATALOG_GROUPS[keyof typeof CATALOG_GROUPS];

/**
 * Fetch TLEs from CelesTrak for a specific catalog group
 */
export async function fetchTLEs(group: CatalogGroup): Promise<TLE[]> {
	const url = `${CELESTRAK_BASE}?GROUP=${group}&FORMAT=tle`;

	try {
		const response = await fetch(url);
		if (!response.ok) {
			throw new Error(`CelesTrak returned ${response.status}`);
		}

		const text = await response.text();
		return parseTLEText(text);
	} catch (error) {
		console.error(`Failed to fetch TLEs for group ${group}:`, error);
		throw error;
	}
}

/**
 * Fetch TLE for a specific NORAD catalog number
 */
export async function fetchTLEByNorad(noradId: number): Promise<TLE | null> {
	const url = `${CELESTRAK_BASE}?CATNR=${noradId}&FORMAT=tle`;

	try {
		const response = await fetch(url);
		if (!response.ok) {
			return null;
		}

		const text = await response.text();
		const tles = parseTLEText(text);
		return tles.length > 0 ? tles[0] : null;
	} catch (error) {
		console.error(`Failed to fetch TLE for NORAD ${noradId}:`, error);
		return null;
	}
}

/**
 * Fetch TLEs by international designator (e.g., "1998-067A" for ISS)
 */
export async function fetchTLEByIntlDes(intldes: string): Promise<TLE[]> {
	const url = `${CELESTRAK_BASE}?INTDES=${intldes}&FORMAT=tle`;

	try {
		const response = await fetch(url);
		if (!response.ok) {
			throw new Error(`CelesTrak returned ${response.status}`);
		}

		const text = await response.text();
		return parseTLEText(text);
	} catch (error) {
		console.error(`Failed to fetch TLEs for INTDES ${intldes}:`, error);
		throw error;
	}
}

/**
 * Fetch TLEs with JSON format (includes additional metadata)
 */
export async function fetchTLEsJSON(group: CatalogGroup): Promise<CelesTrakObject[]> {
	const url = `${CELESTRAK_BASE}?GROUP=${group}&FORMAT=json`;

	try {
		const response = await fetch(url);
		if (!response.ok) {
			throw new Error(`CelesTrak returned ${response.status}`);
		}

		return await response.json();
	} catch (error) {
		console.error(`Failed to fetch JSON TLEs for group ${group}:`, error);
		throw error;
	}
}

/**
 * CelesTrak JSON format object
 */
export interface CelesTrakObject {
	OBJECT_NAME: string;
	OBJECT_ID: string;
	EPOCH: string;
	MEAN_MOTION: number;
	ECCENTRICITY: number;
	INCLINATION: number;
	RA_OF_ASC_NODE: number;
	ARG_OF_PERICENTER: number;
	MEAN_ANOMALY: number;
	EPHEMERIS_TYPE: number;
	CLASSIFICATION_TYPE: string;
	NORAD_CAT_ID: number;
	ELEMENT_SET_NO: number;
	REV_AT_EPOCH: number;
	BSTAR: number;
	MEAN_MOTION_DOT: number;
	MEAN_MOTION_DDOT: number;
}

/**
 * Convert CelesTrak JSON object to TLE format
 */
export function jsonToTLE(obj: CelesTrakObject): TLE {
	// Generate TLE lines from JSON data
	// This is a simplified conversion - real implementation would need proper formatting
	const line1 = generateTLELine1(obj);
	const line2 = generateTLELine2(obj);

	return {
		noradId: obj.NORAD_CAT_ID,
		name: obj.OBJECT_NAME,
		line1,
		line2
	};
}

/**
 * Parse TLE text (3-line format) into TLE objects
 */
export function parseTLEText(text: string): TLE[] {
	const lines = text.trim().split('\n').map(l => l.trim()).filter(l => l.length > 0);
	const tles: TLE[] = [];

	for (let i = 0; i < lines.length; i += 3) {
		if (i + 2 >= lines.length) break;

		const name = lines[i];
		const line1 = lines[i + 1];
		const line2 = lines[i + 2];

		// Validate TLE lines
		if (!line1.startsWith('1 ') || !line2.startsWith('2 ')) {
			console.warn('Invalid TLE format, skipping:', name);
			continue;
		}

		// Extract NORAD ID from line 1
		const noradId = parseInt(line1.substring(2, 7).trim(), 10);

		tles.push({
			noradId,
			name,
			line1,
			line2
		});
	}

	return tles;
}

// Helper functions for TLE line generation (simplified)
function generateTLELine1(obj: CelesTrakObject): string {
	// This is a placeholder - real implementation needs proper TLE formatting
	const noradStr = obj.NORAD_CAT_ID.toString().padStart(5, '0');
	const classification = obj.CLASSIFICATION_TYPE || 'U';

	// Parse epoch
	const epochDate = new Date(obj.EPOCH);
	const year = epochDate.getUTCFullYear() % 100;
	const dayOfYear = getDayOfYear(epochDate) + (epochDate.getUTCHours() + epochDate.getUTCMinutes() / 60 + epochDate.getUTCSeconds() / 3600) / 24;

	return `1 ${noradStr}${classification} ${obj.OBJECT_ID.padEnd(8)} ${year.toString().padStart(2, '0')}${dayOfYear.toFixed(8).padStart(12, '0')}  .00000000  00000-0  00000-0 0  9999`;
}

function generateTLELine2(obj: CelesTrakObject): string {
	const noradStr = obj.NORAD_CAT_ID.toString().padStart(5, '0');

	const inc = obj.INCLINATION.toFixed(4).padStart(8, ' ');
	const raan = obj.RA_OF_ASC_NODE.toFixed(4).padStart(8, ' ');
	const ecc = obj.ECCENTRICITY.toFixed(7).substring(2); // Remove "0."
	const argp = obj.ARG_OF_PERICENTER.toFixed(4).padStart(8, ' ');
	const ma = obj.MEAN_ANOMALY.toFixed(4).padStart(8, ' ');
	const mm = obj.MEAN_MOTION.toFixed(8).padStart(11, ' ');
	const rev = obj.REV_AT_EPOCH.toString().padStart(5, ' ');

	return `2 ${noradStr} ${inc} ${raan} ${ecc} ${argp} ${ma} ${mm}${rev}`;
}

function getDayOfYear(date: Date): number {
	const start = new Date(date.getUTCFullYear(), 0, 0);
	const diff = date.getTime() - start.getTime();
	const oneDay = 1000 * 60 * 60 * 24;
	return Math.floor(diff / oneDay);
}

/**
 * Cached TLE fetcher with expiration
 */
class TLECache {
	private cache: Map<string, { tles: TLE[]; timestamp: number }> = new Map();
	private maxAge: number = 3600000; // 1 hour default

	async get(group: CatalogGroup): Promise<TLE[]> {
		const cached = this.cache.get(group);
		if (cached && Date.now() - cached.timestamp < this.maxAge) {
			return cached.tles;
		}

		const tles = await fetchTLEs(group);
		this.cache.set(group, { tles, timestamp: Date.now() });
		return tles;
	}

	clear(): void {
		this.cache.clear();
	}
}

export const tleCache = new TLECache();

/**
 * Fetch multiple catalog groups in parallel
 */
export async function fetchMultipleGroups(groups: CatalogGroup[]): Promise<Map<CatalogGroup, TLE[]>> {
	const results = new Map<CatalogGroup, TLE[]>();
	const promises = groups.map(async group => {
		try {
			const tles = await tleCache.get(group);
			results.set(group, tles);
		} catch {
			results.set(group, []);
		}
	});

	await Promise.all(promises);
	return results;
}

/**
 * Get a curated set of interesting objects for the demo
 */
export async function fetchDemoData(): Promise<TLE[]> {
	const groups: CatalogGroup[] = [
		CATALOG_GROUPS.STATIONS,    // ISS, Tiangong, etc.
		CATALOG_GROUPS.VISUAL,      // Bright satellites
	];

	const results = await fetchMultipleGroups(groups);
	const allTles: TLE[] = [];

	for (const tles of results.values()) {
		allTles.push(...tles);
	}

	// Deduplicate by NORAD ID
	const seen = new Set<number>();
	return allTles.filter(tle => {
		if (seen.has(tle.noradId)) return false;
		seen.add(tle.noradId);
		return true;
	});
}

/**
 * SGP4 Satellite Propagator
 * Uses satellite.js for TLE propagation to real-time positions
 */

import * as satellite from 'satellite.js';

export interface TLE {
	noradId: number;
	name: string;
	line1: string;
	line2: string;
}

export interface SatellitePosition {
	noradId: number;
	name: string;
	latitude: number;   // degrees
	longitude: number;  // degrees
	altitude: number;   // km
	velocity: {
		x: number;  // km/s
		y: number;
		z: number;
	};
}

export interface OrbitalPath {
	noradId: number;
	positions: Array<{
		time: Date;
		latitude: number;
		longitude: number;
		altitude: number;
	}>;
}

/**
 * Parse TLE and create a satellite record
 */
export function parseTLE(tle: TLE): satellite.SatRec {
	return satellite.twoline2satrec(tle.line1, tle.line2);
}

/**
 * Propagate satellite to a specific time
 */
export function propagate(satrec: satellite.SatRec, time: Date): SatellitePosition | null {
	const positionAndVelocity = satellite.propagate(satrec, time);

	if (typeof positionAndVelocity.position === 'boolean' || !positionAndVelocity.position) {
		return null; // Propagation failed
	}

	const positionEci = positionAndVelocity.position as satellite.EciVec3<number>;
	const velocityEci = positionAndVelocity.velocity as satellite.EciVec3<number>;

	// Convert to geodetic coordinates
	const gmst = satellite.gstime(time);
	const positionGd = satellite.eciToGeodetic(positionEci, gmst);

	return {
		noradId: 0, // Set by caller
		name: '',   // Set by caller
		latitude: satellite.degreesLat(positionGd.latitude),
		longitude: satellite.degreesLong(positionGd.longitude),
		altitude: positionGd.height,
		velocity: {
			x: velocityEci.x,
			y: velocityEci.y,
			z: velocityEci.z
		}
	};
}

/**
 * Propagate a TLE to current time
 */
export function propagateTLE(tle: TLE, time?: Date): SatellitePosition | null {
	const satrec = parseTLE(tle);
	const result = propagate(satrec, time || new Date());

	if (result) {
		result.noradId = tle.noradId;
		result.name = tle.name;
	}

	return result;
}

/**
 * Generate orbital path for visualization
 */
export function generateOrbitalPath(
	tle: TLE,
	startTime: Date,
	durationMinutes: number = 90,
	stepMinutes: number = 1
): OrbitalPath {
	const satrec = parseTLE(tle);
	const positions: OrbitalPath['positions'] = [];

	const steps = Math.floor(durationMinutes / stepMinutes);

	for (let i = 0; i <= steps; i++) {
		const time = new Date(startTime.getTime() + i * stepMinutes * 60 * 1000);
		const pos = propagate(satrec, time);

		if (pos) {
			positions.push({
				time,
				latitude: pos.latitude,
				longitude: pos.longitude,
				altitude: pos.altitude
			});
		}
	}

	return {
		noradId: tle.noradId,
		positions
	};
}

/**
 * Batch propagate multiple satellites
 */
export function propagateAll(tles: TLE[], time?: Date): SatellitePosition[] {
	const t = time || new Date();
	const results: SatellitePosition[] = [];

	for (const tle of tles) {
		const pos = propagateTLE(tle, t);
		if (pos) {
			results.push(pos);
		}
	}

	return results;
}

/**
 * Calculate if satellite is in sunlight (for visualization)
 */
export function isInSunlight(position: SatellitePosition, time: Date): boolean {
	// Simplified calculation - check if satellite is on the sunlit side
	// For accurate results, would need sun position calculation
	const hourAngle = (time.getUTCHours() + time.getUTCMinutes() / 60) * 15 - 180;
	const sunLongitude = -hourAngle; // Approximate sun longitude

	const diffLon = Math.abs(position.longitude - sunLongitude);
	return diffLon < 90 || diffLon > 270;
}

// ============================================
// Sample TLE Data for Demo
// ============================================

export const SAMPLE_TLES: TLE[] = [
	{
		noradId: 25544,
		name: 'ISS (ZARYA)',
		line1: '1 25544U 98067A   24001.50000000  .00016717  00000-0  10270-3 0  9025',
		line2: '2 25544  51.6400 208.9163 0006703  35.5509 324.5921 15.49560234462411'
	},
	{
		noradId: 48274,
		name: 'STARLINK-1007',
		line1: '1 48274U 21021B   24001.50000000  .00001234  00000-0  12345-4 0  9999',
		line2: '2 48274  53.0540 123.4567 0001234  89.1234 270.9876 15.06123456789012'
	},
	{
		noradId: 43013,
		name: 'TIANGONG',
		line1: '1 43013U 17073A   24001.50000000  .00012345  00000-0  67890-4 0  9999',
		line2: '2 43013  41.4700 234.5678 0006789 123.4567 236.7890 15.60987654321098'
	},
	{
		noradId: 20580,
		name: 'HUBBLE SPACE TELESCOPE',
		line1: '1 20580U 90037B   24001.50000000  .00001234  00000-0  12345-4 0  9999',
		line2: '2 20580  28.4700  45.6789 0002345 234.5678 125.4321 15.09876543210987'
	},
	{
		noradId: 37820,
		name: 'TIANLIAN 1-02',
		line1: '1 37820U 11053A   24001.50000000  .00000123  00000-0  00000+0 0  9999',
		line2: '2 37820   1.2345 123.4567 0001234  12.3456 347.6543  1.00273456789012'
	},
	{
		noradId: 41335,
		name: 'GOES 16',
		line1: '1 41335U 16071A   24001.50000000  .00000012  00000-0  00000+0 0  9999',
		line2: '2 41335   0.0123 234.5678 0001234 123.4567 236.5432  1.00273456789012'
	}
];

// Debris objects (approximate TLEs for demonstration)
export const DEBRIS_TLES: TLE[] = [
	{
		noradId: 34454,
		name: 'COSMOS 2251 DEB',
		line1: '1 34454U 93036PX  24001.50000000  .00000500  00000-0  50000-4 0  9999',
		line2: '2 34454  74.0400  12.3456 0123456 234.5678 125.4321 14.12345678901234'
	},
	{
		noradId: 33506,
		name: 'FENGYUN 1C DEB',
		line1: '1 33506U 99025DPM 24001.50000000  .00000300  00000-0  30000-4 0  9999',
		line2: '2 33506  98.7600 345.6789 0054321  45.6789 314.3210 14.56789012345678'
	},
	{
		noradId: 49270,
		name: 'COSMOS 1408 DEB',
		line1: '1 49270U 82092BF  24001.50000000  .00000800  00000-0  80000-4 0  9999',
		line2: '2 49270  82.5600 178.9012 0087654 178.9012 181.0987 14.89012345678901'
	}
];

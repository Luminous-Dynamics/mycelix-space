import { AppWebsocket, type AppClient, type CallZomeRequest } from '@holochain/client';
import { writable, type Writable } from 'svelte/store';

// Connection state
export const connectionStatus: Writable<'disconnected' | 'connecting' | 'connected' | 'error'> = writable('disconnected');
export const errorMessage: Writable<string | null> = writable(null);

let client: AppClient | null = null;

// Default conductor URL (can be overridden)
const DEFAULT_CONDUCTOR_URL = 'ws://localhost:8888';

/**
 * Connect to the Holochain conductor
 */
export async function connect(url: string = DEFAULT_CONDUCTOR_URL): Promise<AppClient> {
	if (client) return client;

	connectionStatus.set('connecting');
	errorMessage.set(null);

	try {
		client = await AppWebsocket.connect(url);
		connectionStatus.set('connected');
		console.log('Connected to Holochain conductor at', url);
		return client;
	} catch (err) {
		const message = err instanceof Error ? err.message : 'Unknown error';
		connectionStatus.set('error');
		errorMessage.set(`Failed to connect: ${message}`);
		console.error('Holochain connection error:', err);
		throw err;
	}
}

/**
 * Disconnect from the conductor
 */
export async function disconnect(): Promise<void> {
	if (client) {
		// AppWebsocket doesn't have a close method, but we can null it
		client = null;
		connectionStatus.set('disconnected');
	}
}

/**
 * Get the current client instance
 */
export function getClient(): AppClient | null {
	return client;
}

/**
 * Call a zome function
 */
export async function callZome<T>(
	zomeName: string,
	fnName: string,
	payload: unknown = null
): Promise<T> {
	if (!client) {
		throw new Error('Not connected to Holochain conductor');
	}

	const request: CallZomeRequest = {
		role_name: 'mycelix_space',
		zome_name: zomeName,
		fn_name: fnName,
		payload
	};

	return await client.callZome(request) as T;
}

// ============================================
// Orbital Objects Zome Functions
// ============================================

export interface OrbitalObject {
	norad_id: number;
	name: string;
	object_type: string;
	country_code: string;
	launch_date: number | null;
	decay_date: number | null;
	rcs_size: string | null;
}

export interface TleData {
	norad_id: number;
	line1: string;
	line2: string;
	epoch: number;
	source: string;
}

export async function registerObject(object: OrbitalObject): Promise<string> {
	return callZome<string>('orbital_objects', 'register_object', object);
}

export async function getObject(noradId: number): Promise<OrbitalObject | null> {
	return callZome<OrbitalObject | null>('orbital_objects', 'get_object', noradId);
}

export async function submitTle(tle: TleData): Promise<string> {
	return callZome<string>('orbital_objects', 'submit_tle', tle);
}

export async function getLatestTle(noradId: number): Promise<TleData | null> {
	return callZome<TleData | null>('orbital_objects', 'get_latest_tle', noradId);
}

export async function searchObjects(query: string): Promise<OrbitalObject[]> {
	return callZome<OrbitalObject[]>('orbital_objects', 'search_objects', query);
}

// ============================================
// Observations Zome Functions
// ============================================

export interface Observation {
	norad_id: number;
	timestamp: number;
	ra: number;
	dec: number;
	range_km: number | null;
	range_rate_km_s: number | null;
	station_id: string;
	quality: string;
}

export async function submitObservation(obs: Observation): Promise<string> {
	return callZome<string>('observations', 'submit_observation', obs);
}

export async function getObservationsForObject(noradId: number): Promise<Observation[]> {
	return callZome<Observation[]>('observations', 'get_observations_for_object', noradId);
}

// ============================================
// Conjunctions Zome Functions
// ============================================

export interface Conjunction {
	primary_norad_id: number;
	secondary_norad_id: number;
	tca: number;
	miss_distance_km: number;
	collision_probability: number;
	risk_level: string;
}

export interface CdmMessage {
	message_id: string;
	primary_object: number;
	secondary_object: number;
	tca: number;
	miss_distance: number;
	collision_probability: number;
	created_at: number;
}

export async function reportConjunction(conj: Conjunction): Promise<string> {
	return callZome<string>('conjunctions', 'report_conjunction', conj);
}

export async function getActiveConjunctions(): Promise<Conjunction[]> {
	return callZome<Conjunction[]>('conjunctions', 'get_active_conjunctions', null);
}

export async function generateCdm(primaryId: number, secondaryId: number): Promise<CdmMessage> {
	return callZome<CdmMessage>('conjunctions', 'generate_cdm', { primary_id: primaryId, secondary_id: secondaryId });
}

// ============================================
// Debris Bounties Zome Functions
// ============================================

export interface Bounty {
	target_norad_id: number;
	title: string;
	description: string;
	reward_amount: number;
	deadline: number | null;
	status: string;
}

export async function createBounty(bounty: Bounty): Promise<string> {
	return callZome<string>('debris_bounties', 'create_bounty', bounty);
}

export async function getActiveBounties(): Promise<Bounty[]> {
	return callZome<Bounty[]>('debris_bounties', 'get_active_bounties', null);
}

export async function fundBounty(bountyId: string, amount: number): Promise<void> {
	return callZome<void>('debris_bounties', 'fund_bounty', { bounty_id: bountyId, amount });
}

// ============================================
// Traffic Control Zome Functions
// ============================================

export interface Alert {
	id: string;
	alert_type: string;
	severity: string;
	message: string;
	related_objects: number[];
	created_at: number;
	expires_at: number | null;
}

export async function getRecentAlerts(limit: number = 50): Promise<Alert[]> {
	return callZome<Alert[]>('traffic_control', 'get_recent_alerts', limit);
}

export async function acknowledgeAlert(alertId: string): Promise<void> {
	return callZome<void>('traffic_control', 'acknowledge_alert', alertId);
}

export async function getNetworkStats(): Promise<{
	total_objects: number;
	active_conjunctions: number;
	tracked_debris: number;
	network_peers: number;
}> {
	return callZome('traffic_control', 'get_network_stats', null);
}

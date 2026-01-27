//! Orbital Objects Coordinator Zome
//!
//! Provides the callable functions for managing the orbital object catalog:
//! - Register new objects
//! - Submit TLE updates
//! - Claim operator status
//! - Query catalog with link-based indexing

use hdk::prelude::*;
use orbital_objects_integrity::*;
use mycelix_space_shared::{SpaceTimestamp, QualityScore, DataSourceType};

// ============================================================================
// Constants for anchor paths
// ============================================================================

const ALL_OBJECTS_ANCHOR: &str = "all_objects";
const OBJECT_TYPE_ANCHOR: &str = "object_type";
const NORAD_INDEX_ANCHOR: &str = "norad";

// ============================================================================
// Entry Creation Functions
// ============================================================================

/// Register a new orbital object in the catalog
#[hdk_extern]
pub fn register_object(input: RegisterObjectInput) -> ExternResult<ActionHash> {
    let agent = agent_info()?.agent_initial_pubkey;

    let object = OrbitalObject {
        norad_id: input.norad_id,
        intl_designator: input.intl_designator,
        name: input.name,
        object_type: input.object_type.clone(),
        country: input.country,
        launch_date: input.launch_date,
        decay_date: None,
        status: input.status.unwrap_or(OperationalStatus::Unknown),
        created_at: SpaceTimestamp::now(),
        created_by: agent,
    };

    let action_hash = create_entry(&EntryTypes::OrbitalObject(object.clone()))?;

    // Create link from "all_objects" anchor to this object
    let all_objects_anchor = anchor_hash(ALL_OBJECTS_ANCHOR)?;
    create_link(
        all_objects_anchor.clone(),
        action_hash.clone(),
        LinkTypes::AllObjects,
        LinkTag::new(object.norad_id.to_string()),
    )?;

    // Create link for object type index
    let type_anchor = anchor_hash(&format!("{}:{:?}", OBJECT_TYPE_ANCHOR, input.object_type))?;
    create_link(
        type_anchor,
        action_hash.clone(),
        LinkTypes::OrbitTypeIndex,
        LinkTag::new(object.norad_id.to_string()),
    )?;

    // Create link from NORAD ID anchor for direct lookup
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, object.norad_id))?;
    create_link(
        norad_anchor,
        action_hash.clone(),
        LinkTypes::AllObjects,
        LinkTag::new("object"),
    )?;

    Ok(action_hash)
}

/// Input for registering a new object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterObjectInput {
    pub norad_id: u32,
    pub intl_designator: String,
    pub name: String,
    pub object_type: ObjectType,
    pub country: Option<String>,
    pub launch_date: Option<SpaceTimestamp>,
    pub status: Option<OperationalStatus>,
}

/// Submit a TLE for an object
#[hdk_extern]
pub fn submit_tle(input: SubmitTleInput) -> ExternResult<ActionHash> {
    let agent = agent_info()?.agent_initial_pubkey;

    // Parse TLE to extract epoch
    let epoch = extract_tle_epoch(&input.line1)?;

    let tle = TleRecord {
        norad_id: input.norad_id,
        line1: input.line1,
        line2: input.line2,
        epoch: epoch.clone(),
        source: input.source.unwrap_or(DataSourceType::SpaceTrack),
        quality: input.quality.unwrap_or_default(),
        submitted_at: SpaceTimestamp::now(),
        submitted_by: agent,
    };

    let action_hash = create_entry(&EntryTypes::TleRecord(tle))?;

    // Create link from NORAD anchor to TLE
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, input.norad_id))?;
    create_link(
        norad_anchor,
        action_hash.clone(),
        LinkTypes::ObjectToTles,
        LinkTag::new(epoch.micros.to_string()),
    )?;

    Ok(action_hash)
}

/// Input for submitting a TLE
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitTleInput {
    pub norad_id: u32,
    pub line1: String,
    pub line2: String,
    pub source: Option<DataSourceType>,
    pub quality: Option<QualityScore>,
}

/// Claim operator status for an object
#[hdk_extern]
pub fn claim_operator(input: ClaimOperatorInput) -> ExternResult<ActionHash> {
    let agent = agent_info()?.agent_initial_pubkey;

    let claim = OperatorClaim {
        norad_id: input.norad_id,
        operator: agent.clone(),
        organization: input.organization.clone(),
        contact: input.contact,
        claimed_at: SpaceTimestamp::now(),
        verified: false,
        verification_hash: input.verification_hash,
    };

    let action_hash = create_entry(&EntryTypes::OperatorClaim(claim))?;

    // Link from NORAD anchor to operator claim
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, input.norad_id))?;
    create_link(
        norad_anchor,
        action_hash.clone(),
        LinkTypes::ObjectToOperator,
        LinkTag::new(input.organization.as_bytes().to_vec()),
    )?;

    // Link from operator to their claimed objects
    let operator_anchor = anchor_hash(&format!("operator:{}", agent))?;
    create_link(
        operator_anchor,
        action_hash.clone(),
        LinkTypes::OperatorToObjects,
        LinkTag::new(input.norad_id.to_string()),
    )?;

    Ok(action_hash)
}

/// Input for claiming operator status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClaimOperatorInput {
    pub norad_id: u32,
    pub organization: String,
    pub contact: Option<String>,
    pub verification_hash: Option<[u8; 32]>,
}

/// Submit metadata for an object
#[hdk_extern]
pub fn submit_metadata(input: SubmitMetadataInput) -> ExternResult<ActionHash> {
    let metadata = ObjectMetadata {
        norad_id: input.norad_id,
        rcs_m2: input.rcs_m2,
        mass_kg: input.mass_kg,
        length_m: input.length_m,
        hard_body_radius_m: input.hard_body_radius_m,
        ballistic_coefficient: input.ballistic_coefficient,
        area_to_mass: input.area_to_mass,
        source: input.source.unwrap_or(DataSourceType::SpaceTrack),
        updated_at: SpaceTimestamp::now(),
    };

    let action_hash = create_entry(&EntryTypes::ObjectMetadata(metadata))?;

    // Link from NORAD anchor to metadata
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, input.norad_id))?;
    create_link(
        norad_anchor,
        action_hash.clone(),
        LinkTypes::ObjectToMetadata,
        LinkTag::new("metadata"),
    )?;

    Ok(action_hash)
}

/// Input for submitting metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitMetadataInput {
    pub norad_id: u32,
    pub rcs_m2: Option<f64>,
    pub mass_kg: Option<f64>,
    pub length_m: Option<f64>,
    pub hard_body_radius_m: Option<f64>,
    pub ballistic_coefficient: Option<f64>,
    pub area_to_mass: Option<f64>,
    pub source: Option<DataSourceType>,
}

// ============================================================================
// Query Functions
// ============================================================================

/// Get an orbital object by its action hash
#[hdk_extern]
pub fn get_object(action_hash: ActionHash) -> ExternResult<Option<OrbitalObject>> {
    let record = get(action_hash, GetOptions::default())?;
    match record {
        Some(r) => {
            let object: OrbitalObject = r
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;
            Ok(Some(object))
        }
        None => Ok(None),
    }
}

/// Get an orbital object by NORAD ID
#[hdk_extern]
pub fn get_object_by_norad_id(norad_id: u32) -> ExternResult<Option<ObjectWithHash>> {
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, norad_id))?;

    let links = get_links_for_base(norad_anchor, LinkTypes::AllObjects, Some("object"))?;

    if let Some(link) = links.first() {
        let action_hash = ActionHash::try_from(link.target.clone())
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid target hash".to_string())))?;

        if let Some(object) = get_object(action_hash.clone())? {
            return Ok(Some(ObjectWithHash {
                action_hash,
                object,
            }));
        }
    }

    Ok(None)
}

/// Object with its action hash
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectWithHash {
    pub action_hash: ActionHash,
    pub object: OrbitalObject,
}

/// List all objects in the catalog
#[hdk_extern]
pub fn list_all_objects(input: ListObjectsInput) -> ExternResult<Vec<ObjectWithHash>> {
    let all_objects_anchor = anchor_hash(ALL_OBJECTS_ANCHOR)?;

    let links = get_links_for_base(all_objects_anchor, LinkTypes::AllObjects, None)?;

    let mut objects: Vec<ObjectWithHash> = Vec::new();
    let limit = input.limit.unwrap_or(100) as usize;

    for link in links.into_iter().take(limit) {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid target hash".to_string())))?;

        if let Some(object) = get_object(action_hash.clone())? {
            // Apply filters
            if let Some(ref filter_type) = input.object_type {
                if &object.object_type != filter_type {
                    continue;
                }
            }
            if let Some(ref filter_status) = input.status {
                if &object.status != filter_status {
                    continue;
                }
            }

            objects.push(ObjectWithHash {
                action_hash,
                object,
            });
        }
    }

    Ok(objects)
}

/// Input for listing objects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListObjectsInput {
    pub object_type: Option<ObjectType>,
    pub status: Option<OperationalStatus>,
    pub limit: Option<u32>,
}

/// List objects by type
#[hdk_extern]
pub fn list_objects_by_type(object_type: ObjectType) -> ExternResult<Vec<ObjectWithHash>> {
    let type_anchor = anchor_hash(&format!("{}:{:?}", OBJECT_TYPE_ANCHOR, object_type))?;

    let links = get_links_for_base(type_anchor, LinkTypes::OrbitTypeIndex, None)?;

    let mut objects: Vec<ObjectWithHash> = Vec::new();

    for link in links {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid target hash".to_string())))?;

        if let Some(object) = get_object(action_hash.clone())? {
            objects.push(ObjectWithHash {
                action_hash,
                object,
            });
        }
    }

    Ok(objects)
}

/// Get the latest TLE for an object
#[hdk_extern]
pub fn get_latest_tle(norad_id: u32) -> ExternResult<Option<TleWithHash>> {
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, norad_id))?;

    let links = get_links_for_base(norad_anchor, LinkTypes::ObjectToTles, None)?;

    // Find the TLE with the most recent epoch (highest micros in tag)
    let mut latest: Option<(i64, ActionHash)> = None;

    for link in links {
        if let Ok(epoch_str) = String::from_utf8(link.tag.0.clone()) {
            if let Ok(epoch_micros) = epoch_str.parse::<i64>() {
                match &latest {
                    None => {
                        latest = Some((epoch_micros, ActionHash::try_from(link.target)
                            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?));
                    }
                    Some((current_max, _)) if epoch_micros > *current_max => {
                        latest = Some((epoch_micros, ActionHash::try_from(link.target)
                            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?));
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some((_, action_hash)) = latest {
        if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
            let tle: TleRecord = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;

            return Ok(Some(TleWithHash { action_hash, tle }));
        }
    }

    Ok(None)
}

/// TLE with its action hash
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TleWithHash {
    pub action_hash: ActionHash,
    pub tle: TleRecord,
}

/// Get TLE history for an object
#[hdk_extern]
pub fn get_tle_history(input: GetTleHistoryInput) -> ExternResult<Vec<TleWithHash>> {
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, input.norad_id))?;

    let links = get_links_for_base(norad_anchor, LinkTypes::ObjectToTles, None)?;

    let mut tles: Vec<(i64, TleWithHash)> = Vec::new();
    let limit = input.limit.unwrap_or(100) as usize;

    for link in links {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?;

        if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
            let tle: TleRecord = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;

            // Apply time filters
            if let Some(start) = &input.start_time {
                if tle.epoch.micros < start.micros {
                    continue;
                }
            }
            if let Some(end) = &input.end_time {
                if tle.epoch.micros > end.micros {
                    continue;
                }
            }

            tles.push((tle.epoch.micros, TleWithHash { action_hash, tle }));
        }
    }

    // Sort by epoch (newest first)
    tles.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(tles.into_iter().take(limit).map(|(_, t)| t).collect())
}

/// Input for getting TLE history
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTleHistoryInput {
    pub norad_id: u32,
    pub start_time: Option<SpaceTimestamp>,
    pub end_time: Option<SpaceTimestamp>,
    pub limit: Option<u32>,
}

/// Get operator claims for an object
#[hdk_extern]
pub fn get_operator_claims(norad_id: u32) -> ExternResult<Vec<OperatorClaimWithHash>> {
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, norad_id))?;

    let links = get_links_for_base(norad_anchor, LinkTypes::ObjectToOperator, None)?;

    let mut claims: Vec<OperatorClaimWithHash> = Vec::new();

    for link in links {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?;

        if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
            let claim: OperatorClaim = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;

            claims.push(OperatorClaimWithHash { action_hash, claim });
        }
    }

    Ok(claims)
}

/// Operator claim with its action hash
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorClaimWithHash {
    pub action_hash: ActionHash,
    pub claim: OperatorClaim,
}

/// Get objects claimed by an operator
#[hdk_extern]
pub fn get_operator_objects(operator: AgentPubKey) -> ExternResult<Vec<OperatorClaimWithHash>> {
    let operator_anchor = anchor_hash(&format!("operator:{}", operator))?;

    let links = get_links_for_base(operator_anchor, LinkTypes::OperatorToObjects, None)?;

    let mut claims: Vec<OperatorClaimWithHash> = Vec::new();

    for link in links {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?;

        if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
            let claim: OperatorClaim = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;

            claims.push(OperatorClaimWithHash { action_hash, claim });
        }
    }

    Ok(claims)
}

/// Get metadata for an object
#[hdk_extern]
pub fn get_object_metadata(norad_id: u32) -> ExternResult<Option<ObjectMetadataWithHash>> {
    let norad_anchor = anchor_hash(&format!("{}:{}", NORAD_INDEX_ANCHOR, norad_id))?;

    let links = get_links_for_base(norad_anchor, LinkTypes::ObjectToMetadata, None)?;

    // Get the most recent metadata
    if let Some(link) = links.last() {
        let action_hash = ActionHash::try_from(link.target.clone())
            .map_err(|_| wasm_error!(WasmErrorInner::Guest("Invalid hash".to_string())))?;

        if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
            let metadata: ObjectMetadata = record
                .entry()
                .to_app_option()
                .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Guest("Entry not found".to_string())))?;

            return Ok(Some(ObjectMetadataWithHash { action_hash, metadata }));
        }
    }

    Ok(None)
}

/// Object metadata with its action hash
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectMetadataWithHash {
    pub action_hash: ActionHash,
    pub metadata: ObjectMetadata,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a deterministic anchor hash from a string
fn anchor_hash(anchor: &str) -> ExternResult<EntryHash> {
    let path = Path::from(anchor);
    path.path_entry_hash()
}

/// Get links for a base with optional tag prefix filter
fn get_links_for_base(
    base: EntryHash,
    link_type: LinkTypes,
    tag_prefix: Option<&str>,
) -> ExternResult<Vec<Link>> {
    // Get the zome info to get the correct zome index
    let zome_info = zome_info()?;

    let query = LinkQuery::new(
        base,
        LinkTypeFilter::single_type(
            zome_info.id,
            LinkType::from(link_type as u8),
        ),
    );

    let mut links = get_links(query, GetStrategy::Local)?;

    // Filter by tag prefix if specified
    if let Some(prefix) = tag_prefix {
        let prefix_bytes = prefix.as_bytes();
        links.retain(|link| link.tag.0.starts_with(prefix_bytes));
    }

    Ok(links)
}

/// Extract epoch from TLE line 1
/// Format: positions 18-32 contain YYDDD.DDDDDDDD
fn extract_tle_epoch(line1: &str) -> ExternResult<SpaceTimestamp> {
    if line1.len() < 32 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "TLE line 1 too short".to_string()
        )));
    }

    let epoch_str = &line1[18..32];
    let epoch_str = epoch_str.trim();

    // Parse year (2 digits)
    let year_2d: i32 = epoch_str[0..2].parse().map_err(|_| {
        wasm_error!(WasmErrorInner::Guest("Cannot parse TLE year".to_string()))
    })?;

    // Y2K handling
    let year = if year_2d < 57 { 2000 + year_2d } else { 1900 + year_2d };

    // Parse day of year (with fractional part)
    let day_of_year: f64 = epoch_str[2..].parse().map_err(|_| {
        wasm_error!(WasmErrorInner::Guest("Cannot parse TLE day".to_string()))
    })?;

    // Convert to Unix timestamp
    let jan1_days = days_since_epoch(year, 1, 1);
    let days = jan1_days + (day_of_year - 1.0);
    let seconds = days * 86400.0;
    let micros = (seconds * 1_000_000.0) as i64;

    Ok(SpaceTimestamp { micros })
}

/// Calculate days since Unix epoch (Jan 1, 1970)
fn days_since_epoch(year: i32, month: u32, day: u32) -> f64 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;

    let jd = day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;

    // Unix epoch is JD 2440587.5
    (jd as f64) - 2440587.5
}

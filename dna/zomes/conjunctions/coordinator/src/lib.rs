//! Conjunctions Coordinator Zome
//!
//! Functions for managing conjunction events, CDMs, and avoidance maneuvers.
//! Includes real-time alert signals for high-risk conjunctions.

use conjunctions_integrity::*;
use hdk::prelude::*;
use mycelix_space_shared::{
    AlertPriority, AlertType, ConjunctionAlert, ConjunctionAssessment, ConjunctionDataMessage,
    ManeuverAlert, ManeuverType as SharedManeuverType, SpaceSignal, SpaceTimestamp,
};

/// Signal types for this zome (used by Holochain's emit_signal)
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum ConjunctionSignal {
    /// Alert for conjunction detection or risk change
    Alert(SpaceSignal),
    /// CDM update notification
    CdmUpdate { event_id: String, version: u32 },
    /// Maneuver announcement
    ManeuverAnnounced { event_id: String, norad_id: u32 },
}

/// Risk level threshold for automatic alerts
const ALERT_THRESHOLD: RiskLevel = RiskLevel::Medium;

/// Create a conjunction event
#[hdk_extern]
pub fn create_conjunction_event(input: CreateEventInput) -> ExternResult<ActionHash> {
    let event = ConjunctionEvent {
        event_id: input.event_id.clone(),
        primary_norad_id: input.primary_norad_id,
        secondary_norad_id: input.secondary_norad_id,
        tca: input.tca.clone(),
        miss_distance_km: input.miss_distance_km,
        max_pc: input.max_pc,
        risk_level: input.risk_level,
        status: EventStatus::Screening,
        created_at: SpaceTimestamp::now(),
        updated_at: SpaceTimestamp::now(),
    };

    let action_hash = create_entry(&EntryTypes::ConjunctionEvent(event.clone()))?;

    // Emit signal if risk level is at or above threshold
    if input.risk_level >= ALERT_THRESHOLD {
        emit_conjunction_alert(&event)?;
    }

    Ok(action_hash)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEventInput {
    pub event_id: String,
    pub primary_norad_id: u32,
    pub secondary_norad_id: u32,
    pub tca: SpaceTimestamp,
    pub miss_distance_km: f64,
    pub max_pc: f64,
    pub risk_level: RiskLevel,
}

/// Update conjunction event risk level (with signal emission)
#[hdk_extern]
pub fn update_conjunction_risk(input: UpdateRiskInput) -> ExternResult<ActionHash> {
    // Get the current event
    let record = get(input.event_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Conjunction event not found".to_string())
    ))?;

    let mut event: ConjunctionEvent = record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize: {:?}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Entry is not a ConjunctionEvent".to_string()
        )))?;

    let previous_risk = event.risk_level;

    // Update the event
    event.risk_level = input.new_risk_level;
    event.max_pc = input.new_pc;
    event.miss_distance_km = input.new_miss_distance_km;
    event.updated_at = SpaceTimestamp::now();

    let action_hash = update_entry(input.event_hash, &event)?;

    // Emit escalation/de-escalation signal if crossing threshold
    if previous_risk < ALERT_THRESHOLD && input.new_risk_level >= ALERT_THRESHOLD {
        // Risk escalated above threshold
        emit_risk_escalation_alert(&event, previous_risk)?;
    } else if input.new_risk_level >= ALERT_THRESHOLD {
        // Already above threshold, emit update
        emit_conjunction_alert(&event)?;
    }

    Ok(action_hash)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRiskInput {
    pub event_hash: ActionHash,
    pub new_risk_level: RiskLevel,
    pub new_pc: f64,
    pub new_miss_distance_km: f64,
}

/// Submit a CDM for a conjunction event
#[hdk_extern]
pub fn submit_cdm(input: SubmitCdmInput) -> ExternResult<ActionHash> {
    let cdm_entry = CdmEntry {
        cdm: input.cdm.clone(),
        version: input.version,
        supersedes: input.supersedes.clone(),
    };

    let action_hash = create_entry(&EntryTypes::Cdm(cdm_entry))?;

    // Emit CDM update signal
    let signal = ConjunctionSignal::CdmUpdate {
        event_id: input.cdm.conjunction_id.clone(),
        version: input.version,
    };
    emit_signal(signal)?;

    Ok(action_hash)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitCdmInput {
    pub cdm: ConjunctionDataMessage,
    pub version: u32,
    pub supersedes: Option<ActionHash>,
}

/// Announce an avoidance maneuver
#[hdk_extern]
pub fn announce_maneuver(input: AnnounceManeuverInput) -> ExternResult<ActionHash> {
    let agent = agent_info()?.agent_initial_pubkey;

    let maneuver = AvoidanceManeuver {
        event_id: input.event_id.clone(),
        norad_id: input.norad_id,
        operator: agent,
        burn_time: input.burn_time.clone(),
        delta_v_ms: input.delta_v_ms,
        direction: input.direction,
        status: ManeuverStatus::Announced,
        created_at: SpaceTimestamp::now(),
    };

    let action_hash = create_entry(&EntryTypes::AvoidanceManeuver(maneuver))?;

    // Emit maneuver announcement signal
    let maneuver_alert = ManeuverAlert {
        alert_type: AlertType::ManeuverRequired,
        priority: AlertPriority::High,
        norad_id: input.norad_id,
        conjunction_primary: None, // Could be populated from event lookup
        conjunction_secondary: None,
        maneuver_type: SharedManeuverType::Combined, // Determine from direction
        delta_v_ms: input.delta_v_ms,
        execution_time: input.burn_time.to_datetime(),
        generated_at: chrono::Utc::now(),
        message: Some(format!(
            "Avoidance maneuver announced for event {}",
            input.event_id
        )),
    };

    let signal = ConjunctionSignal::Alert(SpaceSignal::Maneuver(maneuver_alert));
    emit_signal(signal)?;

    Ok(action_hash)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnounceManeuverInput {
    pub event_id: String,
    pub norad_id: u32,
    pub burn_time: SpaceTimestamp,
    pub delta_v_ms: f64,
    pub direction: [f64; 3],
}

/// Mark maneuver as executed
#[hdk_extern]
pub fn mark_maneuver_executed(input: ManeuverExecutedInput) -> ExternResult<ActionHash> {
    // Get the current maneuver
    let record = get(input.maneuver_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Maneuver not found".to_string())
    ))?;

    let mut maneuver: AvoidanceManeuver = record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize: {:?}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Entry is not an AvoidanceManeuver".to_string()
        )))?;

    // Update status
    maneuver.status = ManeuverStatus::Executed;

    let action_hash = update_entry(input.maneuver_hash, &maneuver)?;

    // Emit executed signal
    let maneuver_alert = ManeuverAlert {
        alert_type: AlertType::ManeuverExecuted,
        priority: AlertPriority::Medium,
        norad_id: maneuver.norad_id,
        conjunction_primary: None,
        conjunction_secondary: None,
        maneuver_type: SharedManeuverType::Combined,
        delta_v_ms: maneuver.delta_v_ms,
        execution_time: maneuver.burn_time.to_datetime(),
        generated_at: chrono::Utc::now(),
        message: Some(format!("Maneuver executed for event {}", maneuver.event_id)),
    };

    let signal = ConjunctionSignal::Alert(SpaceSignal::Maneuver(maneuver_alert));
    emit_signal(signal)?;

    Ok(action_hash)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManeuverExecutedInput {
    pub maneuver_hash: ActionHash,
}

/// Get all high-risk conjunctions
#[hdk_extern]
pub fn get_high_risk_conjunctions(_: ()) -> ExternResult<Vec<ConjunctionEvent>> {
    // Query all conjunction events with risk level >= High
    // In a real implementation, this would use a path or link query
    // For now, return empty - would need proper indexing
    Ok(vec![])
}

/// Query conjunctions by object
#[hdk_extern]
pub fn get_conjunctions_for_object(norad_id: u32) -> ExternResult<Vec<ConjunctionEvent>> {
    // Would query by link or path anchored on the NORAD ID
    Ok(vec![])
}

// =============================================================================
// Internal Helper Functions
// =============================================================================

/// Emit a conjunction alert signal
fn emit_conjunction_alert(event: &ConjunctionEvent) -> ExternResult<()> {
    let assessment = ConjunctionAssessment {
        primary_norad_id: event.primary_norad_id,
        secondary_norad_id: event.secondary_norad_id,
        tca: event.tca.to_datetime(),
        miss_distance_km: event.miss_distance_km,
        relative_velocity_kms: 10.0, // Would come from actual data
        collision_probability: event.max_pc,
        pc_method: mycelix_space_shared::PcMethod::Alfano2D,
        risk_level: convert_risk_level(event.risk_level),
        hard_body_radius_m: 20.0,
        screening_volume_km: 5.0,
    };

    let alert = ConjunctionAlert::new_conjunction(&assessment);
    let signal = ConjunctionSignal::Alert(SpaceSignal::Conjunction(alert));
    emit_signal(signal)?;

    Ok(())
}

/// Emit a risk escalation alert signal
fn emit_risk_escalation_alert(
    event: &ConjunctionEvent,
    previous_risk: RiskLevel,
) -> ExternResult<()> {
    let assessment = ConjunctionAssessment {
        primary_norad_id: event.primary_norad_id,
        secondary_norad_id: event.secondary_norad_id,
        tca: event.tca.to_datetime(),
        miss_distance_km: event.miss_distance_km,
        relative_velocity_kms: 10.0,
        collision_probability: event.max_pc,
        pc_method: mycelix_space_shared::PcMethod::Alfano2D,
        risk_level: convert_risk_level(event.risk_level),
        hard_body_radius_m: 20.0,
        screening_volume_km: 5.0,
    };

    let alert = ConjunctionAlert::risk_escalation(&assessment, convert_risk_level(previous_risk));
    let signal = ConjunctionSignal::Alert(SpaceSignal::Conjunction(alert));
    emit_signal(signal)?;

    Ok(())
}

/// Convert integrity RiskLevel to shared RiskLevel
fn convert_risk_level(level: RiskLevel) -> mycelix_space_shared::RiskLevel {
    match level {
        RiskLevel::Negligible => mycelix_space_shared::RiskLevel::Negligible,
        RiskLevel::Low => mycelix_space_shared::RiskLevel::Low,
        RiskLevel::Medium => mycelix_space_shared::RiskLevel::Medium,
        RiskLevel::High => mycelix_space_shared::RiskLevel::High,
        RiskLevel::Emergency => mycelix_space_shared::RiskLevel::Emergency,
    }
}

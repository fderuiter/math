use super::*;
use oxidize_core::{ModelConfig, ModelState, SimulationModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MockHipaaConfig {
    pub patient_count: usize,
    pub sensitive_data: Vec<u8>,
}

impl ModelConfig for MockHipaaConfig {}

#[derive(Clone, Serialize, Deserialize)]
pub struct MockHipaaState {
    pub processed: bool,
    pub data: Vec<u8>,
}

impl ModelState for MockHipaaState {}

pub struct MockHipaaSimulation {
    config: MockHipaaConfig,
    state: MockHipaaState,
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationError {
    #[error("Simulation failed")]
    Failed,
}

impl SimulationModel for MockHipaaSimulation {
    type Config = MockHipaaConfig;
    type State = MockHipaaState;
    type Error = SimulationError;

    fn initialize(config: Self::Config) -> Result<Self, Self::Error> {
        Ok(Self {
            state: MockHipaaState {
                processed: false,
                data: config.sensitive_data.clone(),
            },
            config,
        })
    }

    fn step(&mut self) -> Result<(), Self::Error> {
        self.state.processed = true;
        for byte in &mut self.state.data {
            *byte = byte.wrapping_add(1); // Some dummy computation
        }
        Ok(())
    }

    fn get_state(&self) -> Self::State {
        self.state.clone()
    }
}

#[test]
fn test_hipaa_compliance_pipeline() {
    let admin = User {
        id: "admin-1".into(),
        role: Role::Admin,
    };
    
    let key = Key::from_slice(b"an example very very secret key.");
    
    let config = MockHipaaConfig {
        patient_count: 100,
        sensitive_data: b"hipaa_sensitive_patient_data_123".to_vec(),
    };
    
    let mut secured_sim = SecuredSimulation::<MockHipaaSimulation>::new(
        &admin,
        config,
        "https://doi.org/10.1234/mock.paper".into(),
        *key,
        None, // In-memory audit log for test
    ).unwrap();
    
    // Attempt step with unauthorized user
    let observer = User {
        id: "obs-1".into(),
        role: Role::Observer,
    };
    assert!(secured_sim.step(&observer).is_err());
    
    // Step with authorized user
    secured_sim.step(&admin).unwrap();
    
    // Get encrypted state
    let encrypted_state = secured_sim.get_encrypted_state(&admin).unwrap();
    
    // Must not be raw data
    assert!(!encrypted_state.windows(4).any(|w| w == b"data"));
    
    // Verify audit log
    let audit_log = secured_sim.get_audit_log();
    assert_eq!(audit_log.len(), 3); // Init, Step, GetState
    assert_eq!(audit_log[0].action, ActionType::Initialize);
    assert_eq!(audit_log[1].action, ActionType::Step);
    assert_eq!(audit_log[2].action, ActionType::GetState);
    
    // Verify execution observability metrics
    assert_eq!(secured_sim.get_progress_count(), 1);
}

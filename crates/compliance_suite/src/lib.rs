#![forbid(unsafe_code)]

use oxidize_core::{ModelConfig, ModelState, SimulationModel};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Researcher,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Initialize,
    Step,
    GetState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub user_id: String,
    pub action: ActionType,
    pub state_hash: String,
    pub theory_link: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Unauthorized access: User role {:?} cannot perform this action", .0)]
    Unauthorized(Role),
    #[error("Encryption error")]
    EncryptionError,
    #[error("Decryption error")]
    DecryptionError,
    #[error("Underlying simulation error: {0}")]
    SimulationError(String),
    #[error("Audit log error: {0}")]
    AuditError(String),
}

pub struct AuditLog {
    entries: Vec<AuditEntry>,
    log_file: Option<File>,
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLog {
    pub fn new() -> Self {
        Self { entries: Vec::new(), log_file: None }
    }

    pub fn with_file(path: PathBuf) -> Result<Self, ComplianceError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ComplianceError::AuditError(e.to_string()))?;
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| ComplianceError::AuditError(e.to_string()))?;
        Ok(Self {
            entries: Vec::new(),
            log_file: Some(file),
        })
    }

    pub fn log(&mut self, entry: AuditEntry) -> Result<(), ComplianceError> {
        if let Some(file) = &mut self.log_file {
            let json = serde_json::to_string(&entry).map_err(|e| ComplianceError::AuditError(e.to_string()))?;
            writeln!(file, "{}", json).map_err(|e| ComplianceError::AuditError(e.to_string()))?;
            file.sync_data().map_err(|e| ComplianceError::AuditError(e.to_string()))?;
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn get_entries(&self) -> &[AuditEntry] {
        &self.entries
    }
}

pub struct SecuredSimulation<M: SimulationModel> {
    model: M,
    audit_log: AuditLog,
    theory_link: String,
    encryption_key: Key,
    progress_count: u64,
}

impl<M: SimulationModel> SecuredSimulation<M> {
    pub fn new(
        user: &User,
        config: M::Config,
        theory_link: String,
        encryption_key: Key,
        log_path: Option<PathBuf>,
    ) -> Result<Self, ComplianceError> {
        if user.role != Role::Admin && user.role != Role::Researcher {
            return Err(ComplianceError::Unauthorized(user.role.clone()));
        }

        let config_bytes = serde_json::to_vec(&config).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&config_bytes);
        let hash = format!("{:x}", hasher.finalize());

        let mut audit_log = if let Some(path) = log_path {
            AuditLog::with_file(path)?
        } else {
            AuditLog::new()
        };

        audit_log.log(AuditEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_id: user.id.clone(),
            action: ActionType::Initialize,
            state_hash: hash,
            theory_link: Some(theory_link.clone()),
        })?;

        let model = M::initialize(config).map_err(|e| ComplianceError::SimulationError(e.to_string()))?;

        Ok(Self {
            model,
            audit_log,
            theory_link,
            encryption_key,
            progress_count: 0,
        })
    }

    pub fn step(&mut self, user: &User) -> Result<(), ComplianceError> {
        if user.role != Role::Admin && user.role != Role::Researcher {
            return Err(ComplianceError::Unauthorized(user.role.clone()));
        }

        self.model.step().map_err(|e| ComplianceError::SimulationError(e.to_string()))?;
        self.progress_count += 1;

        self.audit_log.log(AuditEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_id: user.id.clone(),
            action: ActionType::Step,
            state_hash: "state_omitted_for_step".to_string(),
            theory_link: Some(self.theory_link.clone()),
        })?;

        Ok(())
    }

    pub fn get_encrypted_state(&mut self, user: &User) -> Result<Vec<u8>, ComplianceError>
    where
        M::State: Serialize,
    {
        let state = self.model.get_state();
        let state_bytes = serde_json::to_vec(&state).map_err(|_| ComplianceError::EncryptionError)?;

        let mut hasher = Sha256::new();
        hasher.update(&state_bytes);
        let hash = format!("{:x}", hasher.finalize());

        self.audit_log.log(AuditEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_id: user.id.clone(),
            action: ActionType::GetState,
            state_hash: hash,
            theory_link: Some(self.theory_link.clone()),
        })?;

        let cipher = ChaCha20Poly1305::new(&self.encryption_key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        
        let mut encrypted_data = cipher.encrypt(&nonce, state_bytes.as_ref())
            .map_err(|_| ComplianceError::EncryptionError)?;
            
        // Prepend nonce to the encrypted data
        let mut result = nonce.to_vec();
        result.append(&mut encrypted_data);
        
        Ok(result)
    }
    
    pub fn get_audit_log(&self) -> &[AuditEntry] {
        self.audit_log.get_entries()
    }
    
    pub fn get_progress_count(&self) -> u64 {
        self.progress_count
    }
}

#[cfg(test)]
mod tests;

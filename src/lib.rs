// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-restart: service restart component model.
//!
//! Real implementation: restarts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Restart {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceRestartError {
    EmptyRestartName,
    DuplicateRestart(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceRestartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRestartName => write!(f, "restart name must not be empty"),
            Self::DuplicateRestart(id) => write!(f, "duplicate restart {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceRestartError {}

pub struct ServiceRestart {
    pub name: String,
    restarts: BTreeMap<String, Restart>,
    events: BTreeMap<String, Event>,
}

impl ServiceRestart {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            restarts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_restart(&mut self, item: Restart) -> Result<(), ServiceRestartError> {
        if item.name.trim().is_empty() {
            return Err(ServiceRestartError::EmptyRestartName);
        }
        let id = item.id.clone();
        if self.restarts.contains_key(&id) {
            return Err(ServiceRestartError::DuplicateRestart(id.clone()));
        }
        self.restarts.insert(id, item);
        Ok(())
    }

    pub fn restart_count(&self) -> usize {
        self.restarts.len()
    }

    pub fn get_restart(&self, id: &str) -> Option<&Restart> {
        self.restarts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceRestartError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceRestartError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_restart(id: &str, name: &str) -> Restart {
        Restart {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceRestart::new("ServiceRestart");
        assert_eq!(m.name, "ServiceRestart");
        assert_eq!(m.restart_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_restart() {
        let mut m = ServiceRestart::new("ServiceRestart");
        m.add_restart(sample_restart("a1", "Alpha")).unwrap();
        assert_eq!(m.restart_count(), 1);
        assert!(m.get_restart("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceRestart::new("ServiceRestart");
        assert!(m.add_restart(sample_restart("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceRestart::new("ServiceRestart");
        m.add_restart(sample_restart("x", "One")).unwrap();
        assert!(m.add_restart(sample_restart("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceRestart::new("ServiceRestart");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-security-utility: security utility component model.
//!
//! Real implementation: utilitys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utility {
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
pub enum SecurityUtilityError {
    EmptyUtilityName,
    DuplicateUtility(String),
    EmptyEventSubject,
}

impl fmt::Display for SecurityUtilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUtilityName => write!(f, "utility name must not be empty"),
            Self::DuplicateUtility(id) => write!(f, "duplicate utility {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SecurityUtilityError {}

pub struct SecurityUtility {
    pub name: String,
    utilitys: BTreeMap<String, Utility>,
    events: BTreeMap<String, Event>,
}

impl SecurityUtility {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            utilitys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_utility(&mut self, item: Utility) -> Result<(), SecurityUtilityError> {
        if item.name.trim().is_empty() {
            return Err(SecurityUtilityError::EmptyUtilityName);
        }
        let id = item.id.clone();
        if self.utilitys.contains_key(&id) {
            return Err(SecurityUtilityError::DuplicateUtility(id.clone()));
        }
        self.utilitys.insert(id, item);
        Ok(())
    }

    pub fn utility_count(&self) -> usize {
        self.utilitys.len()
    }

    pub fn get_utility(&self, id: &str) -> Option<&Utility> {
        self.utilitys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SecurityUtilityError> {
        if event.subject.trim().is_empty() {
            return Err(SecurityUtilityError::EmptyEventSubject);
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

    fn sample_utility(id: &str, name: &str) -> Utility {
        Utility {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SecurityUtility::new("SecurityUtility");
        assert_eq!(m.name, "SecurityUtility");
        assert_eq!(m.utility_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_utility() {
        let mut m = SecurityUtility::new("SecurityUtility");
        m.add_utility(sample_utility("a1", "Alpha")).unwrap();
        assert_eq!(m.utility_count(), 1);
        assert!(m.get_utility("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SecurityUtility::new("SecurityUtility");
        assert!(m.add_utility(sample_utility("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SecurityUtility::new("SecurityUtility");
        m.add_utility(sample_utility("x", "One")).unwrap();
        assert!(m.add_utility(sample_utility("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SecurityUtility::new("SecurityUtility");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

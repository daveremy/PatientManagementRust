use uuid::Uuid;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DomainError {
    PatientDischargedError(Uuid),
    PatientTransferredError(Uuid, u32),
}

impl Error for DomainError { }

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::PatientDischargedError(id) => write!(f, "Unable to discharge patient with id {}.  This patient is not currently admitted", id ),
            DomainError::PatientTransferredError(id, ward) => write!(f, "Unable to transfer patient with id {} to ward {}.  This patient is not currently admitted", id, ward),
        }
    }
}
use uuid::Uuid;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum EncounterEvent {
    PatientAdmitted(PatientAdmitted),
    PatientDischarged(PatientDischarged),
    PatientTransferred(PatientTransferred),
}

#[derive(Debug, PartialEq)]
pub struct PatientAdmitted {
    pub patient_id: Uuid,
    pub patient_name: String,
    pub age_in_years: u32,
    pub ward: u32,
}

#[derive(Debug, PartialEq)]
pub struct PatientDischarged {
    pub patient_id: Uuid,
}

#[derive(Debug, PartialEq)]
pub struct PatientTransferred {
    pub patient_id: Uuid,
    pub ward: u32,
}
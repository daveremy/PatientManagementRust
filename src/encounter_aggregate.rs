use uuid::Uuid;
use crate::aggregate_root::AggregateRoot;
use crate::encounter_event::{EncounterEvent, PatientAdmitted, PatientDischarged, PatientTransferred};
use crate::domain_error::DomainError;

#[derive(Debug, Default)]
pub struct Encounter {
    pub patient_id: Option<Uuid>,
    pub patient_name: Option<String>,
    pub age_in_years: Option<u32>,
    pub ward: Option<u32>,
    pub currently_admitted: Option<bool>, 
    version: i32,
    events: Vec<EncounterEvent>,
}

impl AggregateRoot for Encounter {

    fn events(&self) -> &Vec<EncounterEvent> {
        &self.events
    }

    fn clear_events(&mut self) {
        self.events.clear();
    }

    fn id(&self) -> Uuid {
       self.patient_id.unwrap() 
    }

    fn version(&self) -> i32 {
        self.version
    }

    fn apply(&mut self, e: &EncounterEvent) {
        match e {
            EncounterEvent::PatientAdmitted(patient_admitted)  => self.when_patient_admitted(patient_admitted),
            EncounterEvent::PatientDischarged(_) => self.when_patient_discharged(),
            EncounterEvent::PatientTransferred(patient_transferred) => self.when_patient_transferred(patient_transferred),
        }
    }

    fn raise(&mut self, e: EncounterEvent) {
        self.apply(&e);
        self.events.push(e);
    }

}

#[allow(dead_code)]
impl Encounter {
    fn new(patient_id: Uuid, patient_name: String, age_in_years: u32, ward: u32) -> Self {
        let mut encounter = Encounter { version: -1, ..Default::default() };
        encounter.raise(EncounterEvent::PatientAdmitted(PatientAdmitted{
            patient_id,
            patient_name,
            age_in_years,
            ward,
        }));
        encounter
    }
    
    fn discharge_patient(&mut self) -> Result<(), DomainError> {
        match self.currently_admitted {
            Some(true) => {
                self.raise(EncounterEvent::PatientDischarged(PatientDischarged { patient_id: self.patient_id.unwrap() }));
                Ok(())
            },
            Some(false) => Err(DomainError::PatientDischargedError(self.patient_id.unwrap())),
            None => panic!("Encounter not initialized"),
        }
    }

    fn transfer(&mut self, ward: u32) -> Result<(), DomainError> {
        match self.currently_admitted {
            Some(true) => {
                self.raise(EncounterEvent::PatientTransferred(PatientTransferred { patient_id: self.patient_id.unwrap(), ward }));
                Ok(())
            },
            Some(false) => Err(DomainError::PatientTransferredError(self.patient_id.unwrap(), ward)),
            None => panic!("Encounter not initialized"),
        }
    }

    fn when_patient_admitted(&mut self, e: &PatientAdmitted) {
        self.patient_id = Some(e.patient_id);
        self.currently_admitted = Some(true);
        self.patient_name = Some(e.patient_name.to_string());
        self.age_in_years = Some(e.age_in_years);
        self.ward = Some(e.ward);
    }

    fn when_patient_discharged(&mut self) {
        self.currently_admitted = Some(false);
    }

    fn when_patient_transferred(&mut self, e: &PatientTransferred) {
        self.ward = Some(e.ward); 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_admit_patient() {
        let encounter = admit_patient();
        let expected_events = vec![
            EncounterEvent::PatientAdmitted(PatientAdmitted {
                patient_id: encounter.patient_id.unwrap(),
                patient_name: encounter.patient_name.as_ref().unwrap().to_string(),
                age_in_years: encounter.age_in_years.unwrap(),
                ward: encounter.ward.unwrap(),
            }),
        ];
        assert_only_these(&expected_events, &encounter.events()); 
    }

    #[test]
    fn discharge_patient() {
        let mut encounter = admit_patient();
        encounter.discharge_patient().unwrap();
        let expected_events = vec![
            EncounterEvent::PatientAdmitted(PatientAdmitted {
                patient_id: encounter.patient_id.unwrap(),
                patient_name: encounter.patient_name.as_ref().unwrap().to_string(),
                age_in_years: encounter.age_in_years.unwrap(),
                ward: encounter.ward.unwrap(),
            }),
            EncounterEvent::PatientDischarged(PatientDischarged {
                patient_id: encounter.patient_id.unwrap(),
            })
        ];
        assert_only_these(&expected_events, &encounter.events()); 
    }

    #[test]
    fn discharge_already_discharged_patient() {
        let mut encounter = admit_patient();
        encounter.discharge_patient().unwrap();
        match encounter.discharge_patient() {
            Err(DomainError::PatientDischargedError(_)) => (),
            _ => assert!(false, "Patient already discharged. Expected Patient Discharge Error"),
        }
    }

    #[test]
    fn transfer_patient() {
        let mut encounter = admit_patient();
        let original_ward = encounter.ward.unwrap();
        encounter.transfer(22).unwrap();
        let expected_events = vec![
            EncounterEvent::PatientAdmitted(PatientAdmitted {
                patient_id: encounter.patient_id.unwrap(),
                patient_name: encounter.patient_name.as_ref().unwrap().to_string(),
                age_in_years: encounter.age_in_years.unwrap(),
                ward: original_ward,
            }),
            EncounterEvent::PatientTransferred(PatientTransferred {
                patient_id: encounter.patient_id.unwrap(),
                ward: encounter.ward.unwrap(),
            })
        ];
        assert_only_these(&expected_events, &encounter.events());
    }

    #[test]
    fn transfer_discharged_patient() {
        let mut encounter = admit_patient();
        encounter.discharge_patient().unwrap();
        match encounter.transfer(22) {
            Err(DomainError::PatientTransferredError(_, _)) => (),
            _ => assert!(false, "Patient already discharged. Expected Patient Transfer Error"),
        }
    }

    fn admit_patient() -> Encounter {
        let id = Uuid::new_v4();
        let encounter = Encounter::new(id, "Fred Jones".to_string(), 32, 45 );
        encounter
    }

    // Test Helpers

    fn assert_only_these(expected_events: &Vec<EncounterEvent>, actual_events: &Vec<EncounterEvent>) {
        let matching = expected_events.iter().zip(actual_events.iter()).filter(|&(a, b)| a == b).count();
        if matching != expected_events.len() {
            println!("Expected Events ****");
            print_events(expected_events);
            println!("Actual Events ****");
            print_events(actual_events);
            assert!(false, "Expected events do not match actual events");
        }
    }

    fn print_events(events: &Vec<EncounterEvent>) {
        for (i, e) in events.iter().enumerate() {
            println!("{}: {:#?}", i, e);
        }
    }
}
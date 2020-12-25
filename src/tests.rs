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
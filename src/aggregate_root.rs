
use uuid::Uuid;
use crate::encounter_event::EncounterEvent;

pub trait AggregateRoot {
    fn events(&self) -> &Vec<EncounterEvent>;
    fn id(&self) -> Uuid;
    fn clear_events(&mut self);
    fn apply(&mut self, e: &EncounterEvent);
    fn get_version(&self) -> i32;    
    fn raise(&mut self, e: EncounterEvent); 
}


use crate::encounter_event::EncounterEvent;
use uuid::Uuid;

pub trait AggregateRoot {
    fn events(&self) -> &Vec<EncounterEvent>;
    fn id(&self) -> Uuid;
    fn clear_events(&mut self);
    fn apply(&mut self, e: &EncounterEvent);
    fn version(&self) -> i32;    
    fn raise(&mut self, e: EncounterEvent); 
}

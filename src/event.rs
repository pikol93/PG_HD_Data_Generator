use std::cmp::Ordering;
use chrono::{DateTime, Utc};
use crate::person::Policeman;

#[derive(Debug, Copy, Clone)]
pub enum EventAction {
    PolicemanEmployment,
    PolicemanResignation,
    Report,
    SendPatrol(usize),
    FinishedPatrol(usize),
    Snapshot(&'static str, bool),
    PolicemanLastNameChange,
}


#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub time: DateTime<Utc>,
    pub action: EventAction,
}

impl Event {
    pub fn from_policeman_resignation_event(policeman: &Policeman) -> Event {
        Event {
            time: policeman.resignment_date,
            action: EventAction::PolicemanResignation,
        }
    }
}

impl Eq for Event {}

impl PartialEq<Self> for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl PartialOrd<Self> for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
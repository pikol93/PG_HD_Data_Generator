use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::person::Person;
use crate::string_occurrences::StringOccurrences;

const REPORT_TYPES_STRING: &str = include_str!("../data/report_types.txt");

static REPORT_TYPE_ENTRIES: Lazy<StringOccurrences> = Lazy::new(|| StringOccurrences::from_str(REPORT_TYPES_STRING));

#[derive(Debug, Copy, Clone)]
pub struct Report {
    pub id: usize,
    pub report_type: &'static str,
    pub time: DateTime<Utc>,
    pub reporter: Person,
    pub place_id: usize,
}

impl Report {
    pub fn generate_with_time_and_id(generator: &mut ThreadRng, time: DateTime<Utc>, max_place_id: usize, id: usize) -> Self {
        // The ID related to the reporter does not need to be unique
        const PERSON_ID: usize = 0;

        let place_id = generator.gen_range(0..max_place_id);

        Self {
            id,
            report_type: REPORT_TYPE_ENTRIES.get_random_entry(generator),
            time,
            reporter: Person::generate_with_id(generator, PERSON_ID),
            place_id,
        }
    }
}
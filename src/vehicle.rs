use crate::string_occurrences::StringOccurrences;
use once_cell::sync::Lazy;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;
use std::iter::Iterator;
use std::rc::Rc;

const VEHICLE_MODELS_STRING: &str = include_str!("../data/vehicle_models.txt");
const REGISTRATION_PLATE_CODES_STRING: &str = include_str!("../data/registration_plate_codes.txt");

const MANUFACTURE_YEAR_MIN: u32 = 2005;
const MANUFACTURE_YEAR_MAX: u32 = 2020;
const EXPECTED_REGISTRATION_PLATE_LENGTH: usize = 8;
const DEFAULT_SEAT_COUNT: u32 = 5;
static REGISTRATION_PLATE_ALLOWED_CHARACTERS: Lazy<Vec<char>> =
    Lazy::new(|| ('0'..='9').chain('A'..='Z').collect::<Vec<_>>());

static VEHICLE_MODELS_ENTRIES: Lazy<StringOccurrences> =
    Lazy::new(|| StringOccurrences::from_str(VEHICLE_MODELS_STRING));
static REGISTRATION_PLATE_CODES_ENTRIES: Lazy<StringOccurrences> =
    Lazy::new(|| StringOccurrences::from_str(REGISTRATION_PLATE_CODES_STRING));

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VehicleState {
    Available,
    Occupied,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub model: &'static str,
    pub registration_plate: Rc<String>,
    pub manufacture_year: u32,
    pub seat_count: u32,
    pub state: VehicleState,
    pub vehicle_type: &'static str,
}

impl Vehicle {
    pub fn generate_with_id(generator: &mut ThreadRng, id: usize) -> Self {
        Self {
            id,
            model: VEHICLE_MODELS_ENTRIES.get_random_entry(generator),
            registration_plate: Rc::new(generate_registration_plate(generator)),
            manufacture_year: generator.gen_range(MANUFACTURE_YEAR_MIN..MANUFACTURE_YEAR_MAX),
            seat_count: DEFAULT_SEAT_COUNT,
            state: VehicleState::Available,
            vehicle_type: "terenowy",
        }
    }
}

fn generate_registration_plate(generator: &mut ThreadRng) -> String {
    let mut registration_plate = String::with_capacity(EXPECTED_REGISTRATION_PLATE_LENGTH);
    let a = REGISTRATION_PLATE_CODES_ENTRIES.get_random_entry(generator);
    registration_plate.push_str(a);
    registration_plate.push(' ');

    for _ in 0..(EXPECTED_REGISTRATION_PLATE_LENGTH - registration_plate.len()) {
        let registration_plate_char = REGISTRATION_PLATE_ALLOWED_CHARACTERS
            .choose(generator)
            .unwrap();
        registration_plate.push(*registration_plate_char);
    }

    registration_plate
}

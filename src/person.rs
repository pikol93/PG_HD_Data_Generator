use chrono::{DateTime, Datelike, Days, TimeZone, Timelike, Utc};
use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::string_occurrences::StringOccurrences;

const FIRST_NAMES_STRING: &str = include_str!("../data/first_names.txt");
const LAST_NAMES_STRING: &str = include_str!("../data/last_names.txt");
const RANKS_STRING: &str = include_str!("../data/ranks.txt");
const PHONE_NUMBER_MIN: u64 = 100000000;
const PHONE_NUMBER_MAX: u64 = 999999999;
const PESEL_SUFFIX_MAX: u64 = 99999;
const PESEL_SUFFIX_MIN: u64 = 10000;
const SERVICE_NUMBER_MIN: u32 = 100000;
const SERVICE_NUMBER_MAX: u32 = 999999;
const MIN_DAYS_AFTER_BIRTH_TO_EMPLOYMENT: u64 = 7670; // 21 years
const MAX_DAYS_AFTER_BIRTH_TO_EMPLOYMENT: u64 = 12783; // 35 years
const MIN_EMPLOYMENT_DURATION: u64 = 365; // 1 year
const MAX_EMPLOYMENT_DURATION: u64 = 7305; // 20 years

static MIN_BIRTH_DATE: Lazy<u64> = Lazy::new(|| {
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
        .unwrap()
        .timestamp() as u64
});
static MAX_BIRTH_DATE: Lazy<u64> = Lazy::new(|| {
    Utc.with_ymd_and_hms(1995, 12, 1, 0, 0, 0)
        .unwrap()
        .timestamp() as u64
});
static FIRST_NAMES_ENTRIES: Lazy<StringOccurrences> =
    Lazy::new(|| StringOccurrences::from_str(FIRST_NAMES_STRING));
static LAST_NAMES_ENTRIES: Lazy<StringOccurrences> =
    Lazy::new(|| StringOccurrences::from_str(LAST_NAMES_STRING));
static RANK_ENTRIES: Lazy<StringOccurrences> =
    Lazy::new(|| StringOccurrences::from_str(RANKS_STRING));

#[derive(Debug, Copy, Clone)]
pub struct Person {
    pub id: usize,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub birth_date: DateTime<Utc>,
    pub phone_number: u64,
    pub pesel_number: u64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PolicemanState {
    Available,
    Occupied,
    Resigned,
}

#[derive(Debug, Copy, Clone)]
pub struct Policeman {
    pub person: Person,
    pub state: PolicemanState,
    pub service_number: u32,
    pub rank: &'static str,
    pub employment_date: DateTime<Utc>,
    pub resignment_date: DateTime<Utc>,
}

impl Person {
    pub fn generate_with_id(generator: &mut ThreadRng, id: usize) -> Self {
        let birth_date = generate_birth_date(generator);
        let pesel = birth_date_to_pesel(generator, &birth_date);
        Person {
            id,
            first_name: FIRST_NAMES_ENTRIES.get_random_entry(generator),
            last_name: LAST_NAMES_ENTRIES.get_random_entry(generator),
            birth_date,
            phone_number: generator.gen_range(PHONE_NUMBER_MIN..PHONE_NUMBER_MAX),
            pesel_number: pesel,
        }
    }

    pub fn change_to_random_surname(&mut self, generator: &mut ThreadRng) {
        let new_name = LAST_NAMES_ENTRIES.get_random_entry(generator);
        println!(
            "Changed last name of person {}: {} -> {}",
            self.pesel_number, self.last_name, new_name
        );
        self.last_name = new_name;
    }
}

impl Policeman {
    pub fn generate_with_id(generator: &mut ThreadRng, id: usize) -> Self {
        let person = Person::generate_with_id(generator, id);
        let employment_date =
            generate_employment_date_from_birth_date(generator, &person.birth_date);
        let resignment_date =
            generate_resignation_date_from_employment_date(generator, &employment_date);

        Self {
            service_number: generator.gen_range(SERVICE_NUMBER_MIN..SERVICE_NUMBER_MAX),
            person,
            state: PolicemanState::Available,
            rank: RANK_ENTRIES.get_random_entry(generator),
            employment_date,
            resignment_date,
        }
    }

    pub fn generate_just_employed_with_id(
        generator: &mut ThreadRng,
        employment_date: &DateTime<Utc>,
        id: usize,
    ) -> Self {
        let mut person = Person::generate_with_id(generator, id);
        person.birth_date = generate_birth_date_from_employment_date(generator, employment_date);
        let resignment_date =
            generate_resignation_date_from_employment_date(generator, employment_date);

        Self {
            service_number: generator.gen_range(SERVICE_NUMBER_MIN..SERVICE_NUMBER_MAX),
            person,
            state: PolicemanState::Available,
            rank: RANK_ENTRIES.get_random_entry(generator),
            employment_date: *employment_date,
            resignment_date,
        }
    }
}

fn generate_birth_date(generator: &mut ThreadRng) -> DateTime<Utc> {
    let min_birth_date = *Lazy::force(&MIN_BIRTH_DATE);
    let max_birth_date = *Lazy::force(&MAX_BIRTH_DATE);
    let timestamp = generator.gen_range(min_birth_date..max_birth_date);
    Utc.timestamp_opt(timestamp as i64, 0)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

fn generate_birth_date_from_employment_date(
    generator: &mut ThreadRng,
    birth_date: &DateTime<Utc>,
) -> DateTime<Utc> {
    let days_after_birth =
        generator.gen_range(MIN_DAYS_AFTER_BIRTH_TO_EMPLOYMENT..MAX_DAYS_AFTER_BIRTH_TO_EMPLOYMENT);
    birth_date
        .checked_sub_days(Days::new(days_after_birth))
        .unwrap()
}

fn generate_employment_date_from_birth_date(
    generator: &mut ThreadRng,
    birth_date: &DateTime<Utc>,
) -> DateTime<Utc> {
    let days_after_birth =
        generator.gen_range(MIN_DAYS_AFTER_BIRTH_TO_EMPLOYMENT..MAX_DAYS_AFTER_BIRTH_TO_EMPLOYMENT);
    birth_date
        .checked_add_days(Days::new(days_after_birth))
        .unwrap()
}

fn generate_resignation_date_from_employment_date(
    generator: &mut ThreadRng,
    employment_date: &DateTime<Utc>,
) -> DateTime<Utc> {
    let days_after_birth = generator.gen_range(MIN_EMPLOYMENT_DURATION..MAX_EMPLOYMENT_DURATION);
    employment_date
        .checked_add_days(Days::new(days_after_birth))
        .unwrap()
}

fn birth_date_to_pesel(generator: &mut ThreadRng, birth_date: &DateTime<Utc>) -> u64 {
    let mut output = birth_date.year() as u64 % 100;
    output *= 100;
    output += birth_date.month() as u64;
    output *= 100;
    output += birth_date.day() as u64;
    output *= 100000;
    output += generator.gen_range(PESEL_SUFFIX_MIN..PESEL_SUFFIX_MAX);
    return output;
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use rand::thread_rng;

    use crate::person::Policeman;

    #[test]
    fn should_generate_policeman() {
        let mut generator = thread_rng();
        let current_date = Utc::now();
        for _ in 0..100 {
            let a = Policeman::generate_with_id(&mut generator, 0);
            dbg!(a);
        }
    }
}

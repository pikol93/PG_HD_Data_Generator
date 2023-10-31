use rand::prelude::ThreadRng;
use rand::RngCore;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct StringOccurrencesEntry {
    entry: &'static str,
    occurrences: u32,
}

#[derive(Debug, Clone)]
pub struct StringOccurrences {
    total_occurrences: u32,
    entries: Vec<StringOccurrencesEntry>,
}

impl StringOccurrences {
    pub fn from_str(data: &'static str) -> Self {
        let entries = data
            .split('\n')
            .filter_map(|line| {
                let mut line_split = line.split(',');
                let Some(entry) = line_split.next() else {
                    return None;
                };
                let Some(occurrences_string) = line_split.next() else {
                    return None;
                };

                let Ok(occurrences) = u32::from_str(occurrences_string) else {
                    panic!("The second column should be convertible to an u32. Found string \"{}\" in data:\n{}", occurrences_string, data);
                };

                Some(StringOccurrencesEntry { entry, occurrences })
            })
            .collect::<Vec<_>>();

        let total_occurrences = entries.iter().map(|entry| entry.occurrences).sum();

        Self {
            total_occurrences,
            entries,
        }
    }

    pub fn get_random_entry(self: &Self, generator: &mut ThreadRng) -> &'static str {
        let value = generator.next_u32() % self.total_occurrences;
        let mut counter = 0;
        for item in &self.entries {
            counter += item.occurrences;
            if value <= counter {
                return item.entry;
            }
        }

        panic!("This should not happen, as counter should always at some point reach the value.");
    }
}

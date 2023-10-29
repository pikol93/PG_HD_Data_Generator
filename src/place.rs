use once_cell::sync::Lazy;

const PLACES_STRING: &str = include_str!("../data/places.txt");

static PLACES: Lazy<Vec<Place>> = Lazy::new(|| create_places_from_str(PLACES_STRING));

#[derive(Debug, Copy, Clone)]
pub struct Place {
    pub id: usize,
    pub city: &'static str,
    pub street: &'static str,
}

pub fn get_all_places() -> &'static Vec<Place> {
    Lazy::force(&PLACES)
}

fn create_places_from_str(data: &'static str) -> Vec<Place> {
    let mut counter = 0;
    data
        .split('\n')
        .filter_map(|line| {
            let mut line_split = line.split(',');
            let Some(city) = line_split.next() else {
                return None;
            };
            let Some(street) = line_split.next() else {
                return None;
            };

            let place = Some(Place { id: counter, city, street });
            counter += 1;
            place
        })
        .collect()
}
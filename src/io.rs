use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use crate::patrol::Patrol;
use crate::person::Policeman;
use crate::place::Place;
use crate::report::Report;
use crate::vehicle::Vehicle;

const PLACES_OUTPUT_DIRECTORY: &str = "output/";
const PLACES_OUTPUT_FILE: &str = "places.csv";
const REPORTS_OUTPUT_FILE: &str = "reports.csv";
const POLICEMEN_DB_OUTPUT_FILE: &str = "policemen_db.csv";
const POLICEMEN_CSV_OUTPUT_FILE: &str = "policemen_csv.csv";
const VEHICLE_DB_OUTPUT_FILE: &str = "vehicle_db.csv";
const VEHICLE_CSV_OUTPUT_FILE: &str = "vehicle_csv.csv";
const PATROLS_OUTPUT_FILE: &str = "patrols.csv";
const POLICEMEN_PATROLS_OUTPUT_FILE: &str = "policemen_patrols.csv";
const COLUMN_DELIMITER: &str = ",";

pub fn write_places_to_file(snapshot_name: &str, places: &[Place]) {
    let mut file = create_file(snapshot_name, PLACES_OUTPUT_FILE);

    places
        .iter()
        .for_each(|place| {
            let items = &[
                place.id.to_string(),
                place.city.to_string(),
                place.street.to_string()
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_reports_to_file(snapshot_name: &str, reports: &[Report]) {
    let mut file = create_file(snapshot_name, REPORTS_OUTPUT_FILE);

    reports
        .iter()
        .for_each(|report| {
            let items = &[
                report.id.to_string(),
                report.place_id.to_string(),
                report.time.to_string(),
                report.report_type.to_string(),
                report.reporter.phone_number.to_string(),
                report.reporter.first_name.to_string(),
                report.reporter.last_name.to_string()
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_database_policemen_to_file(snapshot_name: &str, policemen: &[Policeman]) {
    let mut file = create_file(snapshot_name, POLICEMEN_DB_OUTPUT_FILE);

    policemen
        .iter()
        .for_each(|policeman| {
            let items = &[
                policeman.person.id.to_string(),
                policeman.service_number.to_string()
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_csv_policemen_to_file(snapshot_name: &str, policemen: &[Policeman]) {
    let mut file = create_file(snapshot_name, POLICEMEN_CSV_OUTPUT_FILE);

    policemen
        .iter()
        .for_each(|policeman| {
            let items = &[
                policeman.service_number.to_string(),
                policeman.person.birth_date.to_string(),
                policeman.employment_date.to_string(),
                policeman.person.first_name.to_string(),
                policeman.person.last_name.to_string(),
                policeman.person.pesel_number.to_string(),
                policeman.resignment_date.to_string()
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_csv_vehicle_to_file(snapshot_name: &str, vehicles: &[Vehicle]) {
    let mut file = create_file(snapshot_name, VEHICLE_CSV_OUTPUT_FILE);

    vehicles
        .iter()
        .for_each(|vehicle| {
            let items = &[
                vehicle.registration_plate.to_string(),
                vehicle.model.to_string(),
                vehicle.manufacture_year.to_string(),
                vehicle.seat_count.to_string(),
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_database_vehicle_to_file(snapshot_name: &str, vehicles: &[Vehicle]) {
    let mut file = create_file(snapshot_name, VEHICLE_DB_OUTPUT_FILE);

    vehicles
        .iter()
        .for_each(|vehicle| {
            let items = &[
                vehicle.id.to_string(),
                vehicle.registration_plate.to_string(),
                vehicle.vehicle_type.to_string(),
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_patrols_to_file(snapshot_name: &str, patrols: &[Patrol]) {
    let mut file = create_file(snapshot_name, PATROLS_OUTPUT_FILE);

    patrols
        .iter()
        .for_each(|item| {
            let items = &[
                item.id.to_string(),
                item.vehicle_id.to_string(),
                item.report_id.to_string(),
                item.sending_time.to_string(),
                item.arrival_time.to_string(),
                item.finish_time.to_string(),
            ];
            write_to_file(&mut file, items);
        });
}

pub fn write_policeman_patrol_to_file(snapshot_name: &str, patrols: &[Patrol]) {
    let mut file = create_file(snapshot_name, POLICEMEN_PATROLS_OUTPUT_FILE);

    patrols
        .iter()
        .flat_map(|item| item
            .policemen_ids
            .iter()
            .map(|policeman_id| (item.id, policeman_id)))
        .for_each(|(patrol_id, policeman_id)| {
            let items = &[
                patrol_id.to_string(),
                policeman_id.to_string(),
            ];
            write_to_file(&mut file, items);
        });
}

fn create_file(snapshot_name: &str, file_suffix: &str) -> File {
    let path_string = PLACES_OUTPUT_DIRECTORY
        .to_owned()
        .add(snapshot_name)
        .add(file_suffix);
    let path = Path::new(&path_string);
    File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn write_to_file(file: &mut File, items: &[String]) {
    file.write_all(items.get(0).unwrap().to_string().as_bytes()).unwrap();

    items.iter()
        .skip(1)
        .for_each(|item| {
            file.write_all(COLUMN_DELIMITER.as_bytes()).unwrap();
            file.write_all(item.as_bytes()).unwrap();
        });

    file.write_all("\n".as_bytes()).unwrap();
}

use crate::event::{Event, EventAction};
use crate::io::{
    write_csv_policemen_to_file, write_csv_vehicle_to_file, write_database_policemen_to_file,
    write_database_vehicle_to_file, write_patrols_to_file, write_places_to_file,
    write_policeman_patrol_to_file, write_reports_to_file,
};
use crate::patrol::Patrol;
use crate::person::{Policeman, PolicemanState};
use crate::place::get_all_places;
use crate::report::Report;
use chrono::{Days, Duration, TimeZone, Utc};
use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng};
use sorted_vec::SortedVec;

use crate::vehicle::{Vehicle, VehicleState};

mod event;
mod io;
mod patrol;
mod person;
mod place;
mod report;
mod string_occurrences;
mod vehicle;

// 10 minutes
const MIN_SECONDS_BETWEEN_REPORTS: i64 = 600;
// 40 minutes
const MAX_SECONDS_BETWEEN_REPORTS: i64 = 2400;
// 5 minutes
const MIN_REPORT_TO_SENDING_SECONDS: i64 = 300;
// 15 minutes
const MAX_REPORT_TO_SENDING_SECONDS: i64 = 900;
const POLICEMEN_COUNT: usize = 80;
const VEHICLES_COUNT: usize = 60;
const POLICEMAN_LAST_NAME_CHANGE_EVENTS_COUNT: i64 = 20;
const TWO_PATROLS_CHANCE: f64 = 0.1;

fn main() {
    let data_start_date = Utc.with_ymd_and_hms(2015, 6, 1, 0, 0, 0).unwrap();
    let snapshots = [
        (
            "SNAPSHOT_A_",
            false,
            Utc.with_ymd_and_hms(2016, 6, 5, 0, 0, 0).unwrap(),
        ),
        (
            "SNAPSHOT_B_",
            true,
            Utc.with_ymd_and_hms(2017, 6, 10, 0, 0, 0).unwrap(),
        ),
    ];

    let mut generator = thread_rng();
    let places = get_all_places();
    let mut policemen = (0..POLICEMEN_COUNT)
        .map(|index| {
            Policeman::generate_just_employed_with_id(&mut generator, &data_start_date, index)
        })
        .collect::<Vec<_>>();
    let mut vehicles = (0..VEHICLES_COUNT)
        .map(|index| Vehicle::generate_with_id(&mut generator, index))
        .collect::<Vec<_>>();
    let mut reports = vec![];
    let mut patrols = vec![];

    let resignation_events = policemen
        .iter()
        .map(Event::from_policeman_resignation_event);

    let snapshot_events = snapshots
        .iter()
        .map(|(snapshot_name, is_terminal, snapshot_date)| Event {
            time: *snapshot_date,
            action: EventAction::Snapshot(snapshot_name, *is_terminal),
        });

    let last_name_change_events = (0..POLICEMAN_LAST_NAME_CHANGE_EVENTS_COUNT).map(|i| Event {
        time: snapshots
            .get(0)
            .unwrap()
            .2
            .checked_add_signed(Duration::days(i))
            .unwrap(),
        action: EventAction::PolicemanLastNameChange,
    });

    let mut events = resignation_events
        .chain(snapshot_events)
        .chain(last_name_change_events)
        .fold(SortedVec::new(), |mut vector, event| {
            vector.push(event);
            vector
        });

    events.push(Event {
        time: data_start_date,
        action: EventAction::Report,
    });

    loop {
        let Some(event) = events.get(0) else {
            break;
        };

        let event = *event;
        events.remove_index(0);
        let current_time = event.time;

        match event.action {
            EventAction::PolicemanEmployment => {
                let policeman = Policeman::generate_just_employed_with_id(
                    &mut generator,
                    &current_time,
                    policemen.len(),
                );
                policemen.push(policeman);
                let event = Event::from_policeman_resignation_event(&policeman);
                events.push(event);
            }
            EventAction::PolicemanResignation => {
                let next_policeman_employment_date =
                    current_time.checked_add_days(Days::new(7)).unwrap();
                let event = Event {
                    time: next_policeman_employment_date,
                    action: EventAction::PolicemanEmployment,
                };
                events.push(event);
            }
            EventAction::Report => {
                let report_id = reports.len();
                let report = Report::generate_with_time_and_id(
                    &mut generator,
                    current_time,
                    places.len(),
                    reports.len(),
                );
                reports.push(report);

                let time_before_sending_patrol = generator
                    .gen_range(MIN_REPORT_TO_SENDING_SECONDS..MAX_REPORT_TO_SENDING_SECONDS);
                let next_report_time = current_time
                    .checked_add_signed(Duration::seconds(time_before_sending_patrol))
                    .unwrap();

                let required_patrols = if generator.gen_bool(TWO_PATROLS_CHANCE) {
                    2
                } else {
                    1
                };
                for _ in 0..required_patrols {
                    events.push(Event {
                        time: next_report_time,
                        action: EventAction::SendPatrol(report_id),
                    });
                }

                let time_between_reports =
                    generator.gen_range(MIN_SECONDS_BETWEEN_REPORTS..MAX_SECONDS_BETWEEN_REPORTS);
                let next_report_time = current_time
                    .checked_add_signed(Duration::seconds(time_between_reports))
                    .unwrap();
                let event = Event {
                    time: next_report_time,
                    action: EventAction::Report,
                };
                events.push(event);
            }
            EventAction::SendPatrol(report_id) => {
                const TARGET_POLICEMEN_COUNT: usize = 2;
                let mut chosen_policemen = policemen
                    .iter_mut()
                    .filter(|policeman| policeman.state == PolicemanState::Available)
                    .choose_multiple(&mut generator, TARGET_POLICEMEN_COUNT);

                if chosen_policemen.len() < TARGET_POLICEMEN_COUNT {
                    let time_before_sending_patrol = generator
                        .gen_range(MIN_REPORT_TO_SENDING_SECONDS..MAX_REPORT_TO_SENDING_SECONDS);
                    let next_report_time = current_time
                        .checked_add_signed(Duration::seconds(time_before_sending_patrol))
                        .unwrap();
                    events.push(Event {
                        time: next_report_time,
                        action: EventAction::SendPatrol(report_id),
                    });
                    continue;
                }

                let chosen_vehicle = vehicles
                    .iter_mut()
                    .filter(|vehicle| vehicle.state == VehicleState::Available)
                    .choose(&mut generator);

                let Some(chosen_vehicle) = chosen_vehicle else {
                    let time_before_sending_patrol = generator.gen_range(MIN_REPORT_TO_SENDING_SECONDS..MAX_REPORT_TO_SENDING_SECONDS);
                    let next_report_time = current_time.checked_add_signed(Duration::seconds(time_before_sending_patrol)).unwrap();
                    events.push(Event { time: next_report_time, action: EventAction::SendPatrol(report_id) });
                    continue;
                };

                let policemen_ids = chosen_policemen
                    .iter_mut()
                    .map(|policeman| {
                        policeman.state = PolicemanState::Occupied;
                        policeman.person.id
                    })
                    .collect::<Vec<_>>();

                chosen_vehicle.state = VehicleState::Occupied;
                let vehicle_id = chosen_vehicle.id;
                let patrol_id = patrols.len();
                let patrol =
                    Patrol::generate_with_report_id_policemen_vehicles_and_sending_time_and_id(
                        &mut generator,
                        report_id,
                        policemen_ids,
                        vehicle_id,
                        current_time,
                        patrol_id,
                    );

                events.push(Event {
                    time: patrol.finish_time,
                    action: EventAction::FinishedPatrol(patrol_id),
                });
                patrols.push(patrol);
            }
            EventAction::FinishedPatrol(patrol_id) => {
                let Some(patrol) = patrols.get(patrol_id) else {
                    panic!("A patrol_id ({}) should always point to a valid patrol. Patrol count = {}", patrol_id, patrols.len());
                };

                for policeman_id in patrol.policemen_ids.as_slice() {
                    let policeman = policemen.get_mut(*policeman_id).unwrap();
                    policeman.state = if current_time < policeman.resignment_date {
                        PolicemanState::Available
                    } else {
                        PolicemanState::Resigned
                    };
                }

                vehicles.get_mut(patrol.vehicle_id).unwrap().state = VehicleState::Available;
            }
            EventAction::Snapshot(snapshot_name, is_terminal) => {
                write_places_to_file(snapshot_name, places);
                write_reports_to_file(snapshot_name, &reports);
                write_database_policemen_to_file(snapshot_name, &policemen);
                write_csv_policemen_to_file(snapshot_name, &policemen, current_time);
                write_csv_vehicle_to_file(snapshot_name, &vehicles);
                write_database_vehicle_to_file(snapshot_name, &vehicles);
                write_patrols_to_file(snapshot_name, &patrols, current_time);
                write_policeman_patrol_to_file(snapshot_name, &patrols);
                dbg!(
                    snapshot_name,
                    is_terminal,
                    places.len(),
                    policemen.len(),
                    vehicles.len(),
                    reports.len(),
                    patrols.len()
                );

                if is_terminal {
                    break;
                }
            }
            EventAction::PolicemanLastNameChange => {
                policemen
                    .iter_mut()
                    .map(|policeman| &mut policeman.person)
                    .choose(&mut generator)
                    .unwrap()
                    .change_to_random_surname(&mut generator);
            }
        }
    }
}

use chrono::{DateTime, Duration, Utc};
use rand::prelude::ThreadRng;
use rand::Rng;

const MIN_SENDING_TO_ARRIVAL_SECONDS: i64 = 300; // 5 minutes
const MAX_SENDING_TO_ARRIVAL_SECONDS: i64 = 1200; // 20 minutes
const MIN_ARRIVAL_TO_FINISH_SECONDS: i64 = 300; // 5 minutes
const MAX_ARRIVAL_TO_FINISH_SECONDS: i64 = 1200; // 20 minutes

#[derive(Debug, Clone)]
pub struct Patrol {
    pub id: usize,
    pub report_id: usize,
    pub policemen_ids: Vec<usize>,
    pub vehicle_id: usize,
    pub sending_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub finish_time: DateTime<Utc>,
}

impl Patrol {
    pub fn generate_with_report_id_policemen_vehicles_and_sending_time_and_id(
        generator: &mut ThreadRng,
        report_id: usize,
        policemen_ids: Vec<usize>,
        vehicle_id: usize,
        sending_time: DateTime<Utc>,
        id: usize,
    ) -> Self {
        let arriving_time =
            generator.gen_range(MIN_SENDING_TO_ARRIVAL_SECONDS..MAX_SENDING_TO_ARRIVAL_SECONDS);
        let arrival_time = sending_time
            .checked_add_signed(Duration::seconds(arriving_time))
            .unwrap();

        let processing_time =
            generator.gen_range(MIN_ARRIVAL_TO_FINISH_SECONDS..MAX_ARRIVAL_TO_FINISH_SECONDS);
        let finish_time = arrival_time
            .checked_add_signed(Duration::seconds(processing_time))
            .unwrap();

        Self {
            id,
            report_id,
            policemen_ids,
            vehicle_id,
            sending_time,
            arrival_time,
            finish_time,
        }
    }
}

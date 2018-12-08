use chrono::offset::TimeZone;
use chrono::DateTime;
use chrono::Timelike;
use chrono::Utc;
use std::collections::HashMap;
use std::ops::Sub;

type Minute = u32;
type GuardId = u32;

#[derive(Debug)]
enum GuardEvent {
    BeginShift(GuardId),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Record {
    timestamp: DateTime<Utc>,
    event: GuardEvent,
}

impl From<&str> for Record {
    fn from(string: &str) -> Self {
        let (time_str, event_str) = string.split_at(18);

        let timestamp = Utc
            .datetime_from_str(time_str, "[%Y-%m-%d %H:%M]")
            .expect("couldn't parse date");

        let event = match event_str.trim() {
            "wakes up" => GuardEvent::WakeUp,
            "falls asleep" => GuardEvent::FallAsleep,
            s => {
                let guard_num = s
                    .trim_matches(|c: char| !c.is_numeric())
                    .parse()
                    .expect("couldn't parse guard number");
                GuardEvent::BeginShift(guard_num)
            }
        };

        Record { timestamp, event }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let s = get_solution(input);
    println!("{:?}", s);
}

fn get_solution(input: &str) -> u32 {
    let records = get_records(input);

    let minutes_asleep = get_each_minute_spent_asleep_per_guard(&records);

    let (frequent_guard, frequent_minute) = minutes_asleep
        .iter()
        .map(|(&guard_id, minutes)| {
            let (most_slept_minute, times_slept) = minutes
                .iter()
                .max_by_key(|(_minute, &times_slept)| times_slept)
                .expect("somehow this guard hasn't slept");

            (guard_id, minutes, most_slept_minute, times_slept)
        })
        .max_by_key(|(_guard_id, _minutes, _most_slept_minute, &times_slept)| times_slept)
        .map(|(guard_id, _minutes, most_slept_minute, _times_slept)| (guard_id, most_slept_minute))
        .expect("somehow no guards have slept.");

    frequent_guard * frequent_minute
}

fn get_records(input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = input.lines().map(Record::from).collect();
    records.sort_by_key(|r| r.timestamp);
    records
}

fn get_each_minute_spent_asleep_per_guard(
    records: &[Record],
) -> HashMap<GuardId, HashMap<Minute, u32>> {
    let mut minutes_spent_asleep = HashMap::new();

    let mut current_guard = None;
    let mut time_fell_asleep = None;

    for record in records {
        match record.event {
            GuardEvent::BeginShift(guard_id) => {
                current_guard = Some(guard_id);
            }
            GuardEvent::FallAsleep => {
                time_fell_asleep = Some(record.timestamp);
            }
            GuardEvent::WakeUp => {
                let guard_id = current_guard.expect("a guard hasn't started their shift yet");
                let started_sleeping = time_fell_asleep.expect("this guard isn't asleep");

                let minute_started = started_sleeping.time().minute();
                let num_minutes_slept = record.timestamp.sub(started_sleeping).num_minutes();

                let times_minute_slept: &mut HashMap<Minute, u32> = minutes_spent_asleep
                    .entry(guard_id)
                    .or_insert_with(HashMap::new);

                for i in 0..num_minutes_slept {
                    let curr_minute = minute_started + (i as u32);
                    *times_minute_slept.entry(curr_minute).or_insert(0) += 1;
                }
            }
        }
    }

    minutes_spent_asleep
}

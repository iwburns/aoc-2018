extern crate chrono;

use chrono::DateTime;
use chrono::Utc;
use chrono::offset::TimeZone;
use std::collections::HashMap;
use std::ops::Sub;
use chrono::Timelike;

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

impl<'a> From<&'a str> for Record {
    fn from(string: &'a str) -> Self {
        let (time_str, event_str) = string.split_at(18);

        let timestamp = Utc.datetime_from_str(time_str, "[%Y-%m-%d %H:%M]")
            .expect("couldn't parse date");

        let event = match event_str.trim() {
            "wakes up" => GuardEvent::WakeUp,
            "falls asleep" => GuardEvent::FallAsleep,
            s => {
                let guard_num = s.trim_matches(|c: char| !c.is_numeric())
                    .parse()
                    .expect("couldn't parse guard number");
                GuardEvent::BeginShift(guard_num)
            },
        };

        Record {
            timestamp,
            event,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let s = get_solution(input);
    println!("{:?}", s);
}

fn get_solution(input: &str) -> u32 {
    let mut records = input.lines()
        .map(Record::from)
        .collect::<Vec<Record>>();

    records.sort_by_key(|r| r.timestamp);

    let minutes_spent_asleep = get_each_minute_spent_asleep_per_guard(&records);

    let guard_id = find_guard_who_has_slept_the_most(&minutes_spent_asleep);

    let minute = get_minute_guard_was_asleep_most_often(&minutes_spent_asleep, guard_id);

    minute * guard_id
}

fn get_each_minute_spent_asleep_per_guard(records: &[Record]) -> HashMap<GuardId, Vec<Minute>> {
    let mut minutes_spent_asleep = HashMap::new();

    let mut current_guard = None;
    let mut time_fell_asleep = None;

    for record in records {
        match record.event {
            GuardEvent::BeginShift(guard_id) => {
                current_guard = Some(guard_id);
            },
            GuardEvent::FallAsleep => {
                time_fell_asleep = Some(record.timestamp);
            },
            GuardEvent::WakeUp => {
                let guard_id = current_guard.expect("a guard hasn't started their shift yet");
                let started_sleeping = time_fell_asleep.expect("this guard isn't asleep");

                let minute_started = started_sleeping.time().minute();
                let num_minutes_slept = record.timestamp.sub(started_sleeping).num_minutes();

                let mut m = minutes_spent_asleep.entry(guard_id).or_insert(Vec::new());

                for i in 0..num_minutes_slept {
                    m.push(minute_started + (i as u32));
                }
            },
        }
    }

    minutes_spent_asleep
}

fn find_guard_who_has_slept_the_most(minutes_spent_asleep_per_guard: &HashMap<GuardId, Vec<Minute>>) -> GuardId {
    let (guard_id, _) = minutes_spent_asleep_per_guard.keys()
        .fold((0, 0), |(guard, minutes), curr_guard| {

            let minutes_spent_asleep = minutes_spent_asleep_per_guard.get(curr_guard)
                .expect("key must be present").len();

            if minutes_spent_asleep > minutes {
                return (*curr_guard, minutes_spent_asleep);
            }

            (guard, minutes)
        });

    guard_id
}

fn get_minute_guard_was_asleep_most_often(minutes_spent_asleep_per_guard: &HashMap<GuardId, Vec<Minute>>, guard_id: GuardId) -> Minute {
    let mut num_times_each_minute_was_spent_asleep: HashMap<Minute, u32> = HashMap::new();

    let all_minutes_guard_spent_asleep: &Vec<Minute> = minutes_spent_asleep_per_guard.get(&guard_id).expect("key must be present");
    for minute in all_minutes_guard_spent_asleep {
        *num_times_each_minute_was_spent_asleep.entry(*minute).or_insert(0) += 1;
    }

    get_most_slept_minute(&num_times_each_minute_was_spent_asleep)
}

fn get_most_slept_minute(num_times_each_minute_was_spent_asleep: &HashMap<Minute, u32>) -> Minute {
    let (minute, _) = num_times_each_minute_was_spent_asleep.keys()
        .fold((0, 0), |(min, amt_slept), curr_minute| {
            let times_asleep = num_times_each_minute_was_spent_asleep.get(curr_minute).expect("key must be present");
            if times_asleep > &amt_slept {
                return (*curr_minute, *times_asleep);
            }
            (min, amt_slept)
        });

    minute
}



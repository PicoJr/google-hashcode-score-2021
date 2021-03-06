use crate::data::{PInputData, POutputData};
use crate::score::Action::{Driving, Waiting};
use nom::lib::std::collections::VecDeque;

use ahash::AHashMap;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;
use log::debug;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

pub(crate) type Score = usize;
type StreetId = usize;
type StreetLength = usize;
type Time = usize;
type CarId = usize;

#[derive(Debug)]
enum Action {
    Waiting(StreetId),
    Driving(StreetId, StreetLength),
    Finished(Time),
}

struct CarTracker {
    id: CarId,
    actions: VecDeque<Action>,
    distance_current_street: StreetLength,
}

// offset, duration, period
type LightSchedule = (usize, usize, usize);

fn is_green(time: Time, light_schedule: &LightSchedule) -> bool {
    let &(offset, duration, period) = light_schedule;
    let tmod = time % period;
    offset <= tmod && tmod < offset + duration
}

fn build_light_schedule(
    output: &POutputData,
    street_name_id_length: &AHashMap<String, (StreetId, StreetLength)>,
) -> AHashMap<StreetId, LightSchedule> {
    let mut light_schedule_of_street: AHashMap<StreetId, LightSchedule> = AHashMap::default();
    for intersection_schedule in &output.intersection_schedules {
        let mut offset: usize = 0;
        let period: usize = intersection_schedule
            .light_schedules
            .iter()
            .map(|(_, duration)| duration)
            .sum();
        for (street_name, light_duration) in &intersection_schedule.light_schedules {
            let (street_id, _) = street_name_id_length
                .get(street_name)
                .expect("unknown street name");
            light_schedule_of_street.insert(*street_id, (offset, *light_duration, period));
            offset += light_duration;
        }
    }
    light_schedule_of_street
}

pub fn compute_score(input: &PInputData, output: &POutputData) -> Score {
    let mut street_name_id_length: AHashMap<String, (StreetId, StreetLength)> = AHashMap::default();
    for (street_id, street) in input.body.streets.iter().enumerate() {
        street_name_id_length.insert(
            street.street_name.clone(),
            (street_id, street.street_length),
        );
    }

    let light_schedule_of_street: AHashMap<StreetId, LightSchedule> =
        build_light_schedule(output, &street_name_id_length);

    let mut car_trackers: Vec<CarTracker> = vec![];
    let mut street_queues: FxIndexMap<StreetId, VecDeque<CarId>> = FxIndexMap::default();
    for (car_id, car_path) in input.body.car_paths.iter().enumerate() {
        let mut actions: VecDeque<Action> = VecDeque::new();
        for (i, street_name) in car_path.street_names.iter().enumerate() {
            let (street_id, street_length) = street_name_id_length
                .get(street_name)
                .expect("unknown street name");
            if i == 0 {
                street_queues
                    .entry(*street_id)
                    .or_insert_with(VecDeque::new)
                    .push_back(car_id);
            } else {
                // car start at the end of first street
                actions.push_back(Driving(*street_id, *street_length))
            }
            actions.push_back(Waiting(*street_id));
        }
        // remove last action (car does not wait at the end of its path)
        debug!("car {}: actions: {:?}", car_id, actions);
        actions.pop_back();
        debug!("car {}: actions: {:?}", car_id, actions);
        car_trackers.push(CarTracker {
            id: car_id,
            actions,
            distance_current_street: 0,
        })
    }

    let mut score: Score = 0;
    for time in 0..input.header.simulation_duration {
        // move cars not stuck at intersections
        for car_tracker in car_trackers.iter_mut() {
            if let Some(Driving(_, _)) = car_tracker.actions.front() {
                car_tracker.distance_current_street += 1;
            }
        }
        // remove empty queues
        street_queues.retain(|_street_id, street_queue| !street_queue.is_empty());

        // move at most one car out of intersection if light is green
        for (street_id, street_queue) in street_queues.iter_mut() {
            let light_schedule = light_schedule_of_street.get(street_id);
            if let Some(light_schedule) = light_schedule {
                if is_green(time, light_schedule) {
                    let car_id_out = street_queue.pop_front();
                    if let Some(car_id_out) = car_id_out {
                        let car_tracker =
                            car_trackers.get_mut(car_id_out).expect("car should exist");
                        debug!(
                            "car {} got green light at the end of street {} at time {}",
                            car_id_out, street_id, time
                        );
                        // There is no delay while a car passes through an intersection
                        // it means this car will move by one on its next street right away
                        car_tracker.distance_current_street = 1;
                        car_tracker.actions.pop_front(); // car no longer waiting
                    }
                }
            }
        }

        // set car at the end of their street to waiting or finished
        for car_tracker in car_trackers.iter_mut() {
            if let Some(Driving(_, street_length)) = car_tracker.actions.front() {
                if car_tracker.distance_current_street >= *street_length {
                    // reset for next street
                    car_tracker.distance_current_street = 0;
                    car_tracker.actions.pop_front(); // discard driving action
                    match car_tracker.actions.front() {
                        // retrieve next action
                        Some(Waiting(street_id)) => {
                            // queue up
                            street_queues
                                .entry(*street_id)
                                .or_insert_with(VecDeque::new)
                                .push_back(car_tracker.id);
                        }
                        None => {
                            // empty actions, car is finished
                            debug!("car {} finished with time {}", car_tracker.id, time);
                            car_tracker.actions.push_back(Action::Finished(time));
                            let time_matlab = time + 1;
                            if time_matlab <= input.header.simulation_duration {
                                score += input.header.bonus
                                    + (input.header.simulation_duration - time_matlab)
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::data::test_data;
    use crate::score::compute_score;

    #[test]
    fn test_compute_score_example() {
        let input_data = test_data::get_example_input_data();
        let output_data = test_data::get_example_output_data();
        let score = compute_score(&input_data, &output_data);
        assert_eq!(score, 1002);
    }
}

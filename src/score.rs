use crate::data::{PCarPath, PInputData, POutputData};
use anyhow::anyhow;
use nom::lib::std::collections::HashMap;

pub(crate) type Score = usize;
type StreetId = usize;
type IntersectionId = usize;
type StreetLength = usize;
type Time = usize;

// offset, duration, period
type LightSchedule = (usize, usize, usize);

fn is_green(time: Time, light_schedule: &LightSchedule) -> bool {
    let &(offset, duration, period) = light_schedule;
    let tmod = time % period;
    offset <= tmod && tmod < offset + duration
}

fn score_car(
    car_path: &PCarPath,
    street_name_id: &HashMap<String, StreetId>,
    light_schedule_of_street: &HashMap<StreetId, LightSchedule>,
    length_of_street: &HashMap<StreetId, StreetLength>,
    simulation_duration: usize,
    simulation_bonus: usize,
) -> anyhow::Result<Score> {
    let mut time: usize = 0;
    for (i, street_name) in car_path.street_names.iter().enumerate() {
        let street_id = street_name_id
            .get(street_name)
            .ok_or_else(|| anyhow!("unknown street id (should not happen)"))?;
        let light_schedule = light_schedule_of_street.get(street_id);
        if let Some(light_schedule) = light_schedule {
            while time < simulation_duration && !is_green(time, light_schedule) {
                time += 1;
            }
            if time >= simulation_duration {
                break;
            } else if i != 0 {
                time += length_of_street
                    .get(street_id)
                    .ok_or_else(|| anyhow!("unknown street id (should not happen)"))?;
            }
        } else {
            // it means light will remain red all the time
            time = simulation_duration; // car will remain here until the end
            break;
        }
    }
    if time >= simulation_duration {
        Ok(0)
    } else {
        Ok(simulation_bonus + (simulation_duration - time))
    }
}

pub fn compute_score(input: &PInputData, output: &POutputData) -> anyhow::Result<Score> {
    let mut street_name_id: HashMap<String, StreetId> = HashMap::new();
    let mut intersection_end_of_street: HashMap<StreetId, IntersectionId> = HashMap::new();
    let mut length_of_street: HashMap<StreetId, StreetLength> = HashMap::new();
    for (street_id, street) in input.body.streets.iter().enumerate() {
        street_name_id.insert(street.street_name.clone(), street_id);
        intersection_end_of_street.insert(street_id, street.intersection_end);
        length_of_street.insert(street_id, street.street_length);
    }
    let mut light_schedule_of_street: HashMap<StreetId, LightSchedule> = HashMap::new();
    for intersection_schedule in &output.intersection_schedules {
        let mut offset: usize = 0;
        let period: usize = intersection_schedule
            .light_schedules
            .iter()
            .map(|(_, duration)| duration)
            .sum();
        for (street_name, light_duration) in &intersection_schedule.light_schedules {
            let street_id = street_name_id
                .get(street_name)
                .ok_or_else(|| anyhow!("unknown street id (should not happen)"))?;
            light_schedule_of_street.insert(*street_id, (offset, *light_duration, period));
            offset += light_duration;
        }
    }
    let mut score: usize = 0;
    for car_path in &input.body.car_paths {
        let car_score = score_car(
            car_path,
            &street_name_id,
            &light_schedule_of_street,
            &length_of_street,
            input.header.simulation_duration,
            input.header.bonus,
        )?;
        score += car_score;
    }
    Ok(score)
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
        assert_eq!(score.unwrap(), 1002);
    }
}

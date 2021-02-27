#[derive(Debug, PartialEq)]
pub struct InputHeader {
    pub(crate) simulation_duration: usize,
    pub(crate) intersections: usize,
    pub(crate) streets: usize,
    pub(crate) cars: usize,
    pub(crate) bonus: usize,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PStreet {
    pub(crate) intersection_start: usize,
    pub(crate) intersection_end: usize,
    pub(crate) street_name: String,
    // slow
    pub(crate) street_length: usize,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PCarPath {
    pub(crate) streets: usize,
    pub(crate) street_names: Vec<String>, // slow
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PInputBody {
    pub(crate) streets: Vec<PStreet>,
    pub(crate) car_paths: Vec<PCarPath>,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PInputData {
    pub(crate) header: InputHeader,
    pub(crate) body: PInputBody,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PIntersectionSchedule {
    pub(crate) intersection_id: usize,
    pub(crate) incoming_streets: usize,
    pub(crate) light_schedules: Vec<(String, usize)>,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct POutputData {
    pub(crate) schedules: usize,
    pub(crate) intersection_schedules: Vec<PIntersectionSchedule>,
}

#[cfg(test)]
pub(crate) mod test_data {
    use crate::data::{
        InputHeader, PCarPath, PInputBody, PInputData, PIntersectionSchedule, POutputData, PStreet,
    };

    pub fn get_example_input_data() -> PInputData {
        PInputData {
            header: InputHeader {
                simulation_duration: 6,
                intersections: 4,
                streets: 5,
                cars: 2,
                bonus: 1000,
            },
            body: PInputBody {
                streets: vec![
                    PStreet {
                        intersection_start: 2,
                        intersection_end: 0,
                        street_name: "rue-de-londres".to_string(),
                        street_length: 1,
                    },
                    PStreet {
                        intersection_start: 0,
                        intersection_end: 1,
                        street_name: "rue-d-amsterdam".to_string(),
                        street_length: 1,
                    },
                    PStreet {
                        intersection_start: 3,
                        intersection_end: 1,
                        street_name: "rue-d-athenes".to_string(),
                        street_length: 1,
                    },
                    PStreet {
                        intersection_start: 2,
                        intersection_end: 3,
                        street_name: "rue-de-rome".to_string(),
                        street_length: 2,
                    },
                    PStreet {
                        intersection_start: 1,
                        intersection_end: 2,
                        street_name: "rue-de-moscou".to_string(),
                        street_length: 3,
                    },
                ],
                car_paths: vec![
                    PCarPath {
                        streets: 4,
                        street_names: vec![
                            "rue-de-londres".to_string(),
                            "rue-d-amsterdam".to_string(),
                            "rue-de-moscou".to_string(),
                            "rue-de-rome".to_string(),
                        ],
                    },
                    PCarPath {
                        streets: 3,
                        street_names: vec![
                            "rue-d-athenes".to_string(),
                            "rue-de-moscou".to_string(),
                            "rue-de-londres".to_string(),
                        ],
                    },
                ],
            },
        }
    }

    pub fn get_example_output_data() -> POutputData {
        POutputData {
            schedules: 3,
            intersection_schedules: vec![
                PIntersectionSchedule {
                    intersection_id: 1,
                    incoming_streets: 2,
                    light_schedules: vec![
                        ("rue-d-athenes".to_string(), 2),
                        ("rue-d-amsterdam".to_string(), 1),
                    ],
                },
                PIntersectionSchedule {
                    intersection_id: 0,
                    incoming_streets: 1,
                    light_schedules: vec![("rue-de-londres".to_string(), 2)],
                },
                PIntersectionSchedule {
                    intersection_id: 2,
                    incoming_streets: 1,
                    light_schedules: vec![("rue-de-moscou".to_string(), 1)],
                },
            ],
        }
    }
}

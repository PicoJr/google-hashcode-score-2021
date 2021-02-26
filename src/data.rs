pub(crate) type Score = usize;

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
    pub(crate) street_name: String, // slow
    pub(crate) street_length: usize,
}

#[derive(Debug, PartialEq)]
pub struct Street {
    pub(crate) intersection_start: usize,
    pub(crate) intersection_end: usize,
    pub(crate) street_id: usize, // use integer instead of String for performance (gotta be fast)
    pub(crate) street_length: usize,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PCarPath {
    pub(crate) streets: usize,
    pub(crate) street_names: Vec<String>, // slow
}

#[derive(Debug, PartialEq)]
pub struct CarPath {
    pub(crate) streets: usize,
    pub(crate) street_ids: Vec<usize>, // use integer instead of String for performance (gotta be fast)
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PInputBody {
    pub(crate) streets: Vec<PStreet>,
    pub(crate) car_paths: Vec<PCarPath>,
}

#[derive(Debug, PartialEq)]
pub struct InputBody {
    pub(crate) streets: Vec<Street>,
    pub(crate) car_paths: Vec<CarPath>,
}

// For parser only
#[derive(Debug, PartialEq)]
pub struct PInputData {
    pub(crate) header: InputHeader,
    pub(crate) body: PInputBody,
}

#[derive(Debug, PartialEq)]
pub struct InputData {
    pub(crate) header: InputHeader,
    pub(crate) body: InputBody,
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

// You can remove this clippy rule override once this function is implemented
#[allow(unused_variables, dead_code)]
pub fn score(input: &PInputData, output: &POutputData) -> anyhow::Result<Score> {
    Ok(0)
}

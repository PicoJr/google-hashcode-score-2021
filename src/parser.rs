use crate::data::{
    InputHeader, PCarPath, PInputBody, PInputData, PIntersectionSchedule, POutputData, PStreet,
};
use anyhow::bail;
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::{map_res, verify};
use nom::error::{context, convert_error, VerboseError};
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

pub(crate) type N = usize;
pub(crate) type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn number(input: &str) -> Res<&str, &str> {
    context("number", take_while1(|c: char| c.is_digit(10)))(input)
}

fn positive_number(input: &str) -> Res<&str, N> {
    map_res(number, |out| N::from_str_radix(out, 10))(input)
}

fn single_space(input: &str) -> Res<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}

fn non_space_or_unix_eol(input: &str) -> Res<&str, &str> {
    take_while1(|c: char| c != ' ' && c != '\n')(input)
}

fn str_list_exact(s: &str, expected_size: usize) -> Res<&str, Vec<&str>> {
    verify(
        separated_list1(single_space, non_space_or_unix_eol),
        |s: &[&str]| s.len() == expected_size,
    )(s)
}

fn input_header(input: &str) -> Res<&str, InputHeader> {
    let (i, (simulation_duration, _, intersections, _, streets, _, cars, _, bonus)) = tuple((
        positive_number,
        single_space,
        positive_number,
        single_space,
        positive_number,
        single_space,
        positive_number,
        single_space,
        positive_number,
    ))(input)?;

    Ok((
        i,
        InputHeader {
            simulation_duration,
            intersections,
            streets,
            cars,
            bonus,
        },
    ))
}

fn input_header_line(s: &str) -> Res<&str, InputHeader> {
    context("header", terminated(input_header, tag("\n")))(s)
}

fn street(input: &str) -> Res<&str, PStreet> {
    let (i, (intersection_start, _, intersection_end, _, street_name, _, street_length)) =
        tuple((
            positive_number,
            single_space,
            positive_number,
            single_space,
            non_space_or_unix_eol,
            single_space,
            positive_number,
        ))(input)?;
    Ok((
        i,
        PStreet {
            intersection_start,
            intersection_end,
            street_name: String::from(street_name),
            street_length,
        },
    ))
}

fn street_line(s: &str) -> Res<&str, PStreet> {
    context("street", terminated(street, tag("\n")))(s)
}

fn car_path(input: &str) -> Res<&str, PCarPath> {
    let (i, streets) = positive_number(input)?;
    let (i, _) = single_space(i)?;
    let (i, street_names) = str_list_exact(i, streets)?;
    Ok((
        i,
        PCarPath {
            streets,
            street_names: street_names.iter().map(|s| String::from(*s)).collect(),
        },
    ))
}

fn car_path_line(s: &str) -> Res<&str, PCarPath> {
    context("car path", terminated(car_path, tag("\n")))(s)
}

fn parse_input_body<'a>(s: &'a str, header: &InputHeader) -> Res<&'a str, PInputBody> {
    let (out, streets) = context(
        "streets",
        many_m_n(header.streets, header.streets, street_line),
    )(s)?;
    let (out, car_paths) = context(
        "car paths",
        many_m_n(header.cars, header.cars, car_path_line),
    )(out)?;
    Ok((out, PInputBody { streets, car_paths }))
}

fn _parse_input(s: &str) -> Res<&str, PInputData> {
    let (out, header) = input_header_line(s)?;
    let (out, body) = parse_input_body(out, &header)?;
    Ok((out, PInputData { header, body }))
}

pub fn parse_input(s: &str) -> anyhow::Result<PInputData> {
    match _parse_input(s) {
        Ok((_, data)) => Ok(data),
        Err(nom::Err::Error(err)) => bail!("{}", convert_error(s, err.clone())),
        _ => unreachable!(),
    }
}

fn light_schedule(s: &str) -> Res<&str, (&str, N)> {
    let (out, (street_name, _, light_duration)) =
        tuple((non_space_or_unix_eol, single_space, positive_number))(s)?;
    Ok((out, (street_name, light_duration)))
}

fn intersection_schedule(s: &str) -> Res<&str, PIntersectionSchedule> {
    let (out, intersection_id) =
        context("intersection_id", terminated(positive_number, tag("\n")))(s)?;
    let (out, incoming_streets) =
        context("incoming_streets", terminated(positive_number, tag("\n")))(out)?;
    let (out, light_schedules) = context(
        "light schedules",
        many_m_n(
            incoming_streets,
            incoming_streets,
            terminated(light_schedule, tag("\n")),
        ),
    )(out)?;
    Ok((
        out,
        PIntersectionSchedule {
            intersection_id,
            incoming_streets,
            light_schedules: light_schedules
                .iter()
                .map(|(street_name, duration)| (String::from(*street_name), *duration))
                .collect(),
        },
    ))
}

fn _parse_output(s: &str) -> Res<&str, POutputData> {
    let (out, schedules) = context("schedules", terminated(positive_number, tag("\n")))(s)?;
    let (out, intersection_schedules) = context(
        "intersection schedules",
        many_m_n(schedules, schedules, intersection_schedule),
    )(out)?;
    Ok((
        out,
        POutputData {
            schedules,
            intersection_schedules,
        },
    ))
}

pub fn parse_output(s: &str) -> anyhow::Result<POutputData> {
    match _parse_output(s) {
        Ok((_, data)) => Ok(data),
        Err(nom::Err::Error(err)) => bail!("{}", convert_error(s, err.clone())),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::test_data;
    use crate::data::{InputHeader, PCarPath, PIntersectionSchedule, PStreet};
    use crate::parser::{
        car_path, car_path_line, input_header, input_header_line, intersection_schedule,
        light_schedule, number, parse_input, parse_output, positive_number, single_space,
        str_list_exact, street, street_line,
    };

    #[test]
    fn test_number() {
        let n = number("42");
        assert_eq!(n, Ok(("", "42")))
    }

    #[test]
    fn test_positive_number() {
        let n = positive_number("42");
        assert_eq!(n, Ok(("", 42)))
    }

    #[test]
    fn test_single_space() {
        let s = single_space(" ");
        assert_eq!(s, Ok(("", " ")))
    }

    #[test]
    fn test_str_list_exact() {
        let sl = str_list_exact("a bc d", 3);
        assert_eq!(sl, Ok(("", vec!["a", "bc", "d"])))
    }

    #[test]
    fn test_input_header() {
        let h = input_header("1 2 3 4 5");
        assert_eq!(
            h,
            Ok((
                "",
                InputHeader {
                    simulation_duration: 1,
                    intersections: 2,
                    streets: 3,
                    cars: 4,
                    bonus: 5
                }
            ))
        )
    }

    #[test]
    fn test_input_header_line() {
        let h = input_header_line("1 2 3 4 5\n");
        assert_eq!(
            h,
            Ok((
                "",
                InputHeader {
                    simulation_duration: 1,
                    intersections: 2,
                    streets: 3,
                    cars: 4,
                    bonus: 5
                }
            ))
        )
    }

    #[test]
    fn test_street() {
        let s = street("1 2 hello-street 4");
        assert_eq!(
            s,
            Ok((
                "",
                PStreet {
                    intersection_start: 1,
                    intersection_end: 2,
                    street_name: "hello-street".to_string(),
                    street_length: 4
                }
            ))
        )
    }

    #[test]
    fn test_street_line() {
        let s = street_line("1 2 hello-street 4\n");
        assert_eq!(
            s,
            Ok((
                "",
                PStreet {
                    intersection_start: 1,
                    intersection_end: 2,
                    street_name: "hello-street".to_string(),
                    street_length: 4
                }
            ))
        )
    }

    #[test]
    fn test_car_path() {
        let s = car_path("4 s1 s2 s3 s4");
        assert_eq!(
            s,
            Ok((
                "",
                PCarPath {
                    streets: 4,
                    street_names: vec![
                        "s1".to_string(),
                        "s2".to_string(),
                        "s3".to_string(),
                        "s4".to_string()
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_car_path_line() {
        let s = car_path_line("4 s1 s2 s3 s4\n");
        assert_eq!(
            s,
            Ok((
                "",
                PCarPath {
                    streets: 4,
                    street_names: vec![
                        "s1".to_string(),
                        "s2".to_string(),
                        "s3".to_string(),
                        "s4".to_string()
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_example_input() {
        let input = "6 4 5 2 1000\n\
                           2 0 rue-de-londres 1\n\
                           0 1 rue-d-amsterdam 1\n\
                           3 1 rue-d-athenes 1\n\
                           2 3 rue-de-rome 2\n\
                           1 2 rue-de-moscou 3\n\
                           4 rue-de-londres rue-d-amsterdam rue-de-moscou rue-de-rome\n\
                           3 rue-d-athenes rue-de-moscou rue-de-londres\n";
        let d = parse_input(input);
        let input_data = test_data::get_example_input_data();
        assert_eq!(d.unwrap(), input_data)
    }

    #[test]
    fn test_light_schedule() {
        let ls = light_schedule("hello-street 4");
        assert_eq!(ls, Ok(("", ("hello-street", 4))));
    }

    #[test]
    fn test_intersection_schedule() {
        let ls = intersection_schedule("1\n2\nrue-d-athenes 2\nrue-d-amsterdam 1\n");
        assert_eq!(
            ls,
            Ok((
                "",
                PIntersectionSchedule {
                    intersection_id: 1,
                    incoming_streets: 2,
                    light_schedules: vec![
                        ("rue-d-athenes".to_string(), 2),
                        ("rue-d-amsterdam".to_string(), 1)
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_example_output() {
        let output = "3\n\
                           1\n\
                           2\n\
                           rue-d-athenes 2\n\
                           rue-d-amsterdam 1\n\
                           0\n\
                           1\n\
                           rue-de-londres 2\n\
                           2\n\
                           1\n\
                           rue-de-moscou 1\n";
        let d = parse_output(output);
        let output_data = test_data::get_example_output_data();
        assert_eq!(d.unwrap(), output_data)
    }
}

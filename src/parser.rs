use crate::data::{InputHeader, PCarPath, PInputBody, PInputData, PStreet};
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

// You can remove this clippy rule override once this function is implemented
#[allow(unused_variables)]
/// Parse input file content as a `InputData`
///
/// s: input file content
///
/// returns InputData parsed from input file content
pub fn parse_input(s: &str) -> anyhow::Result<PInputData> {
    match input_header_line(s) {
        Ok((out, header)) => match parse_input_body(out, &header) {
            Ok((out, body)) => Ok(PInputData { header, body }),
            Err(nom::Err::Error(err)) => bail!("{}", convert_error(s, err.clone())),
            _ => unreachable!(),
        },
        Err(nom::Err::Error(err)) => bail!("{}", convert_error(s, err.clone())),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{InputHeader, PCarPath, PInputBody, PInputData, PStreet};
    use crate::parser::{
        car_path, car_path_line, input_header, input_header_line, number, parse_input,
        positive_number, single_space, str_list_exact, street, street_line,
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
    fn test_example() {
        let input = "6 4 5 2 1000\n\
                           2 0 rue-de-londres 1\n\
                           0 1 rue-d-amsterdam 1\n\
                           3 1 rue-d-athenes 1\n\
                           2 3 rue-de-rome 2\n\
                           1 2 rue-de-moscou 3\n\
                           4 rue-de-londres rue-d-amsterdam rue-de-moscou rue-de-rome\n\
                           3 rue-d-athenes rue-de-moscou rue-de-londres\n";
        let d = parse_input(input);
        assert_eq!(
            d.unwrap(),
            PInputData {
                header: InputHeader {
                    simulation_duration: 6,
                    intersections: 4,
                    streets: 5,
                    cars: 2,
                    bonus: 1000
                },
                body: PInputBody {
                    streets: vec![
                        PStreet {
                            intersection_start: 2,
                            intersection_end: 0,
                            street_name: "rue-de-londres".to_string(),
                            street_length: 1
                        },
                        PStreet {
                            intersection_start: 0,
                            intersection_end: 1,
                            street_name: "rue-d-amsterdam".to_string(),
                            street_length: 1
                        },
                        PStreet {
                            intersection_start: 3,
                            intersection_end: 1,
                            street_name: "rue-d-athenes".to_string(),
                            street_length: 1
                        },
                        PStreet {
                            intersection_start: 2,
                            intersection_end: 3,
                            street_name: "rue-de-rome".to_string(),
                            street_length: 2
                        },
                        PStreet {
                            intersection_start: 1,
                            intersection_end: 2,
                            street_name: "rue-de-moscou".to_string(),
                            street_length: 3
                        },
                    ],
                    car_paths: vec![
                        PCarPath {
                            streets: 4,
                            street_names: vec![
                                "rue-de-londres".to_string(),
                                "rue-d-amsterdam".to_string(),
                                "rue-de-moscou".to_string(),
                                "rue-de-rome".to_string()
                            ]
                        },
                        PCarPath {
                            streets: 3,
                            street_names: vec![
                                "rue-d-athenes".to_string(),
                                "rue-de-moscou".to_string(),
                                "rue-de-londres".to_string()
                            ]
                        },
                    ]
                }
            }
        )
    }
}

use clap::{App, Arg};

pub fn get_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Google Hashcode Score Calculator")
        .arg(
            Arg::with_name("input")
                .help("input file paths")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("output file paths (one for each input provided file)")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
}

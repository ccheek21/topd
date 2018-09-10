use clap::{App, Arg, ArgMatches};

fn is_int(s: String) -> Result<(), String> {
    match s.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(format!("invalid integer {}", s)),
    }
}

pub fn parse_args() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("store")
            .long("store")
            .value_name("FILE")
            .help("Use a custom storage file for the directory weights")
            .takes_value(true))
        .arg(Arg::with_name("purge")
            .short("P")
            .long("purge")
            .help("Purge directories that no longer exist from the database"))
        .arg(Arg::with_name("increase")
            .short("i")
            .long("increase")
            .help("Increase the weight of a directory by WEIGHT")
            .value_name("WEIGHT")
            .takes_value(true))
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .help("Add a visit to a DIRECTORY to the store")
            .value_name("DIRECTORY")
            .takes_value(true))
        .arg(Arg::with_name("decrease")
            .short("d")
            .long("decrease")
            .help("Decrease the weight of a directory by WEIGHT")
            .value_name("WEIGHT")
            .takes_value(true))
        .arg(Arg::with_name("truncate")
            .short("T")
            .long("truncate")
            .help("Truncate the stored directories to only the top N")
            .value_name("N")
            .validator(is_int)
            .takes_value(true))
        .arg(Arg::with_name("sorted")
            .long("sorted")
            .help("Print the stored directories in order of highest to lowest score"))
        .arg(Arg::with_name("sort_method")
            .long("sort_method")
            .help("The method to sort by most used")
            .takes_value(true)
            .possible_values(&["frecent", "frequent", "recent"])
            .default_value("frecent"))
        .arg(Arg::with_name("limit")
            .long("limit")
            .short("l")
            .takes_value(true)
            .help("Limit the number of results printed --sorted"))
        .arg(Arg::with_name("stat")
            .short("s")
            .long("stat")
            .help("Print statistics about the stored directories"))
        .arg(Arg::with_name("directory")
            .index(1)
            .help("The directory to jump to"))
        .get_matches()
}

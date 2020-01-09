extern crate budderbrot;
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("budderbrot")
        .version("0.1.0")
        .author("Malte Müller <malte@malte-mueller.eu>")
        .arg(
            Arg::with_name("dimension")
                .short("d")
                .long("dimension")
                .takes_value(true)
                .help("default 8192x4608"),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .takes_value(true)
                .help("default 100"),
        )
        .arg(
            Arg::with_name("mandel")
                .short("m")
                .long("mandel")
                .takes_value(false)
                .help("render mandel only"),
        )
        .arg(
            Arg::with_name("buddah")
                .short("b")
                .long("buddah")
                .takes_value(false)
                .help("render buddah only"),
        )
        .get_matches();

    let iterations = match matches.value_of("iterations") {
        None => panic!("Missing value"),
        Some(s) => s.parse::<usize>().unwrap_or(100),
    };

    let dimension: Vec<&str> = matches.value_of("dimension").unwrap().split('x').collect();
    let width = dimension[0].parse::<u32>().unwrap_or(8192);
    let height = dimension[1].parse::<u32>().unwrap_or(4608);

    let mandel = !matches.is_present("buddah");
    let buddah = !matches.is_present("mandel");

    budderbrot::run(width, height, iterations, mandel, buddah);
}

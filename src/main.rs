#![allow(non_camel_case_types, dead_code)]

mod perms;

use clap::{App, Arg};
use perms::*;

fn main() {
    cli();
}

fn cli() {
    let matches = App::new("Discord Permissions calculator")
        .version("v1.0.0")
        .author("Ben Richeson <benricheson101@gmail.com>")
        .about("Easily calculate Discord bitfield permissions")
        .arg(Arg::with_name("bits")
            .short("b")
            .long("bits")
            .help("Input bits instead of permission names")
            .value_name("bits")
            .takes_value(true)
        )
        .arg(Arg::with_name("serialize")
            .short("s")
            .long("serialize")
            .help("Show all perms with true and false showing if they are included or not")
        )
        .get_matches();

    let serialize = matches.occurrences_of("serialize") > 0;

    let p = if let Some(bits) = matches.value_of("bits") {
        let bits = bits.parse::<usize>().unwrap();
        Perms::new(bits)
    } else {
        Perms::from_input()
    };

    println!("Bits: {}", p);
    println!("Tokens: {:?}", p.tokens());

    if serialize {
       println!("{:#?}", p.serialize());
    }
}

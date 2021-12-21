use clap::{App, Arg};
mod core;

fn main() {
    let args = App::new("Saltcrypt")
        .version("0.1.0")
        .about("In-place file encryption/decryption tool")
        .author("Mikko Kolehmainen")
        .arg(
            Arg::with_name("filepath")
                .index(1)
                .short("f")
                .long("filepath")
                .takes_value(true)
                .required(true)
                .requires("password")
                .help("Path to a file to be encrypted/decrypted"),
        )
        .arg(
            Arg::with_name("password")
                .index(2)
                .short("p")
                .long("password")
                .takes_value(true)
                .required(true)
                .requires("salt")
                .help("Password to be used"),
        )
        .arg(
            Arg::with_name("salt")
                .index(3)
                .short("s")
                .long("salt")
                .takes_value(true)
                .required(true)
                .requires("mode")
                .help("Salt to be used for extra security"),
        )
        .arg(
            Arg::with_name("mode")
                .index(4)
                .possible_values(&["e", "d"])
                .required(true)
                .help("Choose encryption or decryption mode"),
        )
        .get_matches();

    if args.is_present("filepath") {
        match args.value_of("mode").unwrap() {
            "e" => {
                core::encrypt_file(
                    args.value_of("filepath").unwrap(),
                    args.value_of("password").unwrap(),
                    args.value_of("salt").unwrap(),
                )
                .unwrap();
            }
            "d" => {
                core::decrypt_file(
                    args.value_of("filepath").unwrap(),
                    args.value_of("password").unwrap(),
                    args.value_of("salt").unwrap(),
                )
                .unwrap();
            }
            _ => unreachable!(),
        }
    }
}

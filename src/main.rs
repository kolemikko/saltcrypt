use clap::{crate_version, App, Arg};
mod core;

fn main() {
    let args = App::new("Saltcrypt")
        .version(crate_version!())
        .about("In-place file encryption tool")
        .author("Mikko Kolehmainen")
        .arg(
            Arg::with_name("mode")
                .index(1)
                .possible_values(&["e", "d"])
                .requires("filepath")
                .help("Encryption or decryption mode"),
        )
        .arg(
            Arg::with_name("filepath")
                .index(2)
                .short("f")
                .long("filepath")
                .takes_value(true)
                .requires("password")
                .help("Path to file"),
        )
        .arg(
            Arg::with_name("password")
                .index(3)
                .short("p")
                .long("password")
                .takes_value(true)
                .requires("salt")
                .help("Password to be used"),
        )
        .arg(
            Arg::with_name("salt")
                .index(4)
                .short("s")
                .long("salt")
                .takes_value(true)
                .requires("mode")
                .help("Salt to be used"),
        )
        .get_matches();

    if args.is_present("mode") {
        match args.value_of("mode").unwrap() {
            "e" => {
                match core::encrypt_file(
                    args.value_of("filepath").unwrap(),
                    args.value_of("password").unwrap(),
                    args.value_of("salt").unwrap(),
                ) {
                    Ok(_) => println!(
                        "{} encrypted successfully!",
                        args.value_of("filepath").unwrap()
                    ),
                    Err(error) => println!("Couldn't encrypt the file!\n{}", error),
                };
            }
            "d" => {
                match core::decrypt_file(
                    args.value_of("filepath").unwrap(),
                    args.value_of("password").unwrap(),
                    args.value_of("salt").unwrap(),
                ) {
                    Ok(_) => println!(
                        "{} decrypted successfully!",
                        args.value_of("filepath").unwrap()
                    ),
                    Err(error) => println!("Couldn't decrypt the file!\n{}", error),
                };
            }
            _ => unreachable!(),
        }
    } else {
        println!("Should start gui now!");
    }
}

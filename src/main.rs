use clap::{App, Arg};

mod converter;

fn main() {
    let matches = App::new("vtg")
        .author("Ivan I. <clicker.heroes.acg@gmail.com>")
        .about("This small cli-tool will help you with conversion video into gif.")
        .arg(
            Arg::with_name("origin")
                .help("original video.")
                .short("o")
                .required(true)
                .takes_value(true)
        )
        .get_matches();
    let origin = matches.value_of("origin")
        .expect("Error with origin file...");
}

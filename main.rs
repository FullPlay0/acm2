extern crate clap;
use clap::{App, Arg, SubCommand};

mod acm;

fn main() {
    // Основная логика для вызова из командной строки
    let matches = App::new("ACM CLI")
        .version("1.0.0")
        .author("Your Name <you@example.com>")
        .about("Implements arithmetic congruence monoids (ACM)")
        .subcommand(SubCommand::with_name("calculate")
            .about("Calculates ACM elements")
            .arg(Arg::with_name("a")
                .help("The 'a' parameter in the ACM formula")
                .required(true)
                .index(1))
            .arg(Arg::with_name("b")
                .help("The 'b' parameter in the ACM formula")
                .required(true)
                .index(2)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("calculate") {
        let a = matches.value_of("a").unwrap().parse::<i32>().expect("a needs to be an integer");
        let b = matches.value_of("b").unwrap().parse::<i32>().expect("b needs to be an integer");
        let elements = acm::calculate_acm(a, b);
        println!("ACM elements: {:?}", elements);
    } else {
        // Основная логика для вызова без аргументов командной строки
        let a = 1;
        let b = 4;
        let acm_elements = acm::calculate_acm(a, b);
        println!("Elements of ACM: {:?}", acm_elements);

        let density = acm::calculate_density();
        println!("Atomic density: {}", density);
    }
}

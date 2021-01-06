use clap::{load_yaml, App};
use dialoguer::console::style;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        ("new", Some(sub_m)) => {
            if let Err(err) = velox::create_new_project(sub_m.value_of("name").unwrap()) {
                panic!("{}{}", style("Error: ").red().bold(), err.to_string())
            }
        }
        ("run", Some(sub_m)) => {
            if sub_m.is_present("release") {
                if let Err(err) = velox::run(true) {
                    panic!("{}{}", style("Error: ").red().bold(), err.to_string())
                }
            } else if let Err(err) = velox::run(false) {
                panic!("{}{}", style("Error: ").red().bold(), err.to_string())
            }
        } // push was used
        ("build", Some(sub_m)) => {} // commit was used
        _ => {}
    };
}

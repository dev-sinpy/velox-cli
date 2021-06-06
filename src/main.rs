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
        ("dev", Some(_sub_m)) => {
            if let Err(err) = velox::run() {
                panic!("{}{}", style("Error: ").red().bold(), err.to_string())
            }
        }
        ("build", Some(_sub_m)) => {
            if let Err(err) = velox::build() {
                panic!("{}{}", style("Error: ").red().bold(), err.to_string())
            }
        }
        _ => {}
    };
}

use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        ("new", Some(sub_m)) => {
            velox::create_new_project(sub_m.value_of("name").unwrap()).unwrap();
        }
        ("run", Some(sub_m)) => {
            if sub_m.is_present("release") {
                velox::run(true);
            } else {
                velox::run(false);
            }
        } // push was used
        ("build", Some(sub_m)) => {} // commit was used
        _ => {}
    };
}

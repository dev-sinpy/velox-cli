use velox::AppBuilder;

fn main() {
    let mut app = AppBuilder::from_config(include_str!("../velox-config.json").to_string()).build();
    app.run().unwrap();
}

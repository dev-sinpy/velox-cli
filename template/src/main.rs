use velox::AppBuilder;

fn main() {
    let app = AppBuilder::from_config(include_str!("../velox.conf.json").to_string())
        .invoke_handler(|_proxy, req| {
            Some(velox::json!({
                "result": "Hello, world!",
            }))
        })
        .build();
    app.run().unwrap();
}

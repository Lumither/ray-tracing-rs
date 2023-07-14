use ray_tracing_rs::trace::{Config, Tracer};

fn main() {
    let config = Config::load();

    let tracer = Tracer::new(config);
    println!("Hello, world!");
}

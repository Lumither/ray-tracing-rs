use ray_tracing_rs::trace::Config;

fn main() {
    let config = Config::load();
    // let tracer = Tracer::new(config);
    println!("{config:?}");
}

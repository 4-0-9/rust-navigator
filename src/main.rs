use app::run_app;

pub mod app;
mod interface;
pub mod robot;
pub mod world;
pub mod textures;
pub mod rendering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app()
}

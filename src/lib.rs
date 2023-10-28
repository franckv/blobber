pub mod app;
pub mod controller;
pub mod map;

use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

pub use controller::CameraController;

pub const MAP: &str = include_str!("../assets/dungeon.map");
pub const CUBE: &str = "cube.obj";
pub const LIGHT: &str = "sphere.obj";
pub const TILE_SIZE: f32 = 1.;
pub const WALL_TEXTURE: &str = "tileset.png";
pub const WALL_TEXTURE_N: &str = "normal.png";

pub fn init_logger() {
    let config_other = ConfigBuilder::new().add_filter_ignore_str("blobber").build();
    let config_self = ConfigBuilder::new().add_filter_allow_str("blobber").build();

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            config_other,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Info,
            config_self,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ]);
}
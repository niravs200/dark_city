pub mod menu;
mod menu_cloud;
pub mod splash;

pub use menu::menu_plugin;
pub use splash::splash_plugin;

pub use menu::{MenuAssets, are_menu_assets_loaded, load_menu_assets};

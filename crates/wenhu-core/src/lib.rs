use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::WenhuCore;
#[cfg(mobile)]
use mobile::WenhuCore;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the wenhu-core APIs.
pub trait WenhuCoreExt<R: Runtime> {
  fn wenhu_core(&self) -> &WenhuCore<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WenhuCoreExt<R> for T {
  fn wenhu_core(&self) -> &WenhuCore<R> {
    self.state::<WenhuCore<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("wenhu-core")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let wenhu_core = mobile::init(app, api)?;
      #[cfg(desktop)]
      let wenhu_core = desktop::init(app, api)?;
      app.manage(wenhu_core);
      Ok(())
    })
    .build()
}

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            #[cfg(target_os = "macos")]
            let app_menu = Submenu::with_items(
                handle,
                "Gemini",
                true,
                &[
                    &PredefinedMenuItem::about(handle, Some("Gemini"), None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::services(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::hide(handle, None)?,
                    &PredefinedMenuItem::hide_others(handle, None)?,
                    &PredefinedMenuItem::show_all(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::quit(handle, None)?,
                ],
            )?;

            let edit_menu = Submenu::with_items(
                handle,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(handle, None)?,
                    &PredefinedMenuItem::redo(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::cut(handle, None)?,
                    &PredefinedMenuItem::copy(handle, None)?,
                    &PredefinedMenuItem::paste(handle, None)?,
                    &PredefinedMenuItem::select_all(handle, None)?,
                ],
            )?;

            let reload = MenuItem::with_id(handle, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
            let view_menu = Submenu::with_items(handle, "View", true, &[&reload])?;

            #[cfg(target_os = "macos")]
            let menu = Menu::with_items(handle, &[&app_menu, &edit_menu, &view_menu])?;
            #[cfg(not(target_os = "macos"))]
            let menu = Menu::with_items(handle, &[&edit_menu, &view_menu])?;

            app.set_menu(menu)?;

            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://gemini.google.com/".parse().unwrap()),
            )
            .title("Gemini")
            .inner_size(1200.0, 800.0)
            .theme(Some(tauri::Theme::Dark))
            .build()?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id() == "reload" {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload()");
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

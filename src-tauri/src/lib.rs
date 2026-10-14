mod commands;
mod db;
mod error;
mod models;
mod repositories;

use tauri::Manager;
use tauri_specta::{Builder, collect_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta = Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::courses::list_courses,
        commands::courses::create_course,
        commands::courses::delete_course,
    ]);

    // Génère src/lib/bindings.ts à chaque lancement en dev
    #[cfg(debug_assertions)]
    specta
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("Échec de l'export des bindings TS");

    tauri::Builder::default()
        // single-instance doit être enregistré EN PREMIER
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta.invoke_handler())
        .setup(move |app| {
            specta.mount_events(app);

            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let pool = tauri::async_runtime::block_on(db::init(&dir.join("studytrack.db")))?;
            app.manage(db::Db(pool));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erreur au lancement de l'application");
}

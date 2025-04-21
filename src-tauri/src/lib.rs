use tauri::{Manager, Wry};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_deep_link::DeepLinkExt;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};
use std::{fs, path::PathBuf};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_deep_link::init())
    .setup(|app| {

    let main_window = app.get_webview_window("main").unwrap();
    let app_handle = app.handle().clone();
    
    // app.deep_link().on_open_url(|event| {
    //     println!("TODO, deep link URLs: {:?}", event.urls());
    // });

    // Build and register the navigation plugin
    app.handle().plugin(
        tauri::plugin::Builder::<Wry, ()>::new("external-links")
            .on_navigation(move |window, url| {
                let is_external = !url.to_string().contains("stremio.com");

                if is_external {
                    println!("Opening external link: {}", url);
                    // Use the captured app_handle to get the shell scope
                    let shell_scope = app_handle.shell();
                    shell_scope.open(url.as_str(), None).expect("Failed to open external link");
                    false // Prevent the webview from navigating
                } else {
                    true // Allow the webview to navigate
                }
            })
            .build()
    )?;
    

      #[cfg(target_os = "macos")]
      {
          use tauri::TitleBarStyle;
          main_window.set_title_bar_style(TitleBarStyle::Overlay)
            .expect("Unsupported platform! 'set_title_bar_style'");

          apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), Some(16.0))
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
      }

      // Read the script from the external file
      // Resolve the path to inject.js within the app's resources
      let script_path = app.handle()
          .path()
          .resolve("inject.js", tauri::path::BaseDirectory::Resource)
          .expect("failed to resolve resource path");
      let mut eval_script_content = fs::read_to_string(&script_path)
          .expect(&format!("Failed to read script from {:?}", script_path));

      // Optionally replace placeholders if needed, though the example doesn't need it now
      // eval_script_content = eval_script_content.replace("{{", "{").replace("}}", "}"); // Basic placeholder escape if needed

      let window_clone = main_window.clone();
      main_window.run_on_main_thread(move || {
          window_clone.eval(&eval_script_content).expect("Failed to eval script");
      })?;

      // Existing log plugin setup
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

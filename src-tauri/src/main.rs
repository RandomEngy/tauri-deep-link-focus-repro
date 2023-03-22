// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotificationManager, ToastNotification};
use windows::core::{Result, HSTRING};

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri_plugin_deep_link::prepare("us.engy.focus-repro");

    fire_notification();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle();

            tauri_plugin_deep_link::register(
                "focusrepro",
                move |request| {
                    println!("Activated from second instance: {}", request);
                    if let Some(main_window) = handle.get_window("main") {
                        if let Err(err) = main_window.set_focus() {
                            eprintln!("Could not set focus on main window: {:?}", err);
                        }
                    }
                },
              )
              .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn fire_notification() {
    let toast_xml = create_toast_xml().unwrap();
    let aumid = "us.engy.focus-repro";
    let toast_notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(aumid)).unwrap();
    let notification = ToastNotification::CreateToastNotification(&toast_xml).unwrap();
    toast_notifier.Show(&notification).unwrap();
}

fn create_toast_xml() -> Result<XmlDocument> {
    let toast_xml = XmlDocument::new()?;
    toast_xml.LoadXml(&HSTRING::from("<toast launch=\"focusrepro:open\" activationType=\"protocol\">
        <visual>
            <binding template=\"ToastText01\">
                <text>Testing</text>
            </binding>
        </visual>
    </toast>"))?;

    Ok(toast_xml)
}
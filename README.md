# Tauri deep link focus issue

Repro steps:

1. Set up `tauri_plugin_deep_link::register` with a handler that tries to call `window.set_focus()`
2. Open the app
3. Fire a notification that opens the app with a protocol
4. Put the app in the background by focusing a different app
5. Click the notification

Expected result:
App comes to the foreground

Actual result:
App flashes in the taskbar and does not come to the foreground
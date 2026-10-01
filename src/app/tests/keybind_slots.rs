use super::*;
use crate::domain::input::{InputAction, KeyId, Mods, NavKeys};
use crate::frontend::settings::{ActionId, SettingsMsg};
use iced::keyboard::Modifiers;
use iced::keyboard::key::Named;

const PICKER: crate::domain::input::ActiveScopes =
    crate::domain::input::ActiveScopes { fields: false, search: false, downloads: false };

fn capture(app: &mut App, path: &str, slot: usize, key: keyboard::Key) {
    let _ = update(app, Message::Settings(SettingsMsg::KeybindCapture(path.into(), slot)));
    let _ = update(app, Message::KeyPressed(key, Modifiers::default()));
    let _ =
        update(app, Message::KeyPressed(keyboard::Key::Named(Named::Enter), Modifiers::default()));
}

fn char_action(app: &App, text: &str) -> Option<InputAction> {
    app.input.bindings.lookup_key(&KeyId::Char(text.into()), Mods::NONE, PICKER)
}

fn settings_app() -> App {
    let mut app = test_app();
    let _ = update(&mut app, Message::ToggleSettings);
    app
}

#[test]
fn second_slot_capture_keeps_the_arrow_key() {
    let mut app = settings_app();
    capture(&mut app, "keys.navLeft", 1, keyboard::Key::Character("h".into()));
    assert_eq!(app.config.str_path("keys.navLeft"), "left, h");
    assert_eq!(
        app.input.bindings.lookup_key(&KeyId::Left, Mods::NONE, PICKER),
        Some(InputAction::NavLeft)
    );
    assert_eq!(char_action(&app, "h"), Some(InputAction::NavLeft));
    assert_eq!(char_action(&app, "a"), None);
}

#[test]
fn primary_capture_keeps_the_second_key() {
    let mut app = settings_app();
    capture(&mut app, "keys.navUp", 0, keyboard::Key::Character("i".into()));
    assert_eq!(app.config.str_path("keys.navUp"), "i, w");
    assert_eq!(app.input.bindings.lookup_key(&KeyId::Up, Mods::NONE, PICKER), None);
}

#[test]
fn unbinding_one_slot_leaves_the_other() {
    let mut app = settings_app();
    let _ =
        update(&mut app, Message::Settings(SettingsMsg::KeybindCapture("keys.navDown".into(), 1)));
    let _ = update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureUnbind));
    let _ = update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureApply));
    assert_eq!(app.config.str_path("keys.navDown"), "down");
    assert_eq!(char_action(&app, "s"), None);
}

#[test]
fn capture_back_to_the_default_stores_no_override() {
    let mut app = settings_app();
    capture(&mut app, "keys.navRight", 1, keyboard::Key::Character("l".into()));
    assert_eq!(app.config.str_path("keys.navRight"), "right, l");
    capture(&mut app, "keys.navRight", 1, keyboard::Key::Character("d".into()));
    assert_eq!(app.config.str_path("keys.navRight"), "");
}

#[test]
fn vim_and_wasd_rewrite_the_second_keys() {
    let mut app = settings_app();
    let _ =
        update(&mut app, Message::Settings(SettingsMsg::Run(ActionId::SetNavKeys(NavKeys::Vim))));
    for (path, value) in [
        ("keys.navLeft", "left, h"),
        ("keys.navDown", "down, j"),
        ("keys.navUp", "up, k"),
        ("keys.navRight", "right, l"),
    ] {
        assert_eq!(app.config.str_path(path), value, "{path}");
    }
    for (text, action) in [
        ("h", InputAction::NavLeft),
        ("j", InputAction::NavDown),
        ("k", InputAction::NavUp),
        ("l", InputAction::NavRight),
    ] {
        assert_eq!(char_action(&app, text), Some(action), "{text}");
    }
    assert_eq!(char_action(&app, "a"), None);
    let _ =
        update(&mut app, Message::Settings(SettingsMsg::Run(ActionId::SetNavKeys(NavKeys::Wasd))));
    for path in ["keys.navLeft", "keys.navDown", "keys.navUp", "keys.navRight"] {
        assert_eq!(app.config.str_path(path), "", "{path}");
    }
    assert_eq!(char_action(&app, "a"), Some(InputAction::NavLeft));
    assert_eq!(char_action(&app, "h"), None);
}

#[test]
fn vim_keys_take_a_letter_from_another_action() {
    let mut app = settings_app();
    capture(&mut app, "keys.playlists", 0, keyboard::Key::Character("j".into()));
    assert_eq!(char_action(&app, "j"), Some(InputAction::Playlists));
    let _ =
        update(&mut app, Message::Settings(SettingsMsg::Run(ActionId::SetNavKeys(NavKeys::Vim))));
    assert_eq!(char_action(&app, "j"), Some(InputAction::NavDown));
    assert_eq!(app.config.str_path("keys.playlists"), "none");
    assert_eq!(app.input.bindings.conflicts(), Vec::new());
}

#[test]
fn keyboard_activation_picks_a_slot() {
    use crate::frontend::settings::{Control, SettingsKey, build_tab};
    let mut app = settings_app();
    let _ = update(&mut app, Message::SetSettingsTab(String::from("picker")));
    let cards = build_tab("picker", &app.config, &[], &[], "", &[]);
    let (section, control) = cards
        .iter()
        .enumerate()
        .find_map(|(section, (_, rows))| {
            rows.iter()
                .position(|row| {
                    matches!(&row.control, Control::KeyBinding { path, .. } if path == "keys.navLeft")
                })
                .map(|control| (section, control))
        })
        .expect("nav left row");
    let _ = update(&mut app, Message::Settings(SettingsMsg::SelectSection(section)));
    let _ = update(&mut app, Message::Settings(SettingsMsg::FocusControl(control)));
    let key = |app: &mut App, key| {
        let _ = update(app, Message::Settings(SettingsMsg::Key(key)));
    };
    key(&mut app, SettingsKey::Activate);
    assert!(app.panels.settings.keybind_capture.is_none());
    assert_eq!(app.panels.settings.focused_choice, Some(0));
    key(&mut app, SettingsKey::Down);
    assert_eq!(app.panels.settings.focused_choice, Some(1));
    key(&mut app, SettingsKey::Down);
    assert_eq!(app.panels.settings.focused_choice, Some(0));
    key(&mut app, SettingsKey::Up);
    key(&mut app, SettingsKey::Next);
    let capture = app.panels.settings.keybind_capture.as_ref().expect("capture open");
    assert_eq!(capture.action, InputAction::NavLeft);
    assert_eq!(capture.slot, 1);
    assert_eq!(capture.triggers, crate::domain::input::parse_binding("a").expect("valid"));
}

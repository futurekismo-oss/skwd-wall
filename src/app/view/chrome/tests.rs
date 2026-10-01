#![cfg(test)]

use serde_json::json;

use super::{bar_intent_message, filter_bar_footprint};
use crate::app::tests::test_app;

#[test]
fn theme_mode_controls_share_the_effective_persisted_value() {
    use crate::contracts::settings::SettingsSource;
    use crate::infrastructure::config::Config;

    for (root, expected) in [
        (json!({}), "dark"),
        (json!({"matugen": {"mode": "light"}}), "light"),
        (json!({"matugen": {"mode": "dark"}}), "dark"),
        (json!({"theme": {"mode": "light"}, "matugen": {"mode": "dark"}}), "light"),
        (json!({"theme": {"mode": "dark"}, "matugen": {"mode": "light"}}), "dark"),
        (json!({"theme": {"mode": "auto"}, "matugen": {"mode": "light"}}), "auto"),
        (json!({"theme": {"mode": ""}, "matugen": {"mode": "light"}}), "light"),
    ] {
        let mut app = test_app();
        app.config = Config::from_data(root);
        assert_eq!(SettingsSource::text(&app.config, skwd_config::keys::theme::MODE), expected);
        assert_eq!(super::theme_bar_model(&app, "skwd-iris", false, 1).mode, expected);
    }
}

#[test]
fn pinned_mode_precedes_the_effective_global_mode() {
    use crate::infrastructure::ipc::IpcMsg;

    for (pinned, value, expected) in [
        (true, json!("dark"), "dark"),
        (true, json!("auto"), "auto"),
        (true, json!(""), "light"),
        (true, json!(false), "light"),
        (false, json!("dark"), "light"),
    ] {
        let mut app = test_app();
        app.config.set_key(skwd_config::keys::matugen::MODE, json!("light"));
        let _ = app.daemon.client.call("subscribe", json!({"events": ["skwd."]}));
        let list_id = app.daemon.client.call("wall.list", json!({}));
        app.handle_ipc(IpcMsg::Connected { list_id });
        app.handle_ipc(IpcMsg::Response {
            id: list_id,
            result: Some(json!({"wallpapers": [{
                "name": "pinned.png", "type": "static", "thumb": "/thumbs/pinned.png",
                "hue": 1, "mtime": 0
            }]})),
            error: None,
        });
        app.scene.set_current(0, app.library_session.filtered.len());
        let key = app.library_session.library.catalog().items[0].key.clone();
        app.config.set_key(
            skwd_config::keys::theme::WALLPAPER_PROFILES,
            json!([{"key": key, "settingsPinned": pinned, "settings": {"theme.mode": value}}]),
        );
        assert_eq!(super::theme_bar_model(&app, "skwd-iris", false, 1).mode, expected);
    }
}

#[test]
fn committed_theme_modes_reopen_in_the_bar_and_settings() {
    use crate::app::{App, Message, update};
    use crate::contracts::settings::SettingsSource;
    use crate::frontend::theme_designer::ThemeMsg;
    use crate::infrastructure::config::Config;

    let mut app = test_app();
    app.config.set_key(skwd_config::keys::matugen::MODE, json!("light"));
    for mode in ["dark", "light", "auto"] {
        let _ = update(
            &mut app,
            Message::Theme(ThemeMsg::Option(crate::contracts::picker::theme_setting::MODE, mode)),
        );
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&app.config.config_path).unwrap()).unwrap();
        assert_eq!(saved["theme"]["mode"], mode);
        assert_eq!(saved["matugen"]["mode"], "light");
        let mut reopened = Config::from_data(json!({}));
        reopened.config_path.clone_from(&app.config.config_path);
        assert!(reopened.reload());
        app = App::with_config_using(reopened, |_| {
            crate::infrastructure::ipc::DaemonClient::recording()
        });
        assert_eq!(super::theme_bar_model(&app, "skwd-iris", false, 1).mode, mode);
        assert_eq!(SettingsSource::text(&app.config, skwd_config::keys::theme::MODE), mode);
    }
}

#[test]
fn filter_bar_intents_translate_at_composition() {
    use crate::app::Message;
    use crate::frontend::ui::{BarAction, BarIntent};

    assert!(matches!(
        bar_intent_message(BarIntent::Hover(Some(2), Some(3))),
        Message::BarHover(Some(2), Some(3))
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::Activate(BarAction::Resolution(String::from("4k")))),
        Message::SetResolution(value) if value == "4k"
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::SelectFolder(String::from("nature"))),
        Message::SetFolder(value) if value == "nature"
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::StepVolume(-10)),
        Message::Audio(crate::frontend::audio_panel::AudioMsg::VolumeStep(-10))
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::Theme(
            crate::frontend::theme_designer::ThemeMsg::BackendMenu
        )),
        Message::Theme(crate::frontend::theme_designer::ThemeMsg::BackendMenu)
    ));
}

#[test]
fn footprint_tracks_controls_and_scale() {
    let mut app = test_app();
    let (_, base_w, base_h) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(base_w > 0.0 && base_h > 0.0);

    for key in ["orient", "resolution", "colors", "theme", "random", "favourites", "folder"] {
        app.config.set_key(&format!("filterBar.show.{key}"), json!(false));
    }
    let (_, trimmed_w, _) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(trimmed_w < base_w, "{trimmed_w} vs {base_w}");

    app.config.set_key(skwd_config::keys::general::UI_SCALE, json!(2.0));
    let (_, scaled_w, scaled_h) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(scaled_w > trimmed_w && scaled_h > base_h, "{scaled_w}x{scaled_h}");
}

#[test]
fn open_menus_keep_footprint() {
    let mut app = test_app();
    app.library_session.folder_options =
        vec![String::from("nature"), String::from("nature/forest"), String::from("cities")];
    let closed = filter_bar_footprint(&app, 1200.0, 700.0);

    app.chrome.bar.menu = Some(crate::frontend::ui::MenuKind::Folders);
    assert_eq!(filter_bar_footprint(&app, 1200.0, 700.0), closed);

    app.theme.bar_open = true;
    let theme_closed = filter_bar_footprint(&app, 1200.0, 700.0);
    app.chrome.bar.menu = Some(crate::frontend::ui::MenuKind::Backends);
    assert_eq!(filter_bar_footprint(&app, 1200.0, 700.0), theme_closed);
}

#[test]
fn optional_filter_controls_persist_and_leave_no_bar_items() {
    use crate::app::{Message, update};
    use crate::frontend::settings::SettingsMsg;
    use crate::frontend::ui::{BarAction, build_bar_with_tasks, verticalize_bar};
    use crate::infrastructure::config::Config;
    use skwd_config::keys::filter_bar;

    for path in [
        filter_bar::SHOW_TAG_CLOUD,
        filter_bar::SHOW_ORIENT,
        filter_bar::SHOW_PLAYLISTS,
        filter_bar::SHOW_DOWNLOAD,
    ] {
        let mut app = test_app();
        let original_filters = app.library_session.filters.clone();
        for visible in [true, false, true] {
            let _ = update(&mut app, Message::Settings(SettingsMsg::Toggle(path.into(), visible)));
            let mut reopened = Config::from_data(json!({}));
            reopened.config_path.clone_from(&app.config.config_path);
            assert!(reopened.reload());
            app.config = reopened;
            assert_eq!(
                app.config.filter_show(path.strip_prefix("filterBar.show.").unwrap()),
                visible
            );
            let show = super::bar_show(&app);
            for downloads_enabled in [false, true] {
                for vertical in [false, true] {
                    let mut model = build_bar_with_tasks(
                        &app.library_session.filters,
                        &[],
                        false,
                        0,
                        0,
                        1.0,
                        downloads_enabled,
                        false,
                        false,
                        &show,
                        5000.0,
                        false,
                        false,
                        None,
                        &[],
                    );
                    if vertical {
                        verticalize_bar(&mut model, 500.0, 1.0);
                    }
                    for (key, present) in [
                        (
                            filter_bar::SHOW_TAG_CLOUD,
                            model
                                .items
                                .iter()
                                .any(|item| matches!(item.action, Some(BarAction::TagCloud))),
                        ),
                        (
                            filter_bar::SHOW_ORIENT,
                            model
                                .items
                                .iter()
                                .any(|item| matches!(item.action, Some(BarAction::Orient(_)))),
                        ),
                        (
                            filter_bar::SHOW_PLAYLISTS,
                            model
                                .items
                                .iter()
                                .any(|item| matches!(item.action, Some(BarAction::Playlists))),
                        ),
                        (
                            filter_bar::SHOW_DOWNLOAD,
                            model
                                .items
                                .iter()
                                .any(|item| matches!(item.action, Some(BarAction::Download))),
                        ),
                    ] {
                        let expected = (key != path || visible)
                            && (key != filter_bar::SHOW_DOWNLOAD || downloads_enabled);
                        assert_eq!(
                            present, expected,
                            "{path}={visible}, {key}, vertical={vertical}"
                        );
                    }
                    let settings = model
                        .items
                        .iter()
                        .position(|item| matches!(item.action, Some(BarAction::Settings)))
                        .expect("settings is always on the bar");
                    assert!(matches!(
                        model.items.get(settings + 1).and_then(|item| item.action.as_ref()),
                        Some(BarAction::Stash)
                    ));
                }
            }
            assert_eq!(app.library_session.filters.orient, original_filters.orient);
            assert_eq!(app.library_session.filters.tags, original_filters.tags);
        }
    }
}

#[test]
fn filter_bar_eye_stashes() {
    use crate::app::Message;
    use crate::frontend::ui::{BarAction, BarIntent};
    assert!(matches!(bar_intent_message(BarIntent::Activate(BarAction::Stash)), Message::Stash));
}

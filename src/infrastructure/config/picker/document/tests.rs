use std::os::unix::fs::PermissionsExt;

use super::Config;
use super::secure_config_file;
use serde_json::json;

#[test]
fn secure_private_noop() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(&path, b"{}").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();

    assert!(!secure_config_file(&path));
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o7777, 0o600);
}

#[test]
fn secure_public_once() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(&path, b"{}").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

    assert!(secure_config_file(&path));
    assert!(!secure_config_file(&path));
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o7777, 0o600);
}

#[test]
fn legacy_we_engine_migrates() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "weRender": {"engine": "compatibility", "native": false, "fps": 30}
    }));
    config.config_path = path.clone();

    assert_eq!(config.str_path(skwd_config::keys::we_render::ENGINE), "native");
    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(saved["weRender"]["engine"], "native");
    assert_eq!(saved["weRender"]["fps"], 30);
    assert!(saved["weRender"].get("native").is_none());
}

#[test]
fn legacy_paper_engine_migrates() {
    for retired in ["noctalia", "dms", "unknown"] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        let mut config = Config::from_data(json!({"paper": {"engine": retired}}));
        config.config_path = path.clone();

        config.persist();

        let saved: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(saved["paper"]["engine"], "skwd-paper");
    }
}

#[test]
fn resolution_presets_migrate() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "filterBar": {"resolutionPresets": [
            {"label": "FHD", "width": 1920, "height": 1080},
            {"label": "2K", "width": 2560, "height": 1440},
            {"label": "4K", "width": 3840, "height": 2160}
        ]}
    }));
    config.config_path = path.clone();

    assert_eq!(config.str_path("filterBar.resolutionPresets.0.from"), "1920x1080");
    assert_eq!(config.str_path("filterBar.resolutionPresets.0.to"), "2559x1439");
    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let preset = &saved["filterBar"]["resolutionPresets"][0];
    assert_eq!(preset["from"], "1920x1080");
    assert_eq!(preset["to"], "2559x1439");
    assert_eq!(preset["orientation"], "wide");
    assert!(preset.get("width").is_none());
    assert!(preset.get("height").is_none());
    assert_eq!(saved["filterBar"]["resolutionPresets"][1]["to"], "3839x2159");
    assert_eq!(saved["filterBar"]["resolutionPresets"][2]["to"], "");
}

#[test]
fn awww_engine_kept() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({"paper": {"engine": "awww"}}));
    config.config_path = path.clone();

    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(saved["paper"]["engine"], "awww");
}

#[test]
fn global_apply_button_migrates_without_overwriting_source_choices() {
    use crate::contracts::browser::Source;
    for previous in [true, false] {
        let dir = tempfile::tempdir().unwrap();
        let mut config = Config::from_data(json!({
            "sources": {
                "showApplyButton": previous,
                "wallhaven": {"showApplyButton": !previous},
                "unsplash": {"accessKey": "preserved"}
            }
        }));
        config.config_path = dir.path().join("config.json");
        assert_eq!(config.browser_apply_button(Source::Wallhaven), !previous);
        for source in [Source::Steam, Source::Unsplash, Source::Pexels, Source::Youtube] {
            assert_eq!(config.browser_apply_button(source), previous);
        }
        assert!(!config.browser_apply_button(Source::Bing));
        config.persist();
        let saved: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&config.config_path).unwrap()).unwrap();
        assert!(saved["sources"].get("showApplyButton").is_none());
        assert_eq!(saved["sources"]["unsplash"]["accessKey"], "preserved");
        let reopened = Config::from_data(saved);
        assert_eq!(reopened.browser_apply_button(Source::Wallhaven), !previous);
        assert_eq!(reopened.browser_apply_button(Source::Steam), previous);
    }
}

#[test]
fn legacy_backdrop_blur_splits_into_each_wallpaper_type_on_load_and_save() {
    use skwd_config::keys::niri;
    let dir = tempfile::tempdir().unwrap();
    let mut config = Config::from_data(json!({"niri": {
        "overviewBackdropBlurEnabled": false,
        "overviewBackdropBlur": 45
    }}));
    config.config_path = dir.path().join("config.json");
    for (toggle, radius) in [
        (niri::BACKDROP_BLUR_STATIC, niri::BACKDROP_BLUR_STATIC_RADIUS),
        (niri::BACKDROP_BLUR_VIDEO, niri::BACKDROP_BLUR_VIDEO_RADIUS),
        (niri::BACKDROP_BLUR_WE, niri::BACKDROP_BLUR_WE_RADIUS),
    ] {
        assert!(!config.flag_default_config(toggle));
        assert_eq!(config.num_path(radius), 45.0);
    }
    config.persist();
    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config.config_path).unwrap()).unwrap();
    assert!(saved["niri"].get("overviewBackdropBlurEnabled").is_none());
    assert!(saved["niri"].get("overviewBackdropBlur").is_none());
    assert_eq!(saved["niri"]["backdropBlurWe"], false);
    assert_eq!(saved["niri"]["backdropBlurVideoRadius"], 45);
}

#[test]
fn save_key_keeps_symlink() {
    let dir = tempfile::tempdir().unwrap();
    let dots = dir.path().join("dots");
    std::fs::create_dir(&dots).unwrap();
    let target = dots.join("config.json");
    std::fs::write(&target, b"{}").unwrap();
    let link = dir.path().join("config.json");
    std::os::unix::fs::symlink(&target, &link).unwrap();
    let mut config = Config::from_data(json!({}));
    config.config_path = link.clone();

    config.save_key(skwd_config::keys::launch::ANIMATION, json!("slide"));
    config.save_key(skwd_config::keys::launch::ANIMATION, json!("fade"));

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read_link(&link).unwrap(), target);
    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&target).unwrap()).unwrap();
    assert_eq!(saved["launch"]["animation"], "fade");
    assert_eq!(std::fs::metadata(&target).unwrap().permissions().mode() & 0o7777, 0o600);
    let leftovers = std::fs::read_dir(&dots)
        .unwrap()
        .filter_map(Result::ok)
        .any(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"));
    assert!(!leftovers);
}

#[test]
fn persist_keeps_relative_symlink_into_dotfiles() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let dots = dir.path().join("dotfiles");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&dots).unwrap();
    std::fs::write(dots.join("config.json"), b"{\"launch\":{\"animation\":\"slide\"}}").unwrap();
    let link = home.join("config.json");
    std::os::unix::fs::symlink("../dotfiles/config.json", &link).unwrap();
    let mut config = Config::from_data(json!({"launch": {"animation": "fade"}}));
    config.config_path = link.clone();

    config.persist();

    assert_eq!(std::fs::read_link(&link).unwrap(), std::path::Path::new("../dotfiles/config.json"));
    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dots.join("config.json")).unwrap()).unwrap();
    assert_eq!(saved["launch"]["animation"], "fade");
}

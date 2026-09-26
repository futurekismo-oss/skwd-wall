use serde_json::Value;

use super::common::{DecodeError, DecodeResult, envelope};
use crate::contracts::daemon::{AppThemeStatus, AppThemesResult};

pub fn decode_app_themes(value: &Value) -> DecodeResult<AppThemesResult> {
    envelope("theme.apps", value)?;
    let result: wall_proto::AppThemesResult =
        serde_json::from_value(value.clone()).map_err(|_| DecodeError::InvalidField {
            family: "theme.apps",
            field: "apps",
            expected: "app theme statuses",
        })?;
    Ok(AppThemesResult {
        apps: result
            .apps
            .into_iter()
            .map(|app| AppThemeStatus {
                id: app.id,
                name: app.name,
                installed: app.installed,
                config_found: app.config_found,
                config_path: app.config_path,
                output_path: app.output_path,
                enabled: app.enabled,
                state: app.state,
                detail: app.detail,
                can_enable: app.can_enable,
                can_disable: app.can_disable,
                can_adopt: app.can_adopt,
                template_path: app.template_path,
                customized: app.customized,
                can_disconnect: app.can_disconnect,
                can_reconnect: app.can_reconnect,
            })
            .collect(),
    })
}

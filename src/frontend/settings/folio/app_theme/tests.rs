use super::state_label;
use crate::i18n::tr;

#[test]
fn custom_outputs_are_named_without_mislabeling_other_theme_tools() {
    assert_eq!(
        state_label("conflict", "custom-output"),
        tr("settings-app-themes-custom-output-status")
    );
    assert_eq!(state_label("conflict", "noctalia"), tr("settings-app-themes-conflict"));
    assert_eq!(state_label("changed", "custom-output"), tr("settings-app-themes-changed"));
}

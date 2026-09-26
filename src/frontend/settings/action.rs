#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionId {
    SetThemeTarget(&'static str, bool),
    RefreshAppThemes,
    SetAppTheme(u8, bool),
    AdoptAppTheme(u8),
    RefreshAppTheme(u8),
    CustomizeAppTheme(u8, &'static str),
    CopyAppThemeTemplate(u8),
    ClearCache,
    CaptureWeThumbnails,
    RecomputeColors,
    OptimizeImages,
    RefreshBackdrop,
    CopyLayerRule,
    AddPostCommand,
    RemovePostCommand(u16),
    AddIntegration,
    RemoveIntegration(u16),
    ImportSemanticModel,
    AddSemanticModel,
    RemoveSemanticModel(u16),
    DeleteSemanticModel(u16),
    DeleteActiveSemanticModel,
    AddResolutionPreset,
    CreateResolutionPresetBand(&'static str),
    RemoveResolutionPreset(u16),
    OpenScheduleEditor,
    OpenThemeDesigner,
    ChooseRunningProcess,
    GenerateBugReport,
    ResetMotionFast,
    ResetMotionStandard,
    ResetMotionSlow,
    ResetKeybinds,
}

impl ActionId {
    pub const fn is_destructive(self) -> bool {
        matches!(
            self,
            Self::ClearCache
                | Self::OptimizeImages
                | Self::ResetKeybinds
                | Self::DeleteSemanticModel(_)
                | Self::DeleteActiveSemanticModel
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsFocus {
    Index,
    Sections,
    Controls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsKey {
    FocusNext { backwards: bool },
    CategoryNext { backwards: bool },
    Search,
    Previous,
    Next,
    Up,
    Down,
    Activate,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
    Input(String, String),
    ResolutionInput(String, bool, String),
    ResolutionLimit(String, bool),
    Toggle(String, bool),
    Commit,
    Pick(String, String),
    SelectSection(usize),
    LeaveLayoutStudio,
    FocusControl(usize),
    ToggleDetails(String, usize),
    ToggleBar(String, usize),
    SearchOpen,
    SearchClose,
    SearchInput(String),
    OpenSearchResult(String, usize, usize),
    SelectTab(String),
    Key(SettingsKey),
    Run(ActionId),
    KeybindCapture(String),
    KeybindCaptureCancel,
    KeybindCaptureApply,
    KeybindCaptureDefault,
    KeybindCaptureUnbind,
    KeybindCaptureClick(crate::domain::input::MouseButton),
    ProcessPickerClose,
    ProcessPickerSearch(String),
    ProcessPickerManualInput(String),
    ProcessPickerManualAdd,
    ProcessPickerScroll(f32),
    ProcessPickerAdd(String),
    ProcessPickerRemove(String),
}

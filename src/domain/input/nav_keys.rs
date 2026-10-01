use super::action::InputAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavKeys {
    Wasd,
    Vim,
}

impl NavKeys {
    pub const ALL: [Self; 2] = [Self::Wasd, Self::Vim];

    pub const fn second_keys(self) -> [(InputAction, &'static str); 4] {
        match self {
            Self::Wasd => [
                (InputAction::NavLeft, "a"),
                (InputAction::NavDown, "s"),
                (InputAction::NavUp, "w"),
                (InputAction::NavRight, "d"),
            ],
            Self::Vim => [
                (InputAction::NavLeft, "h"),
                (InputAction::NavDown, "j"),
                (InputAction::NavUp, "k"),
                (InputAction::NavRight, "l"),
            ],
        }
    }

    pub const fn label_key(self) -> &'static str {
        match self {
            Self::Wasd => "settings-keybinds-second-keys-wasd",
            Self::Vim => "settings-keybinds-second-keys-vim",
        }
    }
}

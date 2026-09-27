use iced::widget::{button, canvas, column, container, text};
use iced::{Color, Element, Length};

use crate::app::Message;
use crate::contracts::settings::{SettingsSource, keys};
use crate::domain::theme::{hex_to_hsv, hsv_to_hex};
use crate::frontend::theme::Palette;
use crate::frontend::ui::{label, row};
use crate::i18n::tr;

use super::SettingsMsg;

#[derive(Debug, Clone, Default)]
pub struct BackgroundControl {
    pub output: String,
    pub mode: String,
    pub color: String,
    pub inherited: bool,
}

impl BackgroundControl {
    pub fn new(cfg: &dyn SettingsSource, output: &str) -> Self {
        let mode = cfg.text(&format!("{}.{output}", keys::display::BACKGROUND_MODES));
        let inherited = !output.is_empty() && !matches!(mode.as_str(), "color" | "blur");
        let mode = if output.is_empty() || inherited {
            cfg.text(keys::display::BACKGROUND_MODE)
        } else {
            mode
        };
        let color = if output.is_empty() || inherited {
            String::new()
        } else {
            cfg.text(&format!("{}.{output}", keys::display::BACKGROUND_COLORS))
        };
        let color = if color.is_empty() { cfg.text(keys::display::FILL_COLOR) } else { color };
        let hex = color.trim_start_matches('#');
        let color =
            if matches!(hex.len(), 6 | 8) && hex.bytes().all(|value| value.is_ascii_hexdigit()) {
                format!("#{}", &hex[..6])
            } else {
                String::from("#000000")
            };
        Self { output: output.to_string(), mode, color, inherited }
    }

    pub fn button_width(scale: f32) -> f32 {
        let width = crate::frontend::ui::folio_action_width(tr("settings-background-color"), scale)
            + 28.0 * scale;
        width
            .max(92.0 * scale)
            .max(crate::frontend::ui::folio_action_width(tr("settings-background-blur"), scale))
            .max(crate::frontend::ui::folio_action_width(tr("settings-background-inherit"), scale))
    }

    pub fn choices_width(scale: f32) -> f32 {
        72.0 * scale + 3.0 * Self::button_width(scale) + 30.0 * scale
    }

    pub fn picker_id(&self) -> String {
        format!("background-picker:{}", self.output)
    }

    pub fn view<'a>(
        &self,
        reveal: f32,
        picker: f32,
        scale: f32,
        palette: &'a Palette,
    ) -> Element<'a, Message> {
        let swatch = crate::frontend::theme::parse_hex(&self.color).unwrap_or(Color::BLACK);
        let solid = self.mode != "blur";
        let width = Self::button_width(scale);
        let color = button(
            container(
                row![
                    container(text(""))
                        .width(Length::Fixed(20.0 * scale))
                        .height(Length::Fixed(20.0 * scale))
                        .style(move |_| crate::frontend::ui::box_style(swatch, palette.outline)),
                    label(
                        tr("settings-background-color"),
                        10.0,
                        scale,
                        if solid { palette.primary_text } else { palette.surface_text }
                    ),
                ]
                .spacing(8.0 * scale)
                .align_y(iced::Alignment::Center),
            )
            .center(Length::Fill),
        )
        .width(Length::Fixed(width))
        .height(Length::Fixed(crate::frontend::ui::FOLIO_BUTTON_HEIGHT * scale.max(1.0)))
        .padding([0.0, 8.0 * scale])
        .on_press(Message::Settings(SettingsMsg::BackgroundPicker(self.output.clone())))
        .style(move |_, status| {
            crate::frontend::ui::folio_button_style(solid, false, palette, status)
        });
        let mut choices = row![
            label(tr("settings-background-label"), 9.0, scale, palette.surface_text)
                .width(Length::Fixed(72.0 * scale)),
            color,
            crate::frontend::ui::folio_action(
                tr("settings-background-blur"),
                self.mode == "blur",
                Some(Message::Settings(SettingsMsg::BackgroundMode(
                    self.output.clone(),
                    String::from("blur")
                ))),
                Length::Fixed(width),
                scale,
                palette
            ),
        ]
        .spacing(10.0 * scale)
        .align_y(iced::Alignment::Center);
        if !self.output.is_empty() {
            choices = choices.push(crate::frontend::ui::folio_action(
                tr("settings-background-inherit"),
                self.inherited,
                Some(Message::Settings(SettingsMsg::BackgroundMode(
                    self.output.clone(),
                    String::from("inherit"),
                ))),
                Length::Fixed(width),
                scale,
                palette,
            ));
        }
        let mut body = column![choices].spacing(8.0 * scale);
        if picker > 0.0 {
            let hsv = hex_to_hsv(&self.color).unwrap_or_default();
            let output = self.output.clone();
            let wheel: Element<'_, Message> =
                canvas(crate::frontend::theme_designer::wheel::ColorWheel { hsv })
                    .width(Length::Fixed(176.0 * scale))
                    .height(Length::Fixed(176.0 * scale))
                    .into();
            let wheel = wheel.map(move |message| {
                let next = match message {
                    Message::Theme(crate::frontend::theme_designer::ThemeMsg::Hue(hue)) => {
                        hsv_to_hex(
                            hue,
                            if hsv.1 == 0.0 { 0.5 } else { hsv.1 },
                            if hsv.2 == 0.0 { 0.5 } else { hsv.2 },
                        )
                    }
                    Message::Theme(crate::frontend::theme_designer::ThemeMsg::SV(
                        saturation,
                        value,
                    )) => hsv_to_hex(hsv.0, saturation, value),
                    Message::Theme(crate::frontend::theme_designer::ThemeMsg::DragEnd) => {
                        return Message::Settings(SettingsMsg::BackgroundCommit(output.clone()));
                    }
                    message => return message,
                };
                Message::Settings(SettingsMsg::BackgroundColor(output.clone(), next))
            });
            body = body.push(
                container(
                    row![wheel, label(self.color.clone(), 12.0, scale, palette.surface_text)]
                        .spacing(14.0 * scale),
                )
                .height(Length::Fixed(
                    184.0 * scale * crate::frontend::animation::smoothstep(picker),
                ))
                .clip(true),
            );
        }
        container(body)
            .width(Length::Fill)
            .height(Length::Fixed(
                (52.0 + 192.0 * picker) * scale * crate::frontend::animation::smoothstep(reveal),
            ))
            .clip(true)
            .padding(iced::Padding { top: 8.0 * scale, ..iced::Padding::ZERO })
            .into()
    }
}

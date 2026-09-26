use iced::widget::{container, text};
use iced::{Element, Length};

use crate::app::Message;
use crate::frontend::components::with_alpha;
use crate::frontend::theme::Palette;

pub(super) fn thumb_placeholder<'a>(
    width: impl Into<Length>,
    height: impl Into<Length>,
    palette: &Palette,
) -> Element<'a, Message> {
    let background = with_alpha(palette.surface_variant, 0.4);
    container(text(""))
        .width(width)
        .height(height)
        .style(move |_| crate::frontend::ui::bg_style(background))
        .into()
}

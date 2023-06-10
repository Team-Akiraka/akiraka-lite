use druid::{Insets, Widget, WidgetExt};
use druid::widget::{Flex};
use crate::{AppState, Asset};
use crate::theme::theme;
use crate::widget::button::Button;
use crate::widget::clear_button::ClearButton;
use crate::widget::icon_clear_button::IconClearButton;
use crate::widget::launch_button::LaunchButton;
use crate::widget::primary_button::PrimaryButton;
use crate::widget::profile_button::ProfileButton;

pub fn build() -> impl Widget<AppState> {
    let launch_button = LaunchButton::new(
        std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        "Launch"
    )
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);
    // let launch_button = IconClearButton::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse::<String>().unwrap());

    let profile_button = ProfileButton::new()
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let bar = Flex::row()
        .with_child(profile_button)
        .with_flex_spacer(1.0)
        .with_child(launch_button)
        .center()
        .padding(Insets::new(12.0, 6.0, 12.0, 6.0))
        .fix_height(56.0)
        .background(theme::COLOR_BACKGROUND_DARK)
        .border(theme::COLOR_BORDER_LIGHT, 1.0)
        .expand_width();
    bar
}
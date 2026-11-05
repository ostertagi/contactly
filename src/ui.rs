//! Small reusable view helpers. They are generic over the message type `M`,
//! so any page or dialog can use them with its own `Message` enum.

use iced::widget::{
    button, center, column, container, mouse_area, opaque, row, stack, text, Column, Container,
};
use iced::{alignment, Background, Color, Element, Length, Theme};

use crate::model::Group;
use crate::route::Route;
use crate::theme;

// ------------------------------------------------------------------- spacing

/// Horizontal spacer (pushes siblings in a row apart).
pub fn fill<'a, M: 'a>() -> Element<'a, M> {
    container(text("")).width(Length::Fill).into()
}

/// Vertical spacer.
pub fn fill_v<'a, M: 'a>() -> Element<'a, M> {
    container(text("")).height(Length::Fill).into()
}

// ---------------------------------------------------------------------- text

pub fn heading<'a, M: 'a>(s: impl Into<String>) -> Element<'a, M> {
    text(s.into())
        .size(24.0)
        .font(theme::bold())
        .color(theme::ink())
        .into()
}

pub fn label<'a, M: 'a>(s: impl Into<String>) -> Element<'a, M> {
    text(s.into()).size(15.0).color(theme::ink()).into()
}

pub fn muted<'a, M: 'a>(s: impl Into<String>) -> Element<'a, M> {
    text(s.into()).size(13.0).color(theme::grey()).into()
}

// ------------------------------------------------------------------- buttons

type ButtonStyle = fn(&Theme, button::Status) -> button::Style;

pub fn action<'a, M: Clone + 'a>(
    label: impl Into<String>,
    style: ButtonStyle,
    msg: M,
) -> Element<'a, M> {
    button(text(label.into()).size(14.0))
        .padding(theme::pad(9.0, 16.0))
        .style(style)
        .on_press(msg)
        .into()
}

pub fn primary<'a, M: Clone + 'a>(label: impl Into<String>, msg: M) -> Element<'a, M> {
    action(label, theme::primary, msg)
}

pub fn secondary<'a, M: Clone + 'a>(label: impl Into<String>, msg: M) -> Element<'a, M> {
    action(label, theme::secondary, msg)
}

pub fn danger<'a, M: Clone + 'a>(label: impl Into<String>, msg: M) -> Element<'a, M> {
    action(label, theme::danger, msg)
}

pub fn chip<'a, M: Clone + 'a>(label: impl Into<String>, active: bool, msg: M) -> Element<'a, M> {
    button(text(label.into()).size(13.0))
        .padding(theme::pad(7.0, 14.0))
        .style(move |_t: &Theme, status| theme::chip(active, status))
        .on_press(msg)
        .into()
}

// -------------------------------------------------------------------- blocks

pub fn card<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Container<'a, M> {
    container(content).padding(20.0).style(theme::card)
}

pub fn badge<'a, M: 'a>(label: impl Into<String>, color: Color) -> Element<'a, M> {
    container(text(label.into()).size(12.0).color(color))
        .padding(theme::pad(3.0, 10.0))
        .style(move |_t: &Theme| theme::soft(color))
        .into()
}

pub fn group_badge<'a, M: 'a>(group: Group) -> Element<'a, M> {
    badge(group.to_string(), theme::group_color(group))
}

pub fn avatar<'a, M: 'a>(name: &str, size: f32) -> Element<'a, M> {
    let initial = name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());
    let color = theme::avatar_color(name);

    container(
        text(initial)
            .size(size * 0.42)
            .font(theme::bold())
            .color(Color::WHITE),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center)
    .style(move |_t: &Theme| theme::solid(color, size / 2.0))
    .into()
}

/// Green banner with a dismiss button.
pub fn notice<'a, M: Clone + 'a>(message: &str, on_dismiss: M) -> Element<'a, M> {
    container(
        row![
            text(message.to_string()).size(14.0).color(theme::ink()),
            fill(),
            secondary("Dismiss", on_dismiss),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding(theme::pad(8.0, 14.0))
    .width(Length::Fill)
    .style(theme::notice)
    .into()
}

// ------------------------------------------------------------------- sidebar

/// Dark navigation sidebar. `on_nav` turns a `Route` into the parent's message.
pub fn sidebar<'a, M: Clone + 'a>(
    active: &Route,
    user: &str,
    on_nav: impl Fn(Route) -> M,
) -> Element<'a, M> {
    let section = active.section();

    let mut nav = Column::new().spacing(4.0);
    for item in [Route::Dashboard, Route::Contacts, Route::Settings] {
        let is_active = section == item;
        nav = nav.push(
            button(text(item.title()).size(15.0))
                .width(Length::Fill)
                .padding(theme::pad(10.0, 14.0))
                .style(move |_t: &Theme, status| theme::nav_button(is_active, status))
                .on_press(on_nav(item)),
        );
    }

    let brand = text("Contactly")
        .size(22.0)
        .font(theme::bold())
        .color(Color::WHITE);

    let user_block = column![
        text("Signed in as").size(12.0).color(theme::rgb(0x8A93A6)),
        text(user.to_string()).size(14.0).color(Color::WHITE),
    ]
    .spacing(2.0);

    container(
        column![brand, nav, fill_v(), user_block]
            .spacing(28.0)
            .height(Length::Fill),
    )
    .padding(20.0)
    .width(Length::Fixed(230.0))
    .height(Length::Fill)
    .style(theme::sidebar)
    .into()
}

// --------------------------------------------------------------------- modal

/// Draws `dialog` on top of `base` with a dimmed backdrop.
///
/// `stack!` layers widgets. `opaque` stops mouse events from reaching the
/// widgets underneath, so the page behind the dialog is not clickable.
/// If `on_blur` is given, clicking the dark backdrop sends it.
pub fn modal<'a, M: Clone + 'a>(
    base: impl Into<Element<'a, M>>,
    dialog: impl Into<Element<'a, M>>,
    on_blur: Option<M>,
) -> Element<'a, M> {
    let backdrop = center(opaque(dialog)).style(|_t: &Theme| iced::widget::container::Style {
        background: Some(Background::Color(Color {
            a: 0.55,
            ..Color::BLACK
        })),
        ..iced::widget::container::Style::default()
    });

    let area = mouse_area(backdrop);
    let area = match on_blur {
        Some(msg) => area.on_press(msg),
        None => area,
    };

    stack![base.into(), opaque(area)].into()
}

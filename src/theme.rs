//! Theme inspired by revault-gui: dark sidebar, light content area, white
//! rounded cards, green primary accent, red for danger.
//!
//! All colours are constants at the top, so you can re-skin the app by
//! editing only this block.
//!
//! Two Iced concepts are shown here:
//!   1. `Theme::custom(..)` with a `Palette`, which drives default widgets
//!      such as text_input and pick_list.
//!   2. Style functions (`fn(&Theme, Status) -> Style`) for widgets you want to
//!      look exactly your way, such as buttons and containers.

use iced::theme::Palette;
use iced::widget::{button, container};
use iced::{border, font, Background, Border, Color, Font, Padding, Theme};

use crate::model::Group;

const BACKGROUND: u32 = 0xF2F4F7;
const CARD: u32 = 0xFFFFFF;
const SIDEBAR: u32 = 0x12161F;
const SIDEBAR_HOVER: u32 = 0x1C2230;
const SIDEBAR_ACTIVE: u32 = 0x252D3F;
const INK: u32 = 0x161B26;
const GREY: u32 = 0x6B7385;
const BORDER: u32 = 0xE1E5EB;
const GREEN: u32 = 0x00B86B;
const RED: u32 = 0xF04359;
const AMBER: u32 = 0xFFA700;
const BLUE: u32 = 0x3B82F6;
const PURPLE: u32 = 0x8B5CF6;

pub fn rgb(hex: u32) -> Color {
    Color::from_rgb8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub fn ink() -> Color {
    rgb(INK)
}
pub fn grey() -> Color {
    rgb(GREY)
}
pub fn red() -> Color {
    rgb(RED)
}
pub fn green() -> Color {
    rgb(GREEN)
}
pub fn amber() -> Color {
    rgb(AMBER)
}

pub fn pad(vertical: f32, horizontal: f32) -> Padding {
    Padding {
        top: vertical,
        right: horizontal,
        bottom: vertical,
        left: horizontal,
    }
}

pub fn bold() -> Font {
    Font {
        weight: font::Weight::Bold,
        ..Font::DEFAULT
    }
}

pub fn app_theme() -> Theme {
    Theme::custom(
        "Contactly",
        Palette {
            background: rgb(BACKGROUND),
            text: rgb(INK),
            primary: rgb(GREEN),
            success: rgb(GREEN),
            warning: rgb(AMBER),
            danger: rgb(RED),
        },
    )
}

pub fn group_color(group: Group) -> Color {
    match group {
        Group::Family => rgb(PURPLE),
        Group::Friends => rgb(GREEN),
        Group::Work => rgb(BLUE),
    }
}

pub fn avatar_color(name: &str) -> Color {
    const COLORS: [u32; 5] = [BLUE, PURPLE, GREEN, 0xE8590C, 0x0EA5A4];
    let sum: usize = name.bytes().map(|b| b as usize).sum();
    rgb(COLORS[sum % COLORS.len()])
}

// ---------------------------------------------------------------- containers

pub fn page(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(rgb(BACKGROUND))),
        ..container::Style::default()
    }
}

pub fn card(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(rgb(CARD))),
        border: Border {
            color: rgb(BORDER),
            width: 1.0,
            ..border::rounded(12.0)
        },
        ..container::Style::default()
    }
}

pub fn sidebar(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(rgb(SIDEBAR))),
        text_color: Some(Color::WHITE),
        ..container::Style::default()
    }
}

pub fn notice(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color {
            a: 0.12,
            ..rgb(GREEN)
        })),
        border: Border {
            color: rgb(GREEN),
            width: 1.0,
            ..border::rounded(10.0)
        },
        ..container::Style::default()
    }
}

/// Soft tinted pill, used by badges.
pub fn soft(color: Color) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color { a: 0.14, ..color })),
        border: border::rounded(20.0),
        ..container::Style::default()
    }
}

/// Solid filled shape, used by avatars.
pub fn solid(color: Color, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(color)),
        border: border::rounded(radius),
        ..container::Style::default()
    }
}

// ------------------------------------------------------------------- buttons

fn shade(c: Color, factor: f32) -> Color {
    Color {
        r: (c.r * factor).min(1.0),
        g: (c.g * factor).min(1.0),
        b: (c.b * factor).min(1.0),
        a: c.a,
    }
}

fn filled(bg: Color, fg: Color, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Active => bg,
        button::Status::Hovered => shade(bg, 0.92),
        button::Status::Pressed => shade(bg, 0.82),
        button::Status::Disabled => Color { a: 0.4, ..bg },
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: fg,
        border: border::rounded(8.0),
        ..button::Style::default()
    }
}

pub fn primary(_: &Theme, status: button::Status) -> button::Style {
    filled(rgb(GREEN), Color::WHITE, status)
}

pub fn danger(_: &Theme, status: button::Status) -> button::Style {
    filled(rgb(RED), Color::WHITE, status)
}

pub fn secondary(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered | button::Status::Pressed => rgb(0xEEF0F4),
        _ => Color::WHITE,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: rgb(INK),
        border: Border {
            color: rgb(BORDER),
            width: 1.0,
            ..border::rounded(8.0)
        },
        ..button::Style::default()
    }
}

/// Filter chip: dark when active.
pub fn chip(active: bool, status: button::Status) -> button::Style {
    if active {
        filled(rgb(INK), Color::WHITE, status)
    } else {
        secondary(&Theme::Light, status)
    }
}

/// Sidebar entry.
pub fn nav_button(active: bool, status: button::Status) -> button::Style {
    let bg = if active {
        Some(Background::Color(rgb(SIDEBAR_ACTIVE)))
    } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        Some(Background::Color(rgb(SIDEBAR_HOVER)))
    } else {
        None
    };
    button::Style {
        background: bg,
        text_color: if active { Color::WHITE } else { rgb(0xA9B1C0) },
        border: border::rounded(8.0),
        ..button::Style::default()
    }
}

/// Invisible button that highlights on hover. Used for clickable list rows.
pub fn row_button(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered | button::Status::Pressed => Some(Background::Color(rgb(0xF2F4F7))),
        _ => None,
    };
    button::Style {
        background: bg,
        text_color: rgb(INK),
        border: border::rounded(8.0),
        ..button::Style::default()
    }
}

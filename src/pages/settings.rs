//! Settings: account info + the actions that open dialogs
//! (info -> About, alert/confirm -> Clear all / Log out).

use iced::widget::{column, row, text};
use iced::{alignment, Element, Length};

use super::Event;
use crate::model::Session;
use crate::{theme, ui};

#[derive(Debug, Clone)]
pub enum Message {
    About,
    ClearAll,
    Logout,
}

pub fn update(message: Message) -> Option<Event> {
    Some(match message {
        Message::About => Event::ShowAbout,
        Message::ClearAll => Event::RequestClearAll,
        Message::Logout => Event::RequestLogout,
    })
}

pub fn view<'a>(session: &'a Session, contact_count: usize) -> Element<'a, Message> {
    let account = ui::card(
        row![
            ui::avatar(&session.display_name, 56.0),
            column![
                text(session.display_name.as_str())
                    .size(20.0)
                    .font(theme::bold())
                    .color(theme::ink()),
                ui::muted(format!("@{}", session.username)),
            ]
            .spacing(3.0),
            ui::fill(),
            ui::danger("Log out", Message::Logout),
        ]
        .spacing(16.0)
        .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill);

    let data = ui::card(
        column![
            ui::heading("Your data"),
            ui::label(format!(
                "{contact_count} contacts stored in data/contacts_{}.json",
                session.username
            )),
            row![
                ui::secondary("About this app", Message::About),
                ui::danger("Delete all contacts", Message::ClearAll),
            ]
            .spacing(8.0),
        ]
        .spacing(14.0),
    )
    .width(Length::Fill);

    column![ui::heading("Settings"), account, data]
        .spacing(16.0)
        .into()
}

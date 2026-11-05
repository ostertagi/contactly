//! Dashboard: a *stateless* page. It owns no state, so it is just two free
//! functions. Everything it shows is derived from the shared contact list
//! that `App` passes into `view`.

use iced::widget::{button, column, row, text, Column, Row};
use iced::{alignment, Element, Length};

use super::Event;
use crate::model::{Contact, Group, Session};
use crate::route::Route;
use crate::{theme, ui};

#[derive(Debug, Clone)]
pub enum Message {
    NewContact,
    SeeAll,
    Open(u64),
}

pub fn update(message: Message) -> Option<Event> {
    Some(match message {
        Message::NewContact => Event::NewContact,
        Message::SeeAll => Event::Navigate(Route::Contacts),
        // Navigation with a parameter: /contacts/:id
        Message::Open(id) => Event::Navigate(Route::ContactDetail(id)),
    })
}

pub fn view<'a>(session: &'a Session, contacts: &'a [Contact]) -> Element<'a, Message> {
    let header = row![
        column![
            ui::heading(format!("Welcome back, {}", session.display_name)),
            ui::muted("Here is a summary of your contact book"),
        ]
        .spacing(2.0),
        ui::fill(),
        ui::primary("New contact", Message::NewContact),
    ]
    .align_y(alignment::Vertical::Center);

    let favorites = contacts.iter().filter(|c| c.favorite).count();
    let mut stats = Row::new()
        .spacing(16.0)
        .push(stat("Total contacts", contacts.len()))
        .push(stat("Favorites", favorites));
    for group in Group::ALL {
        let n = contacts.iter().filter(|c| c.group == *group).count();
        stats = stats.push(stat(&group.to_string(), n));
    }

    let mut recent = Column::new().spacing(4.0);
    if contacts.is_empty() {
        recent = recent.push(ui::muted("No contacts yet. Create your first one!"));
    }
    // New contacts are pushed at the end of the Vec, so the newest come last.
    for c in contacts.iter().rev().take(5) {
        recent = recent.push(
            button(
                row![
                    ui::avatar(&c.name, 36.0),
                    column![
                        text(c.name.as_str()).size(15.0).color(theme::ink()),
                        ui::muted(c.email.clone()),
                    ]
                    .spacing(2.0),
                    ui::fill(),
                    ui::group_badge(c.group),
                ]
                .spacing(12.0)
                .align_y(alignment::Vertical::Center),
            )
            .width(Length::Fill)
            .padding(theme::pad(8.0, 10.0))
            .style(theme::row_button)
            .on_press(Message::Open(c.id)),
        );
    }

    let recent_card = ui::card(
        column![
            row![
                ui::heading("Recently added"),
                ui::fill(),
                ui::secondary("See all", Message::SeeAll),
            ]
            .align_y(alignment::Vertical::Center),
            recent,
        ]
        .spacing(14.0),
    )
    .width(Length::Fill);

    column![header, stats, recent_card].spacing(20.0).into()
}

fn stat<'a>(label: &str, value: usize) -> Element<'a, Message> {
    ui::card(
        column![
            ui::muted(label),
            text(value.to_string())
                .size(34.0)
                .font(theme::bold())
                .color(theme::ink()),
        ]
        .spacing(6.0),
    )
    .width(Length::Fill)
    .into()
}

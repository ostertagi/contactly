//! Contact detail: the page behind the parameterised route
//! `Route::ContactDetail(id)`.
//!
//! The page keeps no state. The only thing it needs is the `id` from the
//! route, and it looks the contact up in the shared list on every `view`.
//! If the contact was deleted meanwhile, we render a "not found" state
//! instead of crashing.

use iced::widget::{column, row, text, Row};
use iced::{alignment, Element, Length};

use super::Event;
use crate::model::Contact;
use crate::{theme, ui};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Edit(u64),
    Delete(u64),
    ToggleFavorite(u64),
}

pub fn update(message: Message) -> Option<Event> {
    Some(match message {
        Message::Back => Event::Back,
        Message::Edit(id) => Event::EditContact(id),
        Message::Delete(id) => Event::DeleteContact(id),
        Message::ToggleFavorite(id) => Event::ToggleFavorite(id),
    })
}

pub fn view<'a>(contacts: &'a [Contact], id: u64) -> Element<'a, Message> {
    let back = ui::secondary("< Back", Message::Back);

    let Some(c) = contacts.iter().find(|c| c.id == id) else {
        return column![
            back,
            ui::card(ui::muted("This contact no longer exists.")).width(Length::Fill),
        ]
        .spacing(16.0)
        .into();
    };

    let mut tags = Row::new()
        .spacing(8.0)
        .push(ui::group_badge(c.group));
    if c.favorite {
        tags = tags.push(ui::badge("Favorite", theme::amber()));
    }

    let header = row![
        ui::avatar(&c.name, 72.0),
        column![
            text(c.name.as_str())
                .size(28.0)
                .font(theme::bold())
                .color(theme::ink()),
            tags,
        ]
        .spacing(8.0),
    ]
    .spacing(20.0)
    .align_y(alignment::Vertical::Center);

    let details = column![
        field("Email", &c.email),
        field("Phone", &c.phone),
        field("Group", c.group.to_string()),
        field("Contact ID", format!("#{}", c.id)),
    ]
    .spacing(14.0);

    let actions = row![
        ui::primary("Edit", Message::Edit(id)),
        ui::secondary(
            if c.favorite { "Remove favorite" } else { "Mark as favorite" },
            Message::ToggleFavorite(id)
        ),
        ui::fill(),
        ui::danger("Delete", Message::Delete(id)),
    ]
    .spacing(8.0);

    column![
        back,
        ui::card(column![header, details, actions].spacing(24.0)).width(Length::Fill),
    ]
    .spacing(16.0)
    .into()
}

fn field<'a>(label: &str, value: impl Into<String>) -> Element<'a, Message> {
    let value = value.into();
    let value = if value.trim().is_empty() {
        "Not set".to_string()
    } else {
        value
    };
    column![
        ui::muted(label),
        text(value).size(16.0).color(theme::ink()),
    ]
    .spacing(3.0)
    .into()
}

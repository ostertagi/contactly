//! Contacts list: search + filters + row actions.
//!
//! Stateful page. It owns only *view state* (query text, selected filter).
//! The contacts themselves belong to `App` and are passed to `view()` as a
//! shared reference. That is the "single source of truth" pattern.
//!
//! Because `App` keeps this struct alive between navigations, your search and
//! filter survive a trip to a detail page and back. Contrast with a page that
//! `App` re-creates on every navigation, where that state would be lost.

use iced::widget::{column, row, scrollable, text, text_input, Column, Row};
use iced::{alignment, Element, Length};

use super::Event;
use crate::model::{Contact, Group};
use crate::{theme, ui};

#[derive(Debug, Default)]
pub struct Contacts {
    query: String,
    group: Option<Group>,
    favorites_only: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    QueryChanged(String),
    GroupSelected(Option<Group>),
    FavoritesToggled,
    ClearFilters,
    Open(u64),
    New,
    Edit(u64),
    Delete(u64),
    ToggleFavorite(u64),
}

impl Contacts {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        use crate::route::Route;
        match message {
            // Purely local state changes: no Event needed.
            Message::QueryChanged(q) => {
                self.query = q;
                None
            }
            Message::GroupSelected(g) => {
                self.group = g;
                None
            }
            Message::FavoritesToggled => {
                self.favorites_only = !self.favorites_only;
                None
            }
            Message::ClearFilters => {
                *self = Contacts::default();
                None
            }
            // Anything touching shared data or navigation goes up as an Event.
            Message::Open(id) => Some(Event::Navigate(Route::ContactDetail(id))),
            Message::New => Some(Event::NewContact),
            Message::Edit(id) => Some(Event::EditContact(id)),
            Message::Delete(id) => Some(Event::DeleteContact(id)),
            Message::ToggleFavorite(id) => Some(Event::ToggleFavorite(id)),
        }
    }

    pub fn view<'a>(&'a self, contacts: &'a [Contact]) -> Element<'a, Message> {
        // Filtering happens at view time, so the list is always consistent with
        // both the data and the filters. No second copy to keep in sync.
        let mut visible: Vec<&Contact> = contacts
            .iter()
            .filter(|c| c.matches(&self.query, self.group, self.favorites_only))
            .collect();
        visible.sort_by_key(|c| c.name.to_lowercase());

        let header = row![
            column![
                ui::heading("Contacts"),
                ui::muted(format!("{} of {} shown", visible.len(), contacts.len())),
            ]
            .spacing(2.0),
            ui::fill(),
            ui::primary("New contact", Message::New),
        ]
        .align_y(alignment::Vertical::Center);

        let mut chips = Row::new().spacing(8.0).push(ui::chip(
            "All",
            self.group.is_none(),
            Message::GroupSelected(None),
        ));
        for g in Group::ALL {
            chips = chips.push(ui::chip(
                g.to_string(),
                self.group == Some(*g),
                Message::GroupSelected(Some(*g)),
            ));
        }
        chips = chips.push(ui::chip(
            "Favorites",
            self.favorites_only,
            Message::FavoritesToggled,
        ));

        let filters = row![
            text_input("Search by name, email or phone", &self.query)
                .on_input(Message::QueryChanged)
                .padding(10.0)
                .width(Length::FillPortion(3)),
            chips,
        ]
        .spacing(16.0)
        .align_y(alignment::Vertical::Center);

        let list: Element<'a, Message> = if visible.is_empty() {
            let note = if contacts.is_empty() {
                "No contacts yet. Click \"New contact\" to add one."
            } else {
                "No contacts match your search or filters."
            };
            ui::card(
                column![
                    ui::label(note),
                    ui::secondary("Clear filters", Message::ClearFilters),
                ]
                .spacing(12.0),
            )
            .width(Length::Fill)
            .into()
        } else {
            let mut col = Column::new().spacing(8.0);
            for c in visible {
                col = col.push(contact_row(c));
            }
            scrollable(col)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        column![header, filters, list].spacing(16.0).into()
    }
}

fn contact_row(c: &Contact) -> Element<'_, Message> {
    let mut name = Row::new()
        .spacing(8.0)
        .align_y(alignment::Vertical::Center)
        .push(
            text(c.name.as_str())
                .size(16.0)
                .font(theme::bold())
                .color(theme::ink()),
        );
    if c.favorite {
        name = name.push(ui::badge("Favorite", theme::amber()));
    }

    let info = column![name, ui::muted(c.email.clone())]
        .spacing(3.0)
        .width(Length::FillPortion(4));

    let phone = text(c.phone.as_str())
        .size(14.0)
        .color(theme::ink())
        .width(Length::FillPortion(3));

    let actions = row![
        ui::secondary("View", Message::Open(c.id)),
        ui::secondary(
            if c.favorite { "Unstar" } else { "Star" },
            Message::ToggleFavorite(c.id)
        ),
        ui::secondary("Edit", Message::Edit(c.id)),
        ui::danger("Delete", Message::Delete(c.id)),
    ]
    .spacing(6.0);

    ui::card(
        row![
            ui::avatar(&c.name, 42.0),
            info,
            phone,
            ui::group_badge(c.group),
            actions,
        ]
        .spacing(16.0)
        .align_y(alignment::Vertical::Center),
    )
    .padding(14.0)
    .width(Length::Fill)
    .into()
}

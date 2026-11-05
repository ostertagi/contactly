//! Login page: local form state + an async `Task`.
//!
//! This is the only page whose `update` returns a `Task`, because it starts
//! background work (the fake server call).

use iced::widget::{button, center, text, text_input, Column};
use iced::{alignment, Element, Length, Task};

use super::Event;
use crate::model::Session;
use crate::{auth, theme, ui};

#[derive(Debug, Default)]
pub struct Login {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
    /// Sent back by the Task when the async call finishes.
    Authenticated(Result<Session, String>),
}

impl Login {
    pub fn update(&mut self, message: Message) -> (Task<Message>, Option<Event>) {
        match message {
            Message::UsernameChanged(v) => {
                self.username = v;
                self.error = None;
                (Task::none(), None)
            }
            Message::PasswordChanged(v) => {
                self.password = v;
                self.error = None;
                (Task::none(), None)
            }
            Message::Submit => {
                if self.loading {
                    return (Task::none(), None);
                }
                self.loading = true;
                self.error = None;
                let task = Task::perform(
                    auth::login(self.username.clone(), self.password.clone()),
                    Message::Authenticated,
                );
                (task, None)
            }
            Message::Authenticated(Ok(session)) => {
                (Task::none(), Some(Event::LoggedIn(session)))
            }
            Message::Authenticated(Err(error)) => {
                self.loading = false;
                self.error = Some(error);
                (Task::none(), None)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut form = Column::new()
            .spacing(14.0)
            .push(
                text("Contactly")
                    .size(30.0)
                    .font(theme::bold())
                    .color(theme::ink()),
            )
            .push(ui::muted("Sign in to manage your contacts"))
            .push(
                text_input("Username", &self.username)
                    .on_input(Message::UsernameChanged)
                    .on_submit(Message::Submit)
                    .padding(10.0),
            )
            .push(
                text_input("Password", &self.password)
                    .on_input(Message::PasswordChanged)
                    .on_submit(Message::Submit)
                    .secure(true)
                    .padding(10.0),
            );

        if let Some(error) = &self.error {
            form = form.push(text(error.as_str()).size(13.0).color(theme::red()));
        }

        let label = if self.loading { "Signing in..." } else { "Sign in" };
        form = form
            .push(
                button(
                    text(label)
                        .size(15.0)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .padding(theme::pad(11.0, 16.0))
                .style(theme::primary)
                .on_press_maybe((!self.loading).then_some(Message::Submit)),
            )
            .push(ui::muted("Demo accounts: demo / demo   or   admin / admin123"));

        center(
            ui::card(form)
                .padding(32.0)
                .width(Length::Fixed(400.0)),
        )
        .style(theme::page)
        .into()
    }
}

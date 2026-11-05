//! Modal dialogs: Info, Confirm (alert) and a Form.
//!
//! A dialog is not a window. It is just `Option<Dialog>` in `App`; when it is
//! `Some`, `App::view` draws it on top of the current page.
//!
//! Data flow:
//!   App --(Message::Dialog)--> Dialog::update --(Outcome)--> App
//! The dialog never touches app data. It only reports what the user decided,
//! and `App` applies the change.

use iced::widget::{column, container, pick_list, row, text, text_input, Column};
use iced::{alignment, Element, Length};

use crate::model::{Contact, Group};
use crate::{theme, ui};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfirmAction {
    DeleteContact(u64),
    ClearAll,
    Logout,
}

#[derive(Debug, Clone)]
pub struct ContactForm {
    /// `None` = creating, `Some(id)` = editing.
    pub id: Option<u64>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub group: Group,
    pub error: Option<String>,
}

impl ContactForm {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            email: String::new(),
            phone: String::new(),
            group: Group::Friends,
            error: None,
        }
    }

    pub fn edit(contact: &Contact) -> Self {
        Self {
            id: Some(contact.id),
            name: contact.name.clone(),
            email: contact.email.clone(),
            phone: contact.phone.clone(),
            group: contact.group,
            error: None,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name is required".into());
        }
        let email = self.email.trim();
        let phone = self.phone.trim();
        if email.is_empty() && phone.is_empty() {
            return Err("Provide at least an email or a phone number".into());
        }
        if !email.is_empty() && !(email.contains('@') && email.contains('.')) {
            return Err("Email address looks invalid".into());
        }
        if !phone.chars().all(|c| c.is_ascii_digit() || " +-().".contains(c)) {
            return Err("Phone may only contain digits, spaces and + - ( )".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Dialog {
    Info {
        title: String,
        body: String,
    },
    Confirm {
        title: String,
        body: String,
        confirm_label: String,
        action: ConfirmAction,
    },
    ContactForm(ContactForm),
}

#[derive(Debug, Clone)]
pub enum Message {
    /// OK / Cancel / backdrop click.
    Close,
    /// "Confirm" in an alert dialog.
    Accept,
    NameChanged(String),
    EmailChanged(String),
    PhoneChanged(String),
    GroupChanged(Group),
    Submit,
}

/// What the dialog tells `App` once the user is done.
pub enum Outcome {
    Dismissed,
    Confirmed(ConfirmAction),
    Saved(ContactForm),
}

impl Dialog {
    pub fn info(title: impl Into<String>, body: impl Into<String>) -> Self {
        Dialog::Info {
            title: title.into(),
            body: body.into(),
        }
    }

    pub fn confirm(
        title: impl Into<String>,
        body: impl Into<String>,
        confirm_label: impl Into<String>,
        action: ConfirmAction,
    ) -> Self {
        Dialog::Confirm {
            title: title.into(),
            body: body.into(),
            confirm_label: confirm_label.into(),
            action,
        }
    }

    /// Forms should not vanish (and lose typed data) on a stray backdrop click.
    pub fn dismissable_by_backdrop(&self) -> bool {
        !matches!(self, Dialog::ContactForm(_))
    }

    pub fn update(&mut self, message: Message) -> Option<Outcome> {
        match message {
            Message::Close => Some(Outcome::Dismissed),

            Message::Accept => match self {
                Dialog::Confirm { action, .. } => Some(Outcome::Confirmed(*action)),
                _ => Some(Outcome::Dismissed),
            },

            Message::Submit => match self {
                Dialog::ContactForm(form) => match form.validate() {
                    Ok(()) => Some(Outcome::Saved(form.clone())),
                    Err(e) => {
                        form.error = Some(e);
                        None
                    }
                },
                _ => None,
            },

            Message::NameChanged(v) => {
                if let Dialog::ContactForm(f) = self {
                    f.name = v;
                    f.error = None;
                }
                None
            }
            Message::EmailChanged(v) => {
                if let Dialog::ContactForm(f) = self {
                    f.email = v;
                    f.error = None;
                }
                None
            }
            Message::PhoneChanged(v) => {
                if let Dialog::ContactForm(f) = self {
                    f.phone = v;
                    f.error = None;
                }
                None
            }
            Message::GroupChanged(g) => {
                if let Dialog::ContactForm(f) = self {
                    f.group = g;
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Dialog::Info { title, body } => frame(
                column![
                    ui::heading(title.as_str()),
                    ui::label(body.as_str()),
                    buttons([ui::primary("OK", Message::Close)]),
                ]
                .spacing(16.0),
            ),

            Dialog::Confirm {
                title,
                body,
                confirm_label,
                ..
            } => frame(
                column![
                    ui::heading(title.as_str()),
                    ui::label(body.as_str()),
                    buttons([
                        ui::secondary("Cancel", Message::Close),
                        ui::danger(confirm_label.as_str(), Message::Accept),
                    ]),
                ]
                .spacing(16.0),
            ),

            Dialog::ContactForm(form) => form_view(form),
        }
    }
}

// --------------------------------------------------------------------- views

fn frame<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(24.0)
        .width(Length::Fixed(440.0))
        .style(theme::card)
        .into()
}

/// Right-aligned button row.
fn buttons<'a, const N: usize>(items: [Element<'a, Message>; N]) -> Element<'a, Message> {
    let mut r = row![ui::fill()].spacing(8.0);
    for item in items {
        r = r.push(item);
    }
    r.into()
}

fn field<'a>(
    label: &'a str,
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        ui::muted(label),
        text_input(placeholder, value)
            .on_input(on_input)
            .on_submit(Message::Submit)
            .padding(10.0),
    ]
    .spacing(6.0)
    .into()
}

fn form_view(form: &ContactForm) -> Element<'_, Message> {
    let title = if form.id.is_some() {
        "Edit contact"
    } else {
        "New contact"
    };

    let mut body = Column::new()
        .spacing(14.0)
        .push(ui::heading(title))
        .push(field("Name", "Jane Doe", &form.name, Message::NameChanged))
        .push(field(
            "Email",
            "jane@example.com",
            &form.email,
            Message::EmailChanged,
        ))
        .push(field(
            "Phone",
            "+212 600 000 000",
            &form.phone,
            Message::PhoneChanged,
        ))
        .push(
            column![
                ui::muted("Group"),
                pick_list(Group::ALL, Some(form.group), Message::GroupChanged)
                    .width(Length::Fill)
                    .padding(10.0),
            ]
            .spacing(6.0),
        );

    if let Some(error) = &form.error {
        body = body.push(text(error.as_str()).size(13.0).color(theme::red()));
    }

    body = body.push(
        row![
            ui::fill(),
            ui::secondary("Cancel", Message::Close),
            ui::primary("Save", Message::Submit),
        ]
        .spacing(8.0)
        .align_y(alignment::Vertical::Center),
    );

    frame(body)
}

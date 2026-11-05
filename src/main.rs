//! Contactly: a small Iced 0.14 app to learn
//!   * state management  (`App` owns everything shared)
//!   * navigation        (`Route` + history stack + auth guard)
//!   * data transfer     (page Message -> Event -> App -> shared state)
//!   * dialogs           (info / alert / form, as an overlay)
//!   * async work        (Task::perform for login and saving)
//!
//! Read in this order: main.rs -> pages/mod.rs -> pages/contacts.rs ->
//! dialogs.rs -> ui.rs / theme.rs.

mod auth;
mod dialogs;
mod model;
mod pages;
mod route;
mod store;
mod theme;
mod ui;

use iced::widget::{container, row, Column};
use iced::{Element, Length, Task, Theme};

use dialogs::{ConfirmAction, ContactForm, Dialog, Outcome};
use model::{Contact, Session};
use pages::{contacts, dashboard, detail, login, settings, Event};
use route::Route;

pub fn main() -> iced::Result {
    // iced 0.14: application(boot, update, view), then configure with builders.
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .window_size((1100.0, 720.0))
        .centered()
        .run()
}

// =========================================================================
// State
// =========================================================================

struct App {
    // --- authentication: `None` means "show the login page" (the auth guard)
    session: Option<Session>,

    // --- shared data: single source of truth, pages only borrow it
    contacts: Vec<Contact>,

    // --- navigation
    route: Route,
    history: Vec<Route>,

    // --- per-page state that must survive navigation
    login: login::Login,
    contacts_page: contacts::Contacts,

    // --- overlays / feedback
    dialog: Option<Dialog>,
    notice: Option<String>,
}

// =========================================================================
// Messages: one variant per "sender". Child messages are wrapped, then
// unwrapped in `update`. `Element::map` does the wrapping in `view`.
// =========================================================================

#[derive(Debug, Clone)]
enum Message {
    Login(login::Message),
    Dashboard(dashboard::Message),
    Contacts(contacts::Message),
    Detail(detail::Message),
    Settings(settings::Message),
    Dialog(dialogs::Message),

    /// Top-level navigation (sidebar): resets history.
    Sidebar(Route),
    DismissNotice,
    /// Result of a background save.
    Saved(Result<(), String>),
}

impl App {
    fn new() -> Self {
        Self {
            session: None,
            contacts: Vec::new(),
            route: Route::Dashboard,
            history: Vec::new(),
            login: login::Login::default(),
            contacts_page: contacts::Contacts::default(),
            dialog: None,
            notice: None,
        }
    }

    fn title(&self) -> String {
        match self.session {
            Some(_) => format!("Contactly - {}", self.route.title()),
            None => "Contactly - Sign in".to_string(),
        }
    }

    fn theme(&self) -> Theme {
        theme::app_theme()
    }

    // =====================================================================
    // Update
    // =====================================================================

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(msg) => {
                let (task, event) = self.login.update(msg);
                let task = task.map(Message::Login);
                match event {
                    Some(event) => Task::batch([task, self.handle(event)]),
                    None => task,
                }
            }
            Message::Dashboard(msg) => self.on_event(dashboard::update(msg)),
            Message::Contacts(msg) => {
                let event = self.contacts_page.update(msg);
                self.on_event(event)
            }
            Message::Detail(msg) => self.on_event(detail::update(msg)),
            Message::Settings(msg) => self.on_event(settings::update(msg)),

            Message::Dialog(msg) => {
                let Some(dialog) = self.dialog.as_mut() else {
                    return Task::none();
                };
                match dialog.update(msg) {
                    None => Task::none(),
                    Some(Outcome::Dismissed) => {
                        self.dialog = None;
                        Task::none()
                    }
                    Some(Outcome::Confirmed(action)) => {
                        self.dialog = None;
                        self.confirmed(action)
                    }
                    Some(Outcome::Saved(form)) => {
                        self.dialog = None;
                        self.save_form(form)
                    }
                }
            }

            Message::Sidebar(route) => {
                self.history.clear();
                self.route = route;
                self.notice = None;
                Task::none()
            }
            Message::DismissNotice => {
                self.notice = None;
                Task::none()
            }
            Message::Saved(Ok(())) => Task::none(),
            Message::Saved(Err(error)) => {
                self.dialog = Some(Dialog::info("Could not save", error));
                Task::none()
            }
        }
    }

    fn on_event(&mut self, event: Option<Event>) -> Task<Message> {
        match event {
            Some(event) => self.handle(event),
            None => Task::none(),
        }
    }

    /// The one place where page intents become real changes.
    fn handle(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Navigate(route) => {
                self.navigate(route);
                Task::none()
            }
            Event::Back => {
                self.back();
                Task::none()
            }
            Event::LoggedIn(session) => {
                self.contacts = store::load(&session.username);
                self.session = Some(session);
                self.route = Route::Dashboard;
                self.history.clear();
                self.login = login::Login::default();
                Task::none()
            }
            Event::NewContact => {
                self.dialog = Some(Dialog::ContactForm(ContactForm::new()));
                Task::none()
            }
            Event::EditContact(id) => {
                if let Some(contact) = self.contacts.iter().find(|c| c.id == id) {
                    self.dialog = Some(Dialog::ContactForm(ContactForm::edit(contact)));
                }
                Task::none()
            }
            Event::DeleteContact(id) => {
                if let Some(contact) = self.contacts.iter().find(|c| c.id == id) {
                    self.dialog = Some(Dialog::confirm(
                        "Delete contact?",
                        format!("{} will be permanently removed.", contact.name),
                        "Delete",
                        ConfirmAction::DeleteContact(id),
                    ));
                }
                Task::none()
            }
            Event::ToggleFavorite(id) => {
                if let Some(contact) = self.contacts.iter_mut().find(|c| c.id == id) {
                    contact.favorite = !contact.favorite;
                }
                self.persist()
            }
            Event::ShowAbout => {
                self.dialog = Some(Dialog::info(
                    "About Contactly",
                    "A demo contact book built with Iced 0.14 to learn routing, \
                     state management and dialogs.",
                ));
                Task::none()
            }
            Event::RequestLogout => {
                self.dialog = Some(Dialog::confirm(
                    "Log out?",
                    "You will need to sign in again to see your contacts.",
                    "Log out",
                    ConfirmAction::Logout,
                ));
                Task::none()
            }
            Event::RequestClearAll => {
                self.dialog = Some(Dialog::confirm(
                    "Delete all contacts?",
                    "This removes every contact in your book and cannot be undone.",
                    "Delete all",
                    ConfirmAction::ClearAll,
                ));
                Task::none()
            }
        }
    }

    // ---------------------------------------------------------- navigation

    /// "Push" navigation: remember where we were so Back can return.
    fn navigate(&mut self, route: Route) {
        if self.route == route {
            return;
        }
        let previous = std::mem::replace(&mut self.route, route);
        self.history.push(previous);
        self.notice = None;
    }

    fn back(&mut self) {
        self.route = self.history.pop().unwrap_or(Route::Contacts);
        self.notice = None;
    }

    // -------------------------------------------------------- data changes

    fn confirmed(&mut self, action: ConfirmAction) -> Task<Message> {
        match action {
            ConfirmAction::DeleteContact(id) => {
                self.contacts.retain(|c| c.id != id);
                // Deleting from the detail page: that page no longer makes
                // sense, so leave it.
                if self.route == Route::ContactDetail(id) {
                    self.back();
                }
                self.notice = Some("Contact deleted".to_string());
                self.persist()
            }
            ConfirmAction::ClearAll => {
                self.contacts.clear();
                if matches!(self.route, Route::ContactDetail(_)) {
                    self.back();
                }
                self.notice = Some("All contacts deleted".to_string());
                self.persist()
            }
            ConfirmAction::Logout => {
                self.session = None;
                self.contacts.clear();
                self.history.clear();
                self.route = Route::Dashboard;
                self.contacts_page = contacts::Contacts::default();
                self.notice = None;
                Task::none()
            }
        }
    }

    fn save_form(&mut self, form: ContactForm) -> Task<Message> {
        let name = form.name.trim().to_string();
        let email = form.email.trim().to_string();
        let phone = form.phone.trim().to_string();

        let existing = form
            .id
            .and_then(|id| self.contacts.iter().position(|c| c.id == id));

        let message = match existing {
            Some(index) => {
                let c = &mut self.contacts[index];
                c.name = name.clone();
                c.email = email;
                c.phone = phone;
                c.group = form.group;
                format!("Updated {name}")
            }
            None => {
                let id = self.contacts.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                self.contacts.push(Contact {
                    id,
                    name: name.clone(),
                    email,
                    phone,
                    group: form.group,
                    favorite: false,
                });
                format!("Added {name}")
            }
        };

        self.notice = Some(message);
        self.persist()
    }

    /// Saves in the background; the result comes back as `Message::Saved`.
    fn persist(&self) -> Task<Message> {
        let Some(session) = &self.session else {
            return Task::none();
        };
        let user = session.username.clone();
        let data = self.contacts.clone();
        Task::perform(async move { store::save(&user, &data) }, Message::Saved)
    }

    // =====================================================================
    // View
    // =====================================================================

    fn view(&self) -> Element<'_, Message> {
        let base: Element<'_, Message> = match &self.session {
            // Auth guard: no session, no access to anything else.
            None => self.login.view().map(Message::Login),
            Some(session) => self.shell(session),
        };

        match &self.dialog {
            Some(dialog) => ui::modal(
                base,
                dialog.view().map(Message::Dialog),
                dialog
                    .dismissable_by_backdrop()
                    .then_some(Message::Dialog(dialogs::Message::Close)),
            ),
            None => base,
        }
    }

    /// Sidebar + the page selected by the current route.
    fn shell<'a>(&'a self, session: &'a Session) -> Element<'a, Message> {
        let sidebar = ui::sidebar(&self.route, &session.display_name, Message::Sidebar);

        // The "router": a match on the route.
        let page: Element<'a, Message> = match self.route {
            Route::Dashboard => dashboard::view(session, &self.contacts).map(Message::Dashboard),
            Route::Contacts => self
                .contacts_page
                .view(&self.contacts)
                .map(Message::Contacts),
            Route::ContactDetail(id) => detail::view(&self.contacts, id).map(Message::Detail),
            Route::Settings => {
                settings::view(session, self.contacts.len()).map(Message::Settings)
            }
        };

        let mut content = Column::new()
            .spacing(16.0)
            .width(Length::Fill)
            .height(Length::Fill);
        if let Some(notice) = &self.notice {
            content = content.push(ui::notice(notice, Message::DismissNotice));
        }
        content = content.push(page);

        row![
            sidebar,
            container(content)
                .padding(28.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::page),
        ]
        .into()
    }
}

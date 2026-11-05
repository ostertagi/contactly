//! Every page follows the same pattern (the "Elm architecture", per page):
//!
//!   * a `Message` enum  : what can happen inside this page
//!   * `update(..)`      : handles a `Message`, may return an `Event`
//!   * `view(..)`        : draws the page from state + shared data
//!
//! A page cannot change app-wide state or navigate by itself. It *returns an
//! `Event`* and the parent (`App`) decides what to do. This is how data and
//! intents travel between pages:
//!
//!   page::Message --> page::update --> Event --> App::handle --> shared state

pub mod contacts;
pub mod dashboard;
pub mod detail;
pub mod login;
pub mod settings;

use crate::model::Session;
use crate::route::Route;

#[derive(Debug, Clone)]
pub enum Event {
    Navigate(Route),
    Back,
    LoggedIn(Session),
    NewContact,
    EditContact(u64),
    DeleteContact(u64),
    ToggleFavorite(u64),
    ShowAbout,
    RequestLogout,
    RequestClearAll,
}

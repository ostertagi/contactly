//! Iced has no router. A "route" is just an enum you match on in `view()`.
//! Variants can carry parameters (`ContactDetail(id)`), like `/contacts/:id`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Dashboard,
    Contacts,
    ContactDetail(u64),
    Settings,
}

impl Route {
    /// Which sidebar entry should be highlighted for this route.
    pub fn section(&self) -> Route {
        match self {
            Route::ContactDetail(_) => Route::Contacts,
            other => *other,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Route::Dashboard => "Dashboard",
            Route::Contacts => "Contacts",
            Route::ContactDetail(_) => "Contact",
            Route::Settings => "Settings",
        }
    }
}

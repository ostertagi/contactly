# Contactly: a contact book to learn Iced 0.14

## Run

```bash
cargo run            # needs Rust stable (1.85+ recommended)
cargo test           # runs the search/filter unit tests
```

Demo logins: `demo / demo` or `admin / admin123`.
Data is saved per user in `./data/contacts_<user>.json`.

## What the app shows

| Requirement | Where |
|---|---|
| Authentication | `pages/login.rs`, `auth.rs`, auth guard in `App::view` |
| Several pages + navigation | `route.rs`, `App::shell`, `App::navigate/back`, sidebar in `ui.rs` |
| Info / alert / form dialogs | `dialogs.rs` (opened from Settings and the Contacts page) |
| CRUD | `App::save_form` (create/update), `confirmed(DeleteContact)`, `ToggleFavorite` |
| Search + filter | `pages/contacts.rs` (text query, group chips, favorites) and `Contact::matches` |
| Revault-like theme | `theme.rs` (dark sidebar, light content, white cards, green/red accents) |

## Architecture in one picture

```
                      +-------------------- App (state) --------------------+
                      | session, contacts (source of truth), route, history |
                      | login page, contacts page, dialog, notice           |
                      +-----------------------------------------------------+
   view():  borrows &contacts  --->  page::view()  --->  Element<page::Message>
                                                              | .map(Message::X)
   user clicks                                                v
                                                  App::update(Message::X(msg))
                                                              |
                                                       page::update(msg)
                                                              | returns Option<Event>
                                                              v
                                                       App::handle(event)  -> mutates shared state / navigates / opens dialog
```

### Concepts, mapped to code

1. **State management.** One `App` struct owns everything shared. Pages only
   *borrow* data in `view` (`&'a [Contact]`), so there is a single source of truth.
2. **Routing.** Iced has no router. `Route` is an enum, and `App::shell` matches on it.
   `ContactDetail(u64)` is a route with a parameter.
   - `Message::Sidebar(route)` is a top-level switch and clears history.
   - `Event::Navigate(route)` pushes onto `history`; `Event::Back` pops it.
3. **Auth guard.** `session: Option<Session>`. If `None`, `view` can only render Login.
4. **Data transfer between pages.** A page never mutates app state. It returns an
   `Event` (`pages/mod.rs`); `App::handle` decides. Child messages are wrapped with
   `Element::map(Message::Contacts)` in `view` and unwrapped in `update`.
5. **Dialogs.** `dialog: Option<Dialog>` drawn on top of the page with `stack!` +
   `opaque` (`ui::modal`). The dialog reports an `Outcome`; `App` applies it.
6. **Tasks (async).** Login (`Task::perform(auth::login(..))`) and saving
   (`App::persist`) run in the background and return a `Message` to `update`.
7. **Page state lifetime.** `contacts_page` lives in `App`, so your search survives
   navigating to a detail page and back. Detail/Dashboard/Settings are stateless functions.

## Exercises (in order of difficulty)

1. Make the Contacts page rebuilt on every navigation (replace the field with
   `contacts::Contacts::default()` in `navigate`) and watch the filter reset.
2. Add a `notes: String` field to `Contact` and show it on the detail page.
3. Add a "Sort by" `pick_list` to the contacts page (name, recently added).
4. Close dialogs with Escape: add `.subscription(..)` to `iced::application(..)` using
   `iced::keyboard` events.
5. Add Register + password hashing in `auth.rs`.
6. Add a dark variant of the theme and a toggle in Settings.

## Notes

- The theme was written by hand from the look of revault-gui (dark sidebar, light cards,
  green/red accents), not copied from its source. Tweak the constants at the top of
  `theme.rs` to match it more closely.
- Targets Iced **0.14** (`iced::application(boot, update, view)` + builder methods).
  If you later upgrade, start with `main()` and `theme.rs`, where the API changes most.

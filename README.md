# HadocRx
Simple prescription writer and patient database software.

## Supported Platforms
 - Linux (work in progress)
 - Windows (planned)
 - Android (planned)

## Project To-Dos
- [ ] common (business logic)
    - [ ] open drugs.db as read-only [https://docs.rs/rusqlite/latest/rusqlite/]
    - [ ] use database migrations for patient database [https://docs.rs/rusqlite_migration/latest/rusqlite_migration/]
- [ ] app/linux
    - [ ] read code from [https://github.com/iman-salmani/iplan] to understand libadwaita-rs
    - [ ] migrate from gtk4-rs to libadwaita-rs
    - [ ] main waindow
        - [ ] menubar
        - [ ] toolbar
            - [ ] new patient -> dialog box for patient creation
            - [ ] load patient -> dialog box for searching
            - [ ] new precription -> new tab in the tabview
            - [ ] load prescription -> dialog box that shows all precriptions of current patient -> on selection load prescription in a new tab
        - [ ] tabs for prescription writing
- [ ] app/windows

## Screenshots
<img width="500" alt="image" src="https://github.com/user-attachments/assets/02edfc3d-1b5b-4e19-88d8-4387c128fa23" />

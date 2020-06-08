// Copyright 2020 Google LLC All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::Duration;

use anyhow::Context;
use dbus::blocking::SyncConnection;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use systray::Application;

const INHIBIT_IDLE: u32 = 8;
const GNOME_SESSION_MANAGER: &'static str = "org.gnome.SessionManager";

fn main() -> Result<(), systray::Error> {
    println!("Starting Inhibit App Indicator");
    let mut tray = Application::new().expect("Can't create systray");

    let inhibit = Inhibit::new();
    println!("On Icon: {}", inhibit.icon_on);
    println!("Off Icon: {}", inhibit.icon_off);
    inhibit.icon_on(&tray)?;

    let inhibit = Arc::new(Mutex::new(inhibit));
    let inhibit_toggle = inhibit.clone();
    tray.add_menu_item("Toggle", move |window| {
        let mut inhibit = inhibit_toggle.lock().unwrap();
        if let Err(err) = inhibit.toggle(&window) {
            println!("Toggle error: {:?}", err);
        }
        Ok::<_, systray::Error>(())
    })?;
    let inhibit_quit = inhibit.clone();
    tray.add_menu_item("Quit", move |window| {
        let mut inhibit = inhibit_quit.lock().unwrap();
        if let Some(_) = inhibit.toggle {
            if let Err(err) = inhibit.toggle(&window) {
                println!("Toggle error: {:?}", err);
            }
        }
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    tray.wait_for_message()?;
    Ok(())
}

struct Inhibit {
    conn: SyncConnection,
    icon_on: String,
    icon_off: String,
    toggle: Option<u32>,
}

impl Inhibit {
    fn new() -> Self {
        let mut icon_root = Inhibit::icon_path();
        let mut icon_off_path = icon_root.clone();
        icon_root.push("inhibit-on.png");
        icon_off_path.push("inhibit-off.png");

        Inhibit {
            conn: SyncConnection::new_session().unwrap(),
            icon_on: icon_root
                .into_os_string()
                .into_string()
                .expect("failed to get icon path"),
            icon_off: icon_off_path
                .into_os_string()
                .into_string()
                .expect("failed to get icon path"),
            toggle: None,
        }
    }

    // icon_path returns the root path for the icons
    fn icon_path() -> PathBuf {
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(dir) => {
                let mut icon_root = PathBuf::from(dir);
                icon_root.push("icons");
                icon_root
            }
            Err(_) => PathBuf::from("/usr/share/icons/"),
        }
    }

    fn toggle(&mut self, tray: &Application) -> anyhow::Result<()> {
        let proxy = self.conn.with_proxy(
            GNOME_SESSION_MANAGER,
            "/org/gnome/SessionManager",
            Duration::from_secs(5),
        );

        match self.toggle {
            None => {
                println!("Enabling Inhibit");
                let args: (&str, u32, &str, u32) = (
                    "com.compoundtheory.inhibit",
                    0,
                    "no more screensaver",
                    INHIBIT_IDLE,
                );
                let (cookie,): (u32,) = proxy
                    .method_call(GNOME_SESSION_MANAGER, "Inhibit", args)
                    .with_context(|| "could not Inhibit")?;
                self.toggle = Some(cookie);
                println!("Inhibiting, with cookie: {}", cookie);
                self.icon_off(tray)
                    .with_context(|| "could not enable off icon")
            }
            Some(cookie) => {
                println!("Uninhibiting, with cookie: {}", cookie);
                proxy
                    .method_call(GNOME_SESSION_MANAGER, "Uninhibit", (cookie,))
                    .with_context(|| "could not Uninhibit")?;
                self.toggle = None;
                self.icon_on(tray)
                    .with_context(|| "could not enable on icon")
            }
        }
    }
    fn icon_on(&self, tray: &Application) -> Result<(), systray::Error> {
        tray.set_icon_from_file(&self.icon_on)
    }
    fn icon_off(&self, tray: &Application) -> Result<(), systray::Error> {
        tray.set_icon_from_file(&self.icon_off)
    }
}

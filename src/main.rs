/*
 * Copyright 2020 Google LLC All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env::current_dir;
use systray::Application;

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let mut icon_root = current_dir().unwrap();
    icon_root.push("icons");
    let mut icon_off_path = icon_root.clone();
    icon_root.push("baseline_screen_share_white_18dp.png");
    icon_off_path.push("baseline_stop_screen_share_white_18dp.png");

    let mut inhibit = Inhibit {
        icon_on: icon_root.into_os_string().into_string().unwrap(),
        icon_off: icon_off_path.into_os_string().into_string().unwrap(),
        toggle: None,
    };

    println!("On Icon: {}", inhibit.icon_on);
    println!("Off Icon: {}", inhibit.icon_off);
    inhibit.icon_on(&app)?;

    app.add_menu_item("Toggle", move |window| inhibit.toggle(&window))?;
    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

struct Inhibit {
    icon_on: String,
    icon_off: String,
    toggle: Option<()>,
}

impl Inhibit {
    fn toggle(&mut self, app: &Application) -> Result<(), systray::Error> {
        match self.toggle {
            None => {
                self.toggle = Some(());
                self.icon_off(app)
            }
            Some(_) => {
                self.toggle = None;
                self.icon_on(app)
            }
        }
    }
    fn icon_on(&self, app: &Application) -> Result<(), systray::Error> {
        app.set_icon_from_file(&self.icon_on)
    }
    fn icon_off(&self, app: &Application) -> Result<(), systray::Error> {
        app.set_icon_from_file(&self.icon_off)
    }
}

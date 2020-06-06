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

fn main() -> Result<(), systray::Error> {
    let mut inhibit = Inhibit { toggle: None };
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let mut path = current_dir().unwrap();
    path.push("icons");
    path.push("baseline_screen_share_white_18dp.png");
    let image_path = path.into_os_string().into_string().unwrap();
    println!("Image path: {}", image_path);

    app.set_icon_from_file(image_path.as_str())?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

struct Inhibit {
    toggle: Option<()>,
}

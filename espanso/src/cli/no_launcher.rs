/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{
    cli::{CliModule, CliModuleArgs},
    service::win as service_win,
    path::win as path_win,
};
use anyhow::Result;

pub fn new() -> CliModule {
    CliModule {
        requires_paths: false,
        subcommand: "no-launcher".to_string(),
        entry: no_launcher_main,
        ..Default::default()
    }
}

fn no_launcher_main(_args: CliModuleArgs) -> i32 {
    println!("Setting Espanso defaults (no launcher)...");

    // Step 1: Register as service
    if let Err(e) = service_win::register() {
        eprintln!("Failed to register Espanso service: {e:?}");
        return 1;
    }
    println!("Espanso service registered successfully.");

    // Step 2: Add Espanso to PATH
    if let Err(e) = path_win::add_espanso_to_path(false) {
        eprintln!("Failed to add Espanso to PATH: {e:?}");
        return 1;
    }
    println!("Espanso added to PATH successfully.");

    println!("Espanso defaults setup complete!");
    0
}


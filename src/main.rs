use inquire::Select;
use regex::Regex;
use serde::Serialize;
use std::process::Command;
use std::str;

fn main() {
    #[derive(Serialize, Debug)]
    struct FlatpakApp {
        name: String,
        application_id: String,
        version: String,
        branch: String,
        installation: String,
    }

    let command_output = Command::new("flatpak")
        .arg("list")
        .output()
        .expect("Failed to execute flatpak list");

    let output_str = str::from_utf8(&command_output.stdout).expect("Invalid utf8");

    let mut apps: Vec<FlatpakApp> = Vec::new();

    for line in output_str.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.len() >= 5 {
            let name = fields[0..fields.len() - 4].join("");
            let application_id = fields[fields.len() - 4].to_string();
            let version = fields[fields.len() - 3].to_string();
            let branch = fields[fields.len() - 2].to_string();
            let installation = fields[fields.len() - 1].to_string();

            apps.push(FlatpakApp {
                name,
                application_id,
                version,
                branch,
                installation,
            });
        }
    }

    let app_options: Vec<String> = apps
        .iter()
        .map(|app| format!("{} (ID: {})", app.name, app.application_id))
        .collect();
    let selected_app = Select::new("Select a Flatpak application to run:", app_options)
        .prompt()
        .unwrap();
    let flatpak_regex = Regex::new(r"\b(\w+\.\w+\.\w+\.\w+)\b").unwrap();

    if let Some(captures) = flatpak_regex.captures(&selected_app) {
        if let Some(matched) = captures.get(0) {
            Command::new("flatpak")
                .arg("run")
                .arg(matched.as_str())
                .spawn()
                .expect_err("Error: ");
        } else {
            println!("There was an issue with the app selection");
        }
    }

    // TODO: Launch app selected by the user
    // Command::new("flatpak")
    //     .arg("run")
    //     .arg(application)
    //     .spawn()
    //     .expect("Failed to launch application");

    //TODO: grab wmctrl -l then check the id that has a name similar to the application selected
    //Command::new("wmctrl")
    //    .args(["-r", application, "-b", "add,fullscreen"])
    //.output()
    //.expect("Failed to set the window to fullscreen");
}

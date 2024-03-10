use std::{collections::HashMap, env, process::{Command, exit}};

fn main() {
    let all = vec!["shutdown", "reboot", "logout"];
    let show = all.clone();

    let texts: HashMap<&str, &str> = [("logout", "Logout"), ("reboot", "Reboot"), ("shutdown", "Shut down")].iter().cloned().collect();

    let icons: HashMap<&str, &str> = [("logout", "⏏"), ("reboot", "⭮"), ("shutdown", "⏻"), ("cancel", "x")].iter().cloned().collect();

    let actions: HashMap<&str, &str> = [("logout", "loginctl terminate-session ${XDG_SESSION_ID-}"), ("reboot", "systemctl reboot"), ("shutdown", "systemctl poweroff")].iter().cloned().map(|(k,v)| {
        (k, if k == "logout" { "sh -c 'loginctl terminate-session ${XDG_SESSION_ID-}'" } else { v })
    }).collect();

    let confirmations = vec!["reboot", "shutdown", "logout"];

    let args: Vec<String> = env::args().collect();
    let mut dryrun = false;

    for arg in &args[1..] {
        if arg == "--dry-run" {
            dryrun = true;
        }
    }

    let mut messages: HashMap<&str, String> = HashMap::new();
    let mut confirmation_messages: HashMap<&str, String> = HashMap::new();

    for entry in &all {
        messages.insert(entry, write_message(icons[entry], texts[entry], true, true, ""));
        confirmation_messages.insert(entry, write_message(icons[entry], &format!("Yes, {}", texts[entry]), true, true, ""));
    }
    confirmation_messages.insert("cancel", write_message(icons["cancel"], "No, cancel", true, true, ""));

    let mut selection = args.get(1).cloned().unwrap_or_default();

    if selection.is_empty() {
        println!("\0no-custom\x1ftrue");
        println!("\0markup-rows\x1ftrue");
        println!("\0prompt\x1f⏻");
        for entry in &show {
            println!("{}\0icon\x1f{}", messages[entry], icons[entry]);
        }
    } else {
        for entry in &show {
            if selection == print_selection(&messages[entry]) {
                for confirmation in &confirmations {
                    if entry == confirmation {
                        println!("\0prompt\x1fAre you sure");
                        println!("{}\0icon\x1f{}", confirmation_messages[entry], icons[entry]);
                        println!("{}\0icon\x1f{}", confirmation_messages["cancel"], icons["cancel"]);
                        exit(0);
                    }
                }
                selection = print_selection(&confirmation_messages[entry]);
            }
            if selection == print_selection(&confirmation_messages[entry]) {
                if !dryrun {
                    let _ = Command::new("sh")
                        .arg("-c")
                        .arg(actions[entry])
                        .spawn()
                        .expect("failed to execute process");
                }
                exit(0);
            }
            if selection == print_selection(&confirmation_messages["cancel"]) {
                exit(0);
            }
        }
        eprintln!("Invalid selection: {}", selection);
        exit(1);
    }
}

fn write_message(icon: &str, text: &str, showsymbols: bool, showtext: bool, symbols_font: &str) -> String {
    let icon_str = if showsymbols {
        if showtext {
            format!("‎{} \u{2068}{}", icon, text)
        } else {
            format!("‎{}", icon)
        }
    } else {
        text.to_string()
    };

    if !symbols_font.is_empty() {
        format!("<span font=\"{}\" font_size=\"medium\">{}</span>", symbols_font, icon_str)
    } else {
        format!("<span font_size=\"medium\">{}</span>", icon_str)
    }
}

fn print_selection(message: &str) -> String {
    message.to_string()
}


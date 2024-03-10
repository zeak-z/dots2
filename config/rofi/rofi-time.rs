use std::process::Command;

fn main() {
    // Get the current system time in 12-hour format
    let time = format_system_time();

    // Output the time directly to stdout with HTML tags for styling
    println!(
        "\0no-custom\x1ftrue\n\0markup-rows\x1ftrue\n\0prompt\x1f{}\n\0combos\x1fshift+Return",
        time
    );
}

fn format_system_time() -> String {
    let formatted_time = Command::new("date")
        .args(&["+%I:%M %p"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    String::from_utf8(formatted_time).expect("Invalid UTF-8").trim().to_string()
}


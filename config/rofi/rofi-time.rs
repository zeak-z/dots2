use std::process::Command;

fn main() {
    // Get the current system time and date
    let datetime = format_system_datetime();

    // Output the time and date directly to stdout with HTML tags for styling
    println!(
        "\0no-custom\x1ftrue\n\0markup-rows\x1ftrue\n\0prompt\x1f{}\n\0combos\x1fshift+Return",
        datetime
    );
}

fn format_system_datetime() -> String {
    let formatted_datetime = Command::new("date")
        .args(&["+%I:%M |%p| %m/%d"]) // Changed the format here
        .output()
        .expect("Failed to execute command")
        .stdout;

    String::from_utf8(formatted_datetime).expect("Invalid UTF-8").trim().to_string()
}


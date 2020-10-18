const MAX_BRIGHNTESS_FILE: &'static str = "/sys/class/backlight/intel_backlight/max_brightness";
const BRIGHNTESS_FILE: &'static str = "/sys/class/backlight/intel_backlight/brightness";

fn no_file_msg(file_name: &str) -> String {
    format!("Couldn't read file {}", file_name)
}

fn read_to_int(file_name: &str) -> i32 {
    let value_str = std::fs::read_to_string(file_name).expect(&no_file_msg(file_name));
    value_str.trim().parse().expect("Could not parse value")
}

fn brightness_diff(max_brightness: i32, mode: &str) -> i32 {
    let value = max_brightness / 100;
    match mode {
        "inc" => value,
        "dec" => -value,
        _ => 0
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode: &str = &args[1];

    let max_brightness = read_to_int(MAX_BRIGHNTESS_FILE);
    let brightness = read_to_int(BRIGHNTESS_FILE);

    let diff = brightness_diff(max_brightness, mode);
    let new_brightness = (brightness + diff).max(0).min(max_brightness);

    std::fs::write(BRIGHNTESS_FILE, &format!("{}", new_brightness))
        .expect("Could not write new brightness");
}

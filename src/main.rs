use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target: &str = &args[1];

    let mode: &str = &args[2];

    let (path, max_brightness) = match target{
        "screen" => ("/sys/class/backlight/intel_backlight/brightness", 937),
        "kbd" => ("/sys/class/leds/asus::kbd_backlight/brightness", 3),
        _ => ("/tmp/derp", 0),
    };

    //let max_brightness = 937;

    let mut brightness = File::open(path)
        .expect("Could not open derp file");

    let mut content =  String::new();

    brightness.read_to_string(&mut content).expect("This is weird");
    println!("Content {}", content);

    let mut val: i32 =  match mode{
        "inc" => 1,
        "dec" => -1,
        _ => 0,
    };

    let mut min_val = 0;

    if target == "screen" {
        val *= 50;
        min_val += 5;
    }

    let new_brightness = (val + content.trim().parse::<i32>().expect("Wosh")).max(min_val).min(max_brightness);
    println!("New: {}", new_brightness);

    let mut write_brightness = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Could not open brightness file");
    write_brightness.write(&format!("{}", new_brightness).into_bytes()).expect("Unable to write");
}

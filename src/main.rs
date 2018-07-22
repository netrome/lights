use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let max_brightness = 937;

    let mut brightness = File::open("/sys/class/backlight/intel_backlight/brightness")
        .expect("Could not open derp file");

    let mut content =  String::new();

    brightness.read_to_string(&mut content).expect("This is weird");
    println!("Content {}", content);

    let slice: &str = &args[1];

    let val: i32 =  match slice{
        "inc" => 50,
        "dec" => -50,
        _ => 0,
    };

    let new_brightness = (val + content.trim().parse::<i32>().expect("Wosh")).max(5).min(max_brightness);
    println!("New: {}", new_brightness);

    let mut write_brightness = OpenOptions::new()
        .write(true)
        .open("/sys/class/backlight/intel_backlight/brightness")
        .expect("Could not open brightness file");
    write_brightness.write(&format!("{}", new_brightness).into_bytes()).expect("Unable to write");
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut device = Device::try_new(cli.device_path)?;

    device.read()?;
    device.apply(&cli.delta);
    device.write()?;

    Ok(())
}

/// Controls backlight strenght
#[derive(clap::Parser, Debug)]
struct Cli {
    /// Path to the device to modify. Example "/sys/class/backlight/intel_backlight/"
    #[arg(short, long)]
    device_path: PathBuf,
    #[command(flatten)]
    delta: Delta,
}

#[derive(clap::Args, Debug)]
struct Delta {
    /// Fraction of max brightness to add to current brightness. Sensible range [-1.0, 1.0].
    #[arg(short, long)]
    abs: f64,

    /// Multiplier of current brightness. Sensible range [0.0, +Inf].
    #[arg(short, long)]
    rel: f64,
}

struct Device {
    path: PathBuf,
    brightness: usize,
    max_brightness: usize,
}

impl Device {
    fn try_new(path: PathBuf) -> anyhow::Result<Self> {
        let max_brightness = read_value(&path.join("max_brightness"))?;
        let brightness = read_value(&path.join("brightness"))?;

        Ok(Self {
            path,
            max_brightness,
            brightness,
        })
    }

    fn read(&mut self) -> anyhow::Result<()> {
        self.max_brightness = read_value(self.max_brightness_path())?;
        self.brightness = read_value(self.brightness_path())?;

        Ok(())
    }

    fn apply(&mut self, delta: &Delta) {
        let abs_step = (self.max_brightness as f64) * delta.abs;

        let mut brightness = (self.brightness as f64) * delta.rel;
        brightness += abs_step;

        if brightness.is_normal() {
            self.brightness = brightness.clamp(10., self.max_brightness as f64).ceil() as usize;
        }
    }

    fn write(&self) -> anyhow::Result<()> {
        write_value(self.brightness_path(), self.brightness)?;
        Ok(())
    }

    fn max_brightness_path(&self) -> PathBuf {
        self.path.join("max_brightness")
    }

    fn brightness_path(&self) -> PathBuf {
        self.path.join("brightness")
    }
}

fn read_value<T, P>(file_name: P) -> anyhow::Result<T>
where
    T: FromStr,
    T::Err: Display,
    P: AsRef<Path> + Debug,
{
    let value_str = std::fs::read_to_string(&file_name)?;
    value_str
        .trim()
        .parse::<T>()
        .map_err(|err| anyhow::anyhow!("Failed to parse {:?}: {}", file_name, err))
}

fn write_value<P, T>(path: P, value: T) -> anyhow::Result<()>
where
    T: Display,
    P: AsRef<Path> + Debug,
{
    Ok(std::fs::write(path, format!("{}", value))?)
}

use clap::Parser;

use std::fmt::Debug;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

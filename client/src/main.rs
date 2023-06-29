use std::{time::Duration, thread::sleep, env, process};

use windows::Win32::Graphics::Gdi::{GetPixel, GetDC, GetDeviceCaps, HDC, HORZRES, VERTRES};
use rgb::{RGB, RGBA};
use serialport::{self, SerialPortInfo, SerialPort};

fn main() {
    let ports: Vec<SerialPortInfo> = serialport::available_ports().expect("Failed to find available serial ports");

    print!("Available serial ports: ");
    for p in ports {
        print!("{}", p.port_name);
    }
    println!("\n");

    let port: &String;
    let rate: u32;
    let vertical_leds: i32;
    let horizontal_leds: i32;

    let args: Vec<_> = env::args().collect();
    if args.len() > 4 {
        port = &args[1];
        rate = args[2].parse().unwrap();
        vertical_leds = args[3].parse().unwrap();
        horizontal_leds = args[4].parse().unwrap();
    } else {
        println!("Usage: `{} <serial port (ex. COM1)> <baud rate (ex. 9600)> <vertical leds count> <horizontal leds count>`", args[0]);
        process::exit(0);
    }

    let hdc: HDC = unsafe { GetDC(None) };

    let screen_width: i32 = unsafe { GetDeviceCaps(hdc, HORZRES) };
    let screen_height: i32 = unsafe { GetDeviceCaps(hdc, VERTRES) };

    let vertical_step: i32 = screen_height / vertical_leds;
    let horizontal_step: i32 = screen_width / horizontal_leds;

    println!("Screen width: {}px", screen_width);
    println!("Screen height: {}px\n", screen_height);

    let mut port: Box<dyn SerialPort> = serialport::new(port, rate)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    loop {
        // TODO: improve this maybe
        let mut payload: String = String::new();

        for (i, y) in (1..screen_height).step_by(vertical_step as usize).enumerate() {
            let color: RGB<u32> = get_color(hdc, 1, y);

            payload.push_str(format!("{},{},{},{}|", i + 1, color.r, color.g, color.b).as_str());
        }

        payload = payload.trim_end_matches("|").to_string();
        payload.push_str("~");

        for (i, x) in (1..screen_width).step_by(horizontal_step as usize).enumerate() {
            let color: RGB<u32> = get_color(hdc, x, 1);

            payload.push_str(format!("{},{},{},{}|", i + 1, color.r, color.g, color.b).as_str());
        }

        payload = payload.trim_end_matches("|").to_string();
        
        port.write(payload.as_bytes()).expect("Failed to write to serial port");
        println!("payload sended");

        sleep(Duration::from_millis(500));
    }
}

fn get_color(hdc: HDC, x: i32, y: i32) -> RGB<u32> {
    let color: u32 = unsafe{ GetPixel(hdc, x, y) }.0;

    // convert COLORREFs to ints
    let r: u32 = color & 0xff;
    let g: u32 = color >> 8 & 0xff;
    let b: u32 = color >> 16 & 0xff;
    let a: u32 = color >> 24 & 0xff;

    RGBA::new(r, g, b, a).rgb()
}
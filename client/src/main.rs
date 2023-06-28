use std::{time::Duration, thread::sleep, io};

use windows::Win32::Graphics::Gdi::{GetPixel, GetDC, GetDeviceCaps, HDC, HORZRES, VERTRES};
use rgb::{RGB, RGBA};
use serialport::{self, SerialPortInfo, SerialPort};

fn main() {

    let hdc: HDC = unsafe { GetDC(None) };

    let screen_width: i32 = unsafe { GetDeviceCaps(hdc, HORZRES) };
    let screen_height: i32 = unsafe { GetDeviceCaps(hdc, VERTRES) };

    println!("Screen width: {}px", screen_width);
    println!("Screen height: {}px\n", screen_height);

    let ports: Vec<SerialPortInfo> = serialport::available_ports().expect("Failed to find available serial ports");

    print!("Available ports: ");
    for p in ports {
        print!("{}", p.port_name);
    }
    println!("\n");

    let mut port: String = String::new();
    println!("Give me the port where Arduino is connected: ");
    io::stdin().read_line(&mut port).unwrap();

    let mut rate: String = String::new();
    println!("Give me the baud rate of this port: ");
    io::stdin().read_line(&mut rate).unwrap();

    let mut port: Box<dyn SerialPort> = serialport::new(port.trim(), rate.trim().parse().unwrap())
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    println!("Here we go!");

    loop {
        let color: u32 = unsafe{ GetPixel(hdc, 100, 100) }.0;

        // convert COLORREFs to ints
        let r: u32 = color & 0xff;
        let g: u32 = color >> 8 & 0xff;
        let b: u32 = color >> 16 & 0xff;
        let a: u32 = color >> 24 & 0xff;
    
        let rgb: RGB<u32> = RGBA::new(r, g, b, a).rgb();
    
        port.write(format!("{}|{}|{}", rgb.r, rgb.g, rgb.b).as_bytes()).expect("Failed to write to serial port");

        println!("Color r: {} g: {} b: {} sended to serial port", rgb.r, rgb.g, rgb.b);
        sleep(Duration::from_millis(500));
    }
}
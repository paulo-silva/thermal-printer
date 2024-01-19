extern crate image;

use escpos_rs::{EscposImage, Justification, Printer, PrinterProfile};
use std::env;

// 48 chars per line

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: &String = &args[1];

    let printer_profile = PrinterProfile::usb_builder(0x1fc9, 0x2016).build();
    let printer = match Printer::new(printer_profile) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :("),
        },
        Err(e) => panic!("Error: {}", e),
    };

    maybe_print_logo(&printer, &args);

    match printer.println(text.replace("\\n", "\n")) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn maybe_print_logo(printer: &Printer, args: &Vec<String>) {
    if args.len() > 2 {
        let logo: &String = &args[2];
        let img = image::open(logo).unwrap();
        let escpos_image = EscposImage::new(img, 128, Justification::Center).unwrap();
        match printer.image(escpos_image) {
            Ok(_) => (), // Image should be printed
            Err(e) => println!("Error: {}", e),
        };
    }
}

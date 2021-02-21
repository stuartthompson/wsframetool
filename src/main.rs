extern crate banner;
extern crate base64;
extern crate sha1;

mod decoder;

use banner::{Banner, Color, HeaderLevel, Style};
use decoder::decode_frame;

pub enum Mode {
    Decode,
    Encode,
}

fn main() {
    print_title_banner();

    // Verify startup arguments
    if std::env::args().len() != 3 {
        println!("Usage: wsft [-e -d] content.");
        return;
    }

    // Parse command-line arguments
    let mode_flag: String = std::env::args().nth(1).expect("Error reading argument 1.");
    let content: String = std::env::args().nth(2).expect("Error reading argument 2.");

    if mode_flag == String::from("-D") || mode_flag == String::from("-d") {
        decode_frame(content);
    } else if mode_flag == String::from("-E") || mode_flag == String::from("-e") {
        encode_frame(content);
    } else {
        println!("Mode must be -D/d or -E/e");
    }
}

fn encode_frame(content: String) {}

fn print_title_banner() {
    // Create a style
    let mut style: Style = Style::new();
    style.border.color = Color::Red;
    style.h1.content_color = Color::Yellow;
    style.text.content_color = Color::White;
    // Create header banner
    let mut banner = Banner::new(&style);
    // TODO: Remove when default width is fixed
    banner.width = 0;

    // Add headers
    banner.add_header("WDI - WebSocket Data Inspector", HeaderLevel::H1);
    banner.add_text("Visualize WebSocket data frames.");

    // Print banner
    println!("{}", banner.assemble());
}

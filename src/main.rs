#[allow(dead_code)]
mod colors {
    pub static RED: &str = "\x1b[31m";
    pub static BRIGHT_RED: &str = "\x1b[91m";
    pub static GREEN: &str = "\x1b[32m";
    pub static BLUE: &str = "\x1b[34m";
    pub static BRIGHT_BLUE: &str = "\x1b[96m";
    pub static YELLOW: &str = "\x1b[33m";
    pub static GREY: &str = "\x1b[90m";
    pub static CLEAR: &str = "\x1b[0m";
    pub static BOLD: &str = "\x1b[1m";
    pub static UNDERLINE: &str = "\x1b[4m";
    pub static UNDERLINE_OFF: &str = "\x1b[24m";
}

#[allow(dead_code)]
mod terminal {
    extern "C" { 
        fn kb() -> u8; 
        fn hide_cursor_and_text();
        fn show_cursor_and_text();
    }

    pub fn getch() -> u8 {
        let ch: u8;

        unsafe { 
            ch = kb(); 
        }

        print!("\x1b[1K\x1b[0E");

        ch
    }

    pub fn hide() {
        unsafe { hide_cursor_and_text(); }
    }

    pub fn show() {
        unsafe { show_cursor_and_text(); }
    }

    pub fn cls() {
        print!("\x1b[2J");
    }

    pub fn clear_lines(lines: i32) {
        for _ in 0..lines{
            print!("\x1b[2K\x1b[1A");
        } 
        print!("\x1b[2K");
    }
}

extern crate home;
#[macro_use] extern crate text_io;
fn main() {
    use std::env;
    use std::fs::{OpenOptions, File};
    use std::io::prelude::*;    

    // init
    let mut running: bool = true;
    
    // load & parse file
    let args: Vec<String> = env::args().collect();

    // print help if not enough args
    if args.len() < 2 {
        println!("{}Use: {}todo <name-of-todo-list>{}", colors::GREY, colors::YELLOW, colors::CLEAR);
        println!("{}{}Keybinds{}: {}", colors::GREY, colors::UNDERLINE, colors::UNDERLINE_OFF, colors::CLEAR);
        println!("{}Arrow up/down   {}move selection up/down{}", colors::BRIGHT_BLUE, colors::BLUE, colors::CLEAR);
        println!("{}'w'/'s'         {}move entry up/down{}", colors::BRIGHT_BLUE, colors::BLUE, colors::CLEAR);
        println!("{}'n'             {}create entry{}", colors::BRIGHT_BLUE, colors::GREEN, colors::CLEAR);
        println!("{}'d'             {}delete entry{}", colors::BRIGHT_BLUE, colors::RED, colors::CLEAR);
        println!("{}'e'             {}mark as important{}", colors::BRIGHT_BLUE, colors::BRIGHT_RED, colors::CLEAR);
        println!("{}'c'             {}mark as completed{}", colors::BRIGHT_BLUE, colors::GREEN, colors::CLEAR);
        println!("{}'Space'         {}mark as comment{}", colors::BRIGHT_BLUE, colors::GREY, colors::CLEAR);
        return;
    }

    
    let home_dir: String = home::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let filename = String::from(&home_dir) + "/.todo/" + &args[1] + ".txt";
    
    std::fs::create_dir_all(String::from(&home_dir) + "/.todo").expect("Can't create .todo dir in home directory!");
    
    match OpenOptions::new().create(true).write(true).open(&filename) {
        Err(e) => println!("{:?}", e),
        Ok(_) => ()
    };

    let mut file = File::open(&filename).expect("File does not exist!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong while reading from the file!");

    let _lines: Vec<&str> = content.split("\n").collect();
    let mut lines: Vec<String> = Vec::new();

    for i in 0.._lines.len() {
        let mut l: String = String::from(_lines[i]);
        if l == "\n" || l == "" {
            continue;
        }

        if !l.starts_with("! ") && !l.starts_with("✓ ") && !l.starts_with("# ") && !l.starts_with("· ") {
            l = String::from("· ") + &l[..];
        }
        lines.push(l);
    }

    let mut selected: usize = 0;

    terminal::hide();

    // main loop
    while running {
        // display
        println!("{}{}{}:{}", colors::BRIGHT_BLUE, colors::BOLD, &args[1], colors::CLEAR);
        for i in 0..lines.len() {
            print!("{} ", if i == selected { ">" } else { " " });
            match lines[i].chars().nth(0).unwrap() {
                '!' => print!("{}", colors::BRIGHT_RED),
                '✓' => print!("{}", colors::GREEN),
                '#' => print!("{}", colors::GREY),
                 _  => print!("{}", colors::CLEAR),
            }
            println!("{}{}", lines[i], colors::CLEAR);
        }

        // get keyboard input
        let ch: u8 = terminal::getch();

        // clear screen
        terminal::clear_lines(lines.len() as i32 + 2);

        // process keyboard input
        match ch {
            // Space was pressed
             32 => if lines.len() > 0 { lines[selected] = if lines[selected].starts_with("# ") {
                    String::from("·") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                } else {
                    String::from("#") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                }},
            // arrow up was pressed
             65 => if lines.len() > 0 && selected > 0 { selected -= 1 },
            // arrow down was pressed
             66 => if lines.len() > 0 && selected < lines.len()-1 { selected += 1 }, 
            // c was pressed
             99 => if lines.len() > 0 { lines[selected] = if lines[selected].starts_with("✓ ") {
                    String::from("·") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                } else {
                    String::from("✓") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                }}, 
            // d was pressed
            100 => if lines.len() > 0 { lines.remove(selected); }, 
            // e was pressed
            101 => if lines.len() > 0 { lines[selected] = if lines[selected].starts_with("! ") {
                    String::from("·") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                } else {
                    String::from("!") + &lines[selected][lines[selected].chars().nth(0).unwrap().len_utf8()..] 
                }}, 
            // n was pressed
            110 => {
                terminal::show();
                println!("{}Input a new entry: {}", colors::BRIGHT_BLUE, colors::CLEAR);
                let s: String = read!("{}\n");
                lines.push(String::from("· ") + &s);
                terminal::clear_lines(2);
                terminal::hide();
            }
            // q was pressed
            113 => running = false, 
            // s was pressed
            115 => if lines.len() > 0 && selected < lines.len()-1 {
                let tmp = String::from(&lines[selected]);
                lines[selected] = String::from(&lines[selected + 1]);
                lines[selected+1] = tmp;
                selected += 1;
            }, 
            // w was pressed
            119 => if lines.len() > 0 && selected > 0 {
                let tmp = String::from(&lines[selected]);
                lines[selected] = String::from(&lines[selected - 1]);
                lines[selected-1] = tmp;
                selected -= 1;
            }, 
             _  => (),
        }

    } // end of main loop

    terminal::show();

    // if there are changes, write them to the file
    let mut file = File::create(&filename).expect("Create failed");
    for i in 0..lines.len() {
        if i < lines.len() - 1 {
            file.write_all((String::from(&lines[i]) + "\n").as_bytes()).expect("Write failed!");
        } else {
            file.write_all((String::from(&lines[i])).as_bytes()).expect("Write failed!");
        }
    }
}

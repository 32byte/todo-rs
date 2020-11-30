pub mod colors {
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

pub mod terminal {
    // from the kb.c file
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

        // clear the line, and move the cursor back
        print!("\x1b[1K\x1b[0E");

        ch
    }

    pub fn hide() {
        unsafe {
            hide_cursor_and_text();
        }
    }

    pub fn show() {
        unsafe {
            show_cursor_and_text();
        }
    }

    pub fn clear_lines(lines: i32) {
        for _ in 0..lines {
            print!("\x1b[2K\x1b[1A");
        }
        print!("\x1b[2K");
    }
}

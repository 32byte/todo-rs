#include <stdlib.h>
#include <stdio.h>

unsigned char kb() {
    unsigned char c;

    // enable raw input
    system ("/bin/stty raw");

    // get char
    c = getchar();
    // is char escape code? (like arrow key)
    
    if (c == '\033') {
        // ignore next char = '['
        getchar();
        // save the next char
        c = getchar();
    }

    // disable raw input
    system ("/bin/stty cooked");

    // return char
    return c;
}

void hide_cursor_and_text() {
    system("tput civis; stty -echo");
}

void show_cursor_and_text() {
    system("tput cnorm; stty echo");
}

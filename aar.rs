// the main loop for AAR

#[feature(globs)];

extern crate ncurses;

use ncurses::*;
use asciisprite::*;

mod asciisprite;

fn main() {
    /* Start ncurses. */
    let win: WINDOW = initscr();

	  loop {
	      //printw("Hello, world!");

        let logo = AsciiSprite::new("graphics/logo.txt");

        logo.drawSprite();

        refresh();


	      std::io::timer::sleep(1000);
    }

    /* Wait for a key press. */
    //getch();

    /* Terminate ncurses. */
    //endwin();
}

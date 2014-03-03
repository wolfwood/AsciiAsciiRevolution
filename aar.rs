// the main loop for AAR

#[feature(globs)];
#[feature(struct_inherit)];

extern crate ncurses;

use ncurses::*;
use asciisprite::*;
use animatedasciisprite::*;

mod asciisprite;
mod animatedasciisprite;

fn main() {
    /* Start ncurses. */
    let win: WINDOW = initscr();


    let logo = AsciiSprite::new("graphics/logo.txt");
    logo.drawSprite();
    refresh();

    let mut crongif = AnimatedAsciiSprite::new("graphics/cron-popcron.txt", true, true);

    //clear();

	  loop {

        crongif.drawSprite();
        refresh();

        crongif.nextFrame();

        std::io::timer::sleep(1000);
    }

    /* Wait for a key press. */
    //getch();

    /* Terminate ncurses. */
    //endwin();
}

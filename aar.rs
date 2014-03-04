// the main loop for AAR

#[feature(globs)];
#[feature(struct_inherit)];

extern crate ncurses;

use ncurses::*;
use asciisprite::*;

mod asciisprite;

fn main() {
    /* Start ncurses. */
    let win: WINDOW = initscr();


    let logo = AsciiSprite::new_static("graphics/logo.txt", false);
    logo.drawSprite();
    refresh();

    let mut crongif = AsciiSprite::new("graphics/cron-popcron.txt", false, true);

    //clear();

	  loop {

        crongif.drawSprite();
        refresh();

        crongif.nextFrame();

        std::io::timer::sleep(500);
    }

    /* Wait for a key press. */
    //getch();

    /* Terminate ncurses. */
    //endwin();
}

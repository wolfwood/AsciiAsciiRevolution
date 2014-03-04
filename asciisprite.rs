use ncurses::*;
use std::io::File;
use std::io::BufferedReader;

pub struct AsciiSprite {
    x: uint,
    y: uint,
    sprites: ~[~[~str]],
    frame: uint,
    transparent: bool,
    looping: bool,
    animate: bool,
}

impl AsciiSprite {
    pub fn new(filepath: &str, transparent: bool, looping: bool) -> AsciiSprite {
        let mut file = BufferedReader::new(File::open(&Path::new(filepath)));

        let mut file = BufferedReader::new(File::open(&Path::new(filepath)));

        let mut firstLine = true;
        let mut sprites: ~[~[~str]] = ~[];

        for line in file.lines() {
            if line.contains_char('%') {
                // % is the frame break character
                firstLine = true;
            } else {
                // XXX: trim newlines instead of filtering in draw

                if firstLine {
                    // start a new frame
                    sprites.push(~[line]);

                    firstLine = false;
                } else {
                    // append to frame
                    sprites[sprites.len() -1].push(line);
                }
            }
        }

        AsciiSprite {x: 0, y: 0, sprites: sprites, frame: 0, transparent: transparent, looping:looping, animate: true}
    }

    pub fn new_static(filepath: &str, transparent: bool) -> AsciiSprite{
        AsciiSprite::new(filepath, transparent, false)
    }

    pub fn setXY(&mut self, x: uint, y: uint) {
        self.x = x;
        self.y = y;
    }

    pub fn setY(&mut self, y: uint) {
        self.y = y;
    }

    pub fn drawSprite(&self) {
        let mut y = self.y;

        for line in self.sprites[self.frame].iter() {
            let mut x = self.x;

            for chr in line.chars() {

                // transparent sprites don't draw spaces
                if chr != '\n' && ((chr != ' ' && chr != '$') || !self.transparent) {
                    mvaddch(y as i32, x as i32, chr as u32);
                }

                // $ is the special char for when a transparent sprite needs to blot out what's underneath
                if chr == '$' {
                    mvaddch(y as i32, x as i32, ' ' as u32);
                }

                x+=1;
            }
            y+=1;
        }
    }

    // increments counter, wrapping if looping sprite otherwise disabling further updates
    pub fn nextFrame(&mut self) {
        if self.animate {
            self.frame += 1;

            if !self.looping && self.frame == self.sprites.len() {
                self.animate = false;
            }else{
                self.frame %= self.sprites.len();

            }
        }
    }
}

use ncurses::*;
use std::io::File;
use std::io::BufferedReader;

pub struct AsciiSprite {
    x: uint,
    y: uint,
    sprite: ~[~str],
    transparent: bool,
}

impl AsciiSprite {
    pub fn new(filepath: &str) -> AsciiSprite {
        let mut file = BufferedReader::new(File::open(&Path::new(filepath)));

        AsciiSprite {x: 0, y: 0, sprite: file.lines().collect(), transparent: false}
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

        for line in self.sprite.iter() {
            let mut x = self.x;

            for chr in line.chars() {

                if (chr != ' ' && chr != '$') || !self.transparent {
                    mvaddch(y as i32, x as i32, chr as u32);
                }

                if chr == '$' {
                    mvaddch(y as i32, x as i32, ' ' as u32);
                }

                x+=1;
            }
            y+=1;
        }
    }
}

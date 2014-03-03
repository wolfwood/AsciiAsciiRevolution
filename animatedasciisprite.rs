
use ncurses::*;
use std::io::File;
use std::io::BufferedReader;
use std::vec::*;
use std::rc::*;
use asciisprite::*;


pub struct AnimatedAsciiSprite : AsciiSprite {
    sprites: ~[~[~str]],
    frame: uint,
    looping: bool,
    animate: bool,
}

impl AnimatedAsciiSprite {
    pub fn new(filepath: &str, looping: bool, animate: bool) -> AnimatedAsciiSprite {
        let mut file = BufferedReader::new(File::open(&Path::new(filepath)));

        let mut firstLine = true;
        let mut sprites: ~[~[~str]] = ~[~[]];

        for line in file.lines() {
            if line.contains_char('%') {
                firstLine = true;
            } else {
                if firstLine {
                    sprites.push(~[line]);

                    firstLine = false;
                } else {
                    sprites[sprites.len() -1].push(line);
                }
            }
        }

        // XXX: don't use clone
        AnimatedAsciiSprite {x: 0, y: 0, sprite: sprites[0].clone(), transparent: false, sprites: sprites, frame: 0, looping: looping, animate: animate}
    }

    pub fn nextFrame(&mut self) {
        if self.animate {
            self.frame += 1;

            if !self.looping && self.frame == self.sprites.len() {
                self.animate = false;
            }else{
                self.frame %= self.sprites.len();

                // XXX: don't use clone
                self.sprite = self.sprites[self.frame].clone();
            }
        }
    }

    pub fn drawSprite(&self) {
        let mut y = self.y;

        for line in self.sprites[self.frame].iter() {
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

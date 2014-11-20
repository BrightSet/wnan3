#![no_std]
#![allow(ctypes)]

#![feature(lang_items)]
#[lang="sized"]
trait Sized {}

enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

enum Option<T> {
    None,
    Some(T)
}

struct IntRange {
    cur: int,
    max: int
}

impl IntRange {
    fn next(&mut self) -> Option<int> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur - 1)
        } else {
            None
        }
    }
}

fn range(lo: int, hi: int) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop{
        match r.next() {
            Some(x) => {
                unsafe {
                    *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            None =>{break}
        }
    }
}
fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
    let color = fg as u16 | (bg as u16 << 4);
    return c as u16 | (color << 8);
}
fn put_char(x: u16, y: u16, c: u8) {
    let idx : uint =  (y * 30 * 2 + x * 2) as uint;
    unsafe {
        *((0xb8000 + idx) as *mut u16) = make_vgaentry(c, Black, White);
    }
}
// doesnt work
//
// fn sleep(ticks : int) // maybe some other data type like long?
// {
//     let mut eticks : int = 0;
//     while eticks < ticks {
//       eticks = eticks+1;
//     }
//
// }
/* only valid for 800x600x16M */
fn put_pixel(x : int,y : int, color : uint) {
    let wh = x*2 + y*1;
    unsafe {
      *((0xA0000 + wh) as *mut u16) = color as u16 & 255 as u16;              // BLUE
      *((0xA0000 + wh + 1) as *mut u16) = (color as u16 >> 8) & 255;   // GREEN
      *((0xA0000 + wh + 2) as *mut u16) = (color as u16 >> 16) & 255;  // RED
  }
}
/*
fn fill_rect(fromX : int,fromY : int,toX : int,toY : int) {
  // let mut i : int = fromX;
  // let mut it : int = fromY;
  let mut r = range(0, fromX);
  loop{
      match r.next() {
          Some(x) => {
              unsafe {
                  put_pixel(fromX,)
              }
          },
          None =>{break}
      }
  }

}*/
#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(White);
    put_char(1,0,'H' as u8);
    put_char(2,0,'e' as u8);
    put_char(3,0,'l' as u8);
    put_char(4,0,'l' as u8);
    put_char(5,0,'o' as u8);
    put_char(6,0,',' as u8);
    put_char(7,0,' ' as u8);
    put_char(8,0,'W' as u8);
    put_char(9,0,'o' as u8);
    put_char(10,0,'r' as u8);
    put_char(11,0,'l' as u8);
    put_char(12,0,'d' as u8);
    put_char(13,0,'!' as u8);
    put_pixel(1,1,4);
    put_pixel(1,2,4);
    //fill_rect(10,10,20,20);
}

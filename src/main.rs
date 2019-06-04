#![deny(clippy::all)]
#![deny(clippy::pedantic)]

extern crate pancurses;
use pancurses as curses;

fn main() {
    let window = curses::initscr();
    curses::noecho();

    main_loop(&window);

    curses::endwin();
}

#[derive(Debug,Default)]
struct CharCounts {
    lower_alpha_counts: [i32; 26],
    space_count: i32,
    period_count: i32,
    comma_count: i32,
    other_count: i32,
}

impl CharCounts {
    fn add_char(&mut self, ch: char) {
        let lower_ch = ch.to_ascii_lowercase();
        match lower_ch {
            'a' ... 'z' => {
                self.lower_alpha_counts[lower_ch as usize - 'a' as usize] += 1
            }
            ' ' => self.space_count += 1,
            '.' => self.period_count += 1,
            ',' => self.comma_count += 1,
            _ => self.other_count += 1,
        }
    }
}

/// Writes the entire state of the window out to the screen.
fn render(wnd: &curses::Window, text: &str, counts: &CharCounts)
{
    unimplemented!()
}

fn main_loop(window: &curses::Window) {
    let mut char_counts = CharCounts::default();
    let mut text = String::new();

    loop {
        render(&window, &text, &char_counts);

        match window.getch() {
            Some(curses::Input::Character(input)) => {
                text.push(input);
                char_counts.add_char(input);
            },
            _ => break
        };
    }
}

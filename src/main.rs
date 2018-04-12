extern crate pancurses;
use pancurses as curses;

fn main() {
    let window = curses::initscr();
    window.printw("Done initializing");
    curses::noecho();

    main_loop(&window);

    curses::endwin();
    println!("All done!")
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


fn main_loop(window: &curses::Window) {
    let mut counts: CharCounts = Default::default();

    println!("(can i even do this?)");
    loop {
        let ch: char = match window.getch() {
            Some(curses::Input::Character(input)) => {
                counts.add_char(input);
                input
            },
            _ => break
        };

        window.printw(&ch.to_string());
    }
}

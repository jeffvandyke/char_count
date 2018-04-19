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
fn render(wnd: &curses::Window, text: &String, counts: &CharCounts)
{
    // Shoot, I don't know how to use curses best!
    let screen_width = wnd.get_max_x();

    // first, print out 1st 5 lines of text
    const MAX_ROWS: i32 = 5;
    let str_n_rows
        = ((text.len() as f32) / (screen_width as f32)).ceil() as i32;

    // just try a little functional weirdness and hope I get this math right
    let text_row_width = screen_width - 2;
    let text_print_rows = (std::cmp::max(str_n_rows - MAX_ROWS, 0)..str_n_rows)
        .map(|row| text.get(
                (row * text_row_width) as usize
                ..std::cmp::min(
                    (row + 1) * text_row_width,
                    text.len() as i32) as usize));

    wnd.mv(0, 0);
    for print_row_n in text_print_rows {
        wnd.printw("| ");
        wnd.printw(print_row_n.unwrap_or("HELP! BAD INDEX!!!"));
    }
}

fn main_loop(window: &curses::Window) {
    let mut counts: CharCounts = Default::default();
    let mut text = String::new();

    loop {
        render(&window, &text, &counts);

        match window.getch() {
            Some(curses::Input::Character(input)) => {
                text.push(input);
                counts.add_char(input);
            },
            _ => break
        };
    }
}

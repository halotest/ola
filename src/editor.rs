use crate::Document;
use crate::Row;
use crate::{terminal::CursorState, Terminal};
use termion::event::Key;

#[derive(PartialEq, Eq)]
pub enum EditorMode {
    Visual,
    Insert,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    pub mode: EditorMode,
    document: Document,
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
            cursor_position: Position::default(),
            mode: EditorMode::Visual,
            document: Document::open(),
        }
    }

    fn switch_mode(&self, key: Key) {
        let next_mode = if Key::Esc == key {
            EditorMode::Visual
        } else {
            EditorMode::Insert
        };

        match next_mode {
            EditorMode::Insert => Terminal::set_cursor_state(CursorState::Bar),
            EditorMode::Visual => Terminal::set_cursor_state(CursorState::Block),
        }
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;

        let is_visual = EditorMode::Visual == self.mode;

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('i') | Key::Esc => self.switch_mode(pressed_key),
            Key::Char('k') => {
                if is_visual {
                    self.move_cursor(pressed_key)
                } else {
                }
            }
            Key::Char('j') => {
                if is_visual {
                    self.move_cursor(pressed_key)
                } else {
                }
            }
            Key::Char('h') => {
                if is_visual {
                    self.move_cursor(pressed_key)
                } else {
                }
            }
            Key::Char('l') => {
                if is_visual {
                    self.move_cursor(pressed_key)
                } else {
                }
            }
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J"); https://vt100.net/docs/vt100-ug/chapter3.html#ED | Erase in display \x1b-ESC [2J - clear all
        // Terminal::clear_screen(); // same as above
        Terminal::set_cursor_state(CursorState::Hidden);
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen(); // same as above
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::set_cursor_state(CursorState::Visible);

        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!(
            "Welcome editor -- version {}{}",
            VERSION,
            self.mode == EditorMode::Visual
        );
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if terminal_row / 3 == height {
                self.draw_welcome_message();
            } else {
                println!(
                    "{}   {}~\r",
                    terminal_row,
                    termion::color::Fg(termion::color::Red)
                );
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        let is_visual = self.mode == EditorMode::Visual;
        println!("{}", is_visual);

        match key {
            Key::Char('k') => {
                if is_visual {
                    y = y.saturating_sub(1)
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Char('j') => {
                if is_visual {
                    if y < height {
                        y = y.saturating_add(1)
                    }
                }
            }
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Char('h') => {
                if x < width {
                    x = x.saturating_sub(1)
                }
            }
            Key::Left => {
                if x < width {
                    x = x.saturating_sub(1)
                }
            }
            Key::Char('l') => x = x.saturating_add(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),
        }

        self.cursor_position = Position { x, y };
    }
}

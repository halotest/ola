use std::io::{self, stdout, Write};

use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

pub enum CursorState {
    Hidden,
    Visible,
    Block,
    Bar,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub fn set_cursor_state(state: CursorState) {
        match state {
            CursorState::Hidden => print!("{}", termion::cursor::Hide),
            CursorState::Visible => print!("{}", termion::cursor::Show),
            CursorState::Block => print!("{}", termion::cursor::SteadyBlock),
            CursorState::Bar => print!("{}", termion::cursor::SteadyBar),
        }
    }

    pub fn clear_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}

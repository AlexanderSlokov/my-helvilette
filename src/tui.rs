//! `meow`: Helvilette's room, full screen. Keys and the mouse go to the cat;
//! the bottom rows are the food tray and the status line.

use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::{cursor, execute, queue, terminal};

use crate::art::{self, BLANK, Cell, Layout};
use crate::cat::{Cat, FOODS};
use crate::state;

const MIN_COLS: u16 = 30;
/// No meal for 8 hours, or never: Laguz, the sad True Self, comes.
const HUNGRY: &str = "đói meo";
const MIN_ROWS: u16 = 16;
const WORDS_AT: art::P = (1575.0, 585.0);
const WORDS_RGB: [u8; 3] = [255, 140, 220];
const TEXT_RGB: [u8; 3] = [150, 200, 230];

pub fn run() -> io::Result<()> {
    let mut out = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(
        out,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        event::EnableMouseCapture,
        event::EnableFocusChange
    )?;
    // A panic must not leave the terminal in raw mode.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        default_hook(info);
    }));
    let result = Room::new().and_then(|mut room| room.run(&mut out));
    restore();
    result
}

fn restore() {
    let _ = execute!(
        io::stdout(),
        event::DisableFocusChange,
        event::DisableMouseCapture,
        cursor::Show,
        terminal::LeaveAlternateScreen
    );
    let _ = terminal::disable_raw_mode();
}

struct Room {
    cat: Cat,
    cols: u16,
    rows: u16,
    shown: Vec<Cell>,
    state: Option<PathBuf>,
    fed_at: Option<u64>,
    error: Option<String>,
}

impl Room {
    fn new() -> io::Result<Self> {
        let (cols, rows) = terminal::size()?;
        let state = state::path();
        let fed_at = state.as_deref().and_then(state::fed_at);
        Ok(Room {
            cat: Cat::new(state::now() as u32),
            cols,
            rows,
            shown: Vec::new(),
            state,
            fed_at,
            error: None,
        })
    }

    fn layout(&self) -> Layout {
        Layout::new(self.cols, self.rows.saturating_sub(2))
    }

    fn run(&mut self, out: &mut impl Write) -> io::Result<()> {
        let mut last = Instant::now();
        loop {
            if event::poll(Duration::from_secs_f32(self.cat.frame_secs()))? {
                // Take all waiting events, so a burst of mouse moves does not lag.
                loop {
                    if !self.handle(event::read()?) {
                        return Ok(());
                    }
                    if !event::poll(Duration::ZERO)? {
                        break;
                    }
                }
            }
            let now = Instant::now();
            self.cat.set_hungry(self.hunger() == HUNGRY);
            self.cat
                .update(now.duration_since(last).as_secs_f32().min(0.5));
            last = now;
            self.draw(out)?;
        }
    }

    /// Returns false when the user closes the room.
    fn handle(&mut self, ev: Event) -> bool {
        match ev {
            Event::Key(k) if k.kind != KeyEventKind::Release => match k.code {
                KeyCode::Esc => return false,
                KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => return false,
                KeyCode::F(n @ 1..=5) => self.feed(n as usize - 1),
                _ => self.cat.key(),
            },
            Event::Mouse(m) => {
                let layout = self.layout();
                match m.kind {
                    MouseEventKind::Moved | MouseEventKind::Drag(_) => {
                        self.cat.pointer(layout.to_image(m.column, m.row));
                    }
                    MouseEventKind::Down(MouseButton::Left) if m.row == layout.rows => {
                        if let Some(&(_, _, food)) = tray(self.cols)
                            .iter()
                            .find(|(a, b, _)| (*a..*b).contains(&m.column))
                        {
                            self.feed(food);
                        }
                    }
                    _ => {}
                }
            }
            Event::FocusGained => self.cat.focus(true),
            Event::FocusLost => self.cat.focus(false),
            Event::Resize(c, r) => {
                (self.cols, self.rows) = (c, r);
                self.shown.clear();
            }
            _ => {}
        }
        true
    }

    fn feed(&mut self, food: usize) {
        self.cat.feed(food);
        let now = state::now();
        self.fed_at = Some(now);
        self.error = match &self.state {
            Some(path) => state::save_fed_at(path, now)
                .err()
                .map(|e| format!("không lưu được bữa ăn: {e}")),
            None => Some("không lưu được bữa ăn: không có $HOME".into()),
        };
    }

    fn hunger(&self) -> &'static str {
        match self.fed_at.map(|t| state::now().saturating_sub(t) / 3600) {
            Some(h) if h < 4 => "no bụng",
            Some(h) if h < 8 => "hơi đói",
            _ => HUNGRY,
        }
    }

    fn frame(&self) -> Vec<Cell> {
        let (cols, rows) = (self.cols as usize, self.rows as usize);
        let mut frame = vec![BLANK; cols * rows];
        if self.cols < MIN_COLS || self.rows < MIN_ROWS {
            put(
                &mut frame,
                cols,
                0,
                0,
                "Terminal nhỏ quá, Helvilette không vừa.",
                TEXT_RGB,
            );
            return frame;
        }
        let layout = self.layout();
        let art = art::render(&layout, &self.cat.pose());
        frame[..art.len()].copy_from_slice(&art);
        if let Some(words) = self.cat.words() {
            let (c, r) = layout.to_cell(WORDS_AT);
            let c = c.min(cols as i32 - words.chars().count() as i32).max(0);
            put(
                &mut frame,
                cols,
                c as usize,
                r.max(0) as usize,
                &words,
                WORDS_RGB,
            );
        }
        for (start, _, food) in tray(self.cols) {
            let label = format!("F{} {}", food + 1, FOODS[food].0);
            put(&mut frame, cols, start as usize, rows - 2, &label, TEXT_RGB);
        }
        let status = match &self.error {
            Some(e) => format!(" Helvilette · {e}"),
            None => format!(" Helvilette · {} · Esc: thoát", self.hunger()),
        };
        put(&mut frame, cols, 0, rows - 1, &status, [110, 130, 160]);
        frame
    }

    /// Writes only the cells that changed since the last frame.
    fn draw(&mut self, out: &mut impl Write) -> io::Result<()> {
        let frame = self.frame();
        let full = self.shown.len() != frame.len();
        if full {
            queue!(out, terminal::Clear(terminal::ClearType::All))?;
        }
        let cols = self.cols as usize;
        let mut color = None;
        let mut next = None;
        for (i, cell) in frame.iter().enumerate() {
            if !full && self.shown[i] == *cell || full && *cell == BLANK {
                continue;
            }
            if next != Some(i) {
                queue!(out, cursor::MoveTo((i % cols) as u16, (i / cols) as u16))?;
            }
            if color != Some(cell.rgb) {
                let [r, g, b] = cell.rgb;
                queue!(out, SetForegroundColor(Color::Rgb { r, g, b }))?;
                color = Some(cell.rgb);
            }
            queue!(out, Print(cell.ch))?;
            next = Some(i + 1);
        }
        out.flush()?;
        self.shown = frame;
        Ok(())
    }
}

/// Columns of each food label on the tray row: start, end, food.
fn tray(cols: u16) -> Vec<(u16, u16, usize)> {
    let mut x = 1u16;
    let mut items = Vec::new();
    for (i, (name, _)) in FOODS.iter().enumerate() {
        let w = format!("F{} {}", i + 1, name).chars().count() as u16;
        if x + w > cols {
            break;
        }
        items.push((x, x + w, i));
        x += w + 3;
    }
    items
}

fn put(frame: &mut [Cell], cols: usize, col: usize, row: usize, text: &str, rgb: [u8; 3]) {
    for (i, ch) in text.chars().enumerate() {
        if col + i < cols && row * cols + col + i < frame.len() {
            frame[row * cols + col + i] = Cell { ch, rgb };
        }
    }
}

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::data_base::Entry;

pub fn display_setup(entry: &Entry) -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new(entry).run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Clone, Copy, Debug)]
struct App<'a> {
    entry: &'a Entry,
    scroll_state: u16,
    exit: bool,
}

impl<'a> App<'a> {
    fn new(entry: &'a Entry) -> Self {
        Self {
            entry,
            scroll_state: 0,
            exit: false,
        }
    }
    fn run(&'a mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Mouse(_mouse_event) => todo!(),
            Event::Resize(_, _) => todo!(),
            _ => todo!(),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            event::KeyCode::Char('j') | event::KeyCode::Down => {
                self.scroll_state = self.scroll_state.saturating_add(1)
            }
            event::KeyCode::Char('k') | event::KeyCode::Up => {
                self.scroll_state = self.scroll_state.saturating_sub(1)
            }
            event::KeyCode::Char('q') | event::KeyCode::Char('Q') => self.exit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw(self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

// TODO: Implement stronger md rendering for entry
impl Widget for App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let heading_style = Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED);
        let body_style = Style::new();
        let mut text = Text::default();

        text.push_line(Line::styled("TLDR", heading_style));
        self.entry
            .tldr
            .lines()
            .for_each(|l| text.push_line(Line::styled(l, body_style)));
        text.push_line("");
        text.push_line(Line::styled("Info", heading_style));
        self.entry
            .info
            .lines()
            .for_each(|l| text.push_line(Line::styled(l, body_style)));
        text.push_line("");
        text.push_line(Line::styled("Common Uses", heading_style));
        self.entry
            .common_uses
            .lines()
            .for_each(|l| text.push_line(Line::styled(l, body_style)));
        text.push_line("");
        text.push_line(Line::styled("Resources", heading_style));
        self.entry
            .resources
            .lines()
            .for_each(|l| text.push_line(Line::styled(l, body_style)));
        text.push_line("");
        text.push_line(Line::styled(
            self.entry.extra.lines().take(1).collect::<String>(),
            heading_style,
        ));
        // extra prints out the # at the moment (BAD)
        self.entry
            .extra
            .lines()
            .skip(1)
            .for_each(|l| text.push_line(Line::styled(l, body_style)));

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .title(&self.entry.name[..])
                    .title_bottom("<j|↑> scroll up | <k|↓> scroll down | <q|Q> quit"),
            )
            .scroll((self.scroll_state, 1))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

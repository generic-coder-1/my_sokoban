use ratatui::{buffer::Buffer, crossterm::event::KeyCode, layout::Rect, text::{Line, Text}, widgets::{Block, Padding, Paragraph, Widget, WidgetRef}};

use crate::menu::Menu;

#[derive(Default, Clone, Copy)]
pub enum QuitMenu {
    #[default] Undecided,
    Yes,
    No,
}

impl Menu for QuitMenu {
    type Output = ();

    fn handle_input(&mut self, input: ratatui::crossterm::event::KeyEvent) {
        match input.code{
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'y') => *self = QuitMenu::Yes,
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'n') => *self = QuitMenu::No,
            _ => {}
        }
    }

    fn is_done(&mut self) -> Option<crate::menu::MenuOptions<Self::Output>> {
        match self{
            QuitMenu::Undecided => None,
            QuitMenu::Yes => Some(crate::menu::MenuOptions::Exit(())),
            QuitMenu::No => Some(crate::menu::MenuOptions::GoBack),
        }
    }
}

impl WidgetRef for QuitMenu {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Text::from(vec![
            Line::from("Are you sure you want to quit?"),
            Line::from(""),
            Line::from("(Y)es   (N)o"),
        ])
        .centered();
        let block = Block::bordered()
            .title_alignment(ratatui::layout::Alignment::Center)
            .title_top("Tank Game")
            .padding(Padding::new(
                0,                                        // left
                0,                                        // right
                (area.height - text.height() as u16) / 2, // top
                0,                                        // bottom
            ))
            .border_type(ratatui::widgets::BorderType::Rounded);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

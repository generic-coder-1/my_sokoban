use std::{ffi::OsString, fs};

use crate::{
    board::Board, edit_menu::EditMenu, menu::{Menu, MenuOptions}
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget, WidgetRef},
};
use ron::ser::PrettyConfig;

pub struct MainMenu {
    selected_tool: Option<MainMenuState>,
}

#[derive(Debug)]
enum MainMenuState {
    Play,
    Edit,
    Create,
    Quit,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            selected_tool: None,
        }
    }
}

impl Menu for MainMenu {
    type Output = ();

    fn handle_input(&mut self, input: ratatui::crossterm::event::KeyEvent) {
        match input.code {
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'p') => {
                self.selected_tool = Some(MainMenuState::Play)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'e') => {
                self.selected_tool = Some(MainMenuState::Edit)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'c') => {
                self.selected_tool = Some(MainMenuState::Create)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'q') => {
                self.selected_tool = Some(MainMenuState::Quit)
            }
            _ => {}
        }
    }

    fn is_done(&mut self) -> std::option::Option<MenuOptions<()>> {
        self.selected_tool.take().map(|tool| match tool {
            MainMenuState::Play => todo!(),
            MainMenuState::Edit => todo!(),
            MainMenuState::Create => {
                //let board = Board::new(21, 21);
                //let mut name = 1;
                //let project_dirs = directories::ProjectDirs::from("", "", "Generic's Sokoban")
                //    .expect("cannot find home directory");
                //let path = project_dirs.data_dir();
                //if fs::read_dir(path).is_err() {
                //    fs::create_dir_all(path).expect("cannot create game directory");
                //}
                //let files: Vec<OsString> = fs::read_dir(path)
                //    .expect("cannot access game directory")
                //    .filter_map(|file| file.ok().map(|file| file.file_name()))
                //    .collect();
                //while files.iter().any(|file_name| {
                //    *file_name == Into::<OsString>::into(format!("new_map_{}.ron", name))
                //}) {
                //    name += 1;
                //}
                //let mut path_buf = path.to_path_buf();
                //path_buf.push(format!("new_map_{}.ron", name));
                //fs::write(
                //    &path_buf,
                //    ron::ser::to_string_pretty(&board, PrettyConfig::new())
                //        .expect("couldn't serialize default map"),
                //)
                //.expect("couldn't create new map");
                //
                //MenuOptions::Continue(Box::new(EditMenu::from_path(path_buf.as_path()).expect("couldn't access new map")))
                MenuOptions::Continue(Box::new(EditMenu::default()))
            }
            MainMenuState::Quit => MenuOptions::Exit(()),
        })
    }
}

impl WidgetRef for MainMenu {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Text::from(vec![
            Line::from("(P)lay"),
            Line::from(""),
            Line::from("(E)dit"),
            Line::from(""),
            Line::from("(C)reate"),
            Line::from(""),
            Line::from("(Q)uit"),
            Line::from(format!("{:?}", self.selected_tool)),
        ])
        .centered();
        let block = Block::bordered()
            .title_alignment(ratatui::layout::Alignment::Center)
            .title_top("Generic's Sokoban")
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

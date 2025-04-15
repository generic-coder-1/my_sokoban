use ratatui::{crossterm::event::KeyEvent, widgets::WidgetRef};

pub trait Menu: WidgetRef{
    type Output;

    fn handle_input(&mut self, input: KeyEvent);
    fn is_done(&mut self) -> Option<MenuOptions<Self::Output>>;
}

pub enum MenuOptions<T>{
    GoBack,
    BackToFirst,
    Continue(Box<dyn Menu<Output = T>>),
    Exit(T),
}


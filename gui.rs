use iced::{
    Application, Command, Element, executor, widget::Text
};

pub struct GUI;

impl Application for GUI {
    type Executor = executor::Default; // Исполнитель по умолчанию
    type Message = (); // Тип сообщений, используемых в приложении
    type Flags = (); // Флаги для инициализации приложения
    type Theme = iced::theme::Theme; // Тема приложения

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        "ACM GUI".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Hello, ACM!").into()
    }
}

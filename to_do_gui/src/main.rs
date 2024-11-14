#[allow(unused_imports)]
use rfd;
#[allow(unused_imports)]
use iced::{
    Font, Application, executor, Command, widget::{
        button::Button, Row, tooltip, horizontal_space, button::Appearance, button, column, container, row, text, text_input, Column, Container, Text, TextInput
    }, Alignment, theme, Theme, Color, Vector, Shadow, border::Radius, Border, Background, Element, Length, Sandbox, Settings
};
use iced::widget::button::StyleSheet;
#[allow(unused_imports)]
use std::{fs, io, sync::Arc, path::{Path, PathBuf}};

fn main() -> iced::Result {
    ToDoApp::run(Settings {
        default_font: Font::MONOSPACE,
        fonts: vec![include_bytes!("../font/editor-icons.ttf").as_slice().into()],
        ..Settings::default()
    })
}

enum ButtonStyle {
    Clicked,
    Unclicked,
}

impl StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _theme: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(match self {
                Self::Clicked => Color::BLACK,
                Self::Unclicked => Color::WHITE,
            })),
            text_color: Color::BLACK,
            border: Border {
                color: Color::BLACK,
                width: 2.0,
                radius: Radius::default(),
            },
            shadow: Shadow::default(),
        }
    }
}

#[derive(Default)]
struct ToDoApp {
    text: String,
    button: Vec<String>,
    path: Option<PathBuf>,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    TodoChanged(String, usize),
    Submit,
    Remove(usize),
    Mark(usize),
    New,
    Save,
    Import,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FileSaved(Result<PathBuf, Error>),
}

impl Application for ToDoApp {
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self,Command<Message>) {
           ( Self {
                           text: String::from(""),
                           button: Vec::new(),
                           path: None,
                           error: None,
                       },
                Command::none(),
                       )

    }

    fn title(&self) -> String {
        String::from("Text Input Example")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(l) => {
                self.text = l;
                Command::none()
            }
            Message::TodoChanged(l, index) => {
                if self.button[index].chars().last().unwrap_or('n') == '�' {
                    self.button[index] = l+"�"; 
                } else {
                self.button[index] = l; }
                Command::none()
            }
            Message::Submit => {
                let placeholder = self.text.clone();
                self.button.push(placeholder);
                self.text = String::from("");
                Command::none()
            }
            Message::Remove(index) => {
                self.button.remove(index);
                Command::none()
            }
            Message::Mark(index) => {
                self.button[index] = self.button[index].clone() + "�";
                Command::none()
            }
            Message::FileOpened(Ok((path, _content))) => {
                self.path = Some(path);
                Command::none()
            }
            Message::New => {
                self.path = None;
                self.button = Vec::new();
                Command::none()
            }
            Message::Import => {
                // Handle file import synchronously
                if let Ok((path, content)) = pick_file() {
                    self.path = Some(path);
                    if !content.is_empty() { 
                        self.button = content.split('☔').map(|s| s.to_string()).collect();
                    }
                } else {
                    self.error = Some(Error::DialogClosed);
                }
                Command::none()
            }
            Message::Save => {
                // Handle file save synchronously
                let text = self.button.join("☔");
                if let Ok(path) = save_file(self.path.clone(), text) {
                    self.path = Some(path);
                } else {
                    self.error = Some(Error::IOFailed(io::ErrorKind::Other));
                }
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                println!("Error opening file");
                self.error = Some(error);
                Command::none()
            }
            Message::FileSaved(Ok(path)) => {
                println!("File saved");
                self.path = Some(path);
                Command::none()
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input = TextInput::new(
            "This is the placeholder...",
            &self.text,
        )
        .padding(10)
        .on_input(Message::TextInputChanged)
        .on_submit(Message::Submit);

        let mut col = Column::new().push(row![
            action(new_icon(), " New ", Message::New),
            action(open_icon(), " Open ", Message::Import),
            action(save_icon(), " Save ", Message::Save)
        ].spacing(10));
         let status = if let Some(Error::IOFailed(error)) = self.error.as_ref() {
            text(error.to_string())
        } else {
            match self.path.as_deref().and_then(Path::to_str) {
                Some(path) => text(path).size(14),
                None => text("New File"),
            }
        };
        col = col.push(row![status,horizontal_space()]);
        col = col.push(input);

        for (index, item) in self.button.iter().enumerate() {
    let is_clicked = item.
    chars().
    last()
    .unwrap_or('n') == '�';
    let mut text = item.clone();
    if is_clicked {
        text.pop();
    }
    
    let button_style = if is_clicked {
        ButtonStyle::Clicked
    } else {
        ButtonStyle::Unclicked
    };
    
    let button = Button::new(Text::new("    ").size(20))
        .style(iced::theme::Button::Custom(Box::new(button_style)))
        .on_press(if is_clicked { Message::Remove(index) } else { Message::Mark(index) });

    let row = Row::new()
        .push(button)
        .push(TextInput::new("placeholder", &text)
            .on_input(move |value| Message::TodoChanged(value, index)))
        .spacing(20);

    col = col.push(row);
}
        col.into()
    }
}

fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::FileDialog::new().set_title("choose a text file").pick_file().ok_or(Error::DialogClosed)?;
    println!("File selected");
    load_file(handle.as_path().to_owned())
}

fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = fs::read_to_string(&path)
        .map(Arc::new)
        .map_err(|error| Error::IOFailed(error.kind()))?;
    Ok((path, contents))
}

fn new_icon<'a>() -> Element<'a, Message> {
    icons('\u{e800}')
}

fn save_icon<'a>() -> Element<'a, Message> {
    icons('\u{e801}')
}

fn open_icon<'a>() -> Element<'a, Message> {
    icons('\u{e802}')
}

fn icons<'a>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONTS: Font = Font::with_name("editor-icons");
    text(codepoint).font(ICON_FONTS).into()
}

fn action<'a>(content: Element<'a, Message>, label: &'a str, on_press: Message) -> Element<'a, Message> {
    tooltip(button(container(content).width(30).center_x())
        .on_press(on_press)
        .padding([5, 10]), label, tooltip::Position::FollowCursor)
        .style(theme::Container::Box)
        .into()
}

fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        let handle = rfd::FileDialog::new().set_title("Choose a file name...").save_file().ok_or(Error::DialogClosed)?;
        handle.as_path().to_owned()
    };
    fs::write(&path, text).map_err(|error| Error::IOFailed(error.kind()))?;
    Ok(path)
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind),
}

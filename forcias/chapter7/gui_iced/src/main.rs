use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Text,
};

struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

// const FONT: Font = Font::External {
//     name: "PixelMplus10-Regular",
//     bytes: include_bytes!("../../../assets/fonts/PixelMplus10-Regular.ttf"),
// };

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Forcias")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        // init widgets
        let tick_text = Text::new("00:00:00")
            // .font(FONT)
            .size(50);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                // .font(FONT),
        ).min_width(80);
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                // .font(FONT),
        ).min_width(80);

        // prepare layout
        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

fn main() ->iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings)
}
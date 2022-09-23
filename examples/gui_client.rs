use iced::widget::{button, Button, Column, Row, Text};
use iced::{Alignment, Element, Sandbox, Settings};
use smart_socket_tcp::response::Response;
use smart_socket_tcp::{command::Command, socket_client::SocketClient};

struct ViewModel {
    working: Option<bool>,
    power: Option<f32>,
    button_on: button::State,
    button_off: button::State,
    server: SocketClient,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SwitchToOn,
    SwitchToOff,
}

impl ViewModel {
    fn calculate_power(&mut self) -> Option<f32> {
        match self.server.run_command(Command::GetPower) {
            Ok(response) => match response {
                Response::Power(value) => Some(value),
                _ => None,
            },
            _ => None,
        }
    }

    fn calculate_working(&mut self) -> Option<bool> {
        match self.server.run_command(Command::IsEnabled) {
            Ok(response) => match response {
                Response::Enabled => Some(true),
                Response::Disabled => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Sandbox for ViewModel {
    type Message = Message;

    fn new() -> Self {
        let client = SocketClient::new("127.0.0.1:7890").unwrap();

        let mut res = Self {
            working: None,
            power: None,
            button_on: Default::default(),
            button_off: Default::default(),
            server: client,
        };
        res.working = res.calculate_working();
        res.power = res.calculate_power();
        res
    }

    fn title(&self) -> String {
        String::from("Smart socket - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchToOff => {
                self.server.run_command(Command::TurnOff).unwrap();
            }
            Message::SwitchToOn => {
                self.server.run_command(Command::TurnOn).unwrap();
            }
        }
        self.power = self.calculate_power();
        self.working = self.calculate_working();
    }

    fn view(&mut self) -> Element<Message> {
        let status_str = match self.working {
            Some(working) => {
                let status = match working {
                    true => "is working ",
                    _ => "isn't working ",
                };
                let power = match self.power {
                    Some(value) => format!("and power is {}", value),
                    _ => "".to_owned(),
                };
                let res = format!("{}{}", status, power.as_str());
                res
            }
            _ => "have status as undefined".to_string(),
        };

        let text = Text::new(format!("Socket {}", status_str)).size(24);
        let button_on =
            Button::new(&mut self.button_on, Text::new("to On")).on_press(Message::SwitchToOn);
        let button_off =
            Button::new(&mut self.button_off, Text::new("to Off")).on_press(Message::SwitchToOff);
        let row = Row::new()
            .padding(20)
            .align_items(Alignment::Fill)
            .push(button_on)
            .push(button_off);
        Column::new().padding(20).push(text).push(row).into()
    }
}

pub fn main() -> iced::Result {
    ViewModel::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            ..Default::default()
        },
        ..Default::default()
    })
}

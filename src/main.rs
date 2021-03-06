extern crate iced;
extern crate rodio;

mod player;
mod style;

use iced::{Column, Container, Element, Length, Sandbox, Settings};
use player::*;

pub fn main() {
    Miso::run(Settings {
        window: iced::window::Settings {
            resizable: true,
            decorations: true,
            size: (400, 600),
        },
        ..Settings::default()
    });
}

#[derive(Debug, Clone)]
enum Message {
    PlayerMessage(usize, PlayerMessage),
}

struct Miso {
    players: Vec<Player>,
}

impl Sandbox for Miso {
    type Message = Message;

    fn new() -> Miso {
        Miso {
            players: vec![
                Player::new("resources/birds.wav".to_string(), "Birds".to_string()),
                Player::new("resources/beach.wav".to_string(), "Beach".to_string()),
                Player::new(
                    "resources/forest_wind.wav".to_string(),
                    "Forest Wind".to_string(),
                ),
                Player::new("resources/fan.wav".to_string(), "Fan".to_string()),
            ],
        }
    }

    fn title(&self) -> String {
        String::from("Miso")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayerMessage(i, message) => {
                if let Some(player) = self.players.get_mut(i) {
                    player.update(message);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let players =
            self.players
                .iter_mut()
                .enumerate()
                .fold(Column::new(), |col, (i, player)| {
                    col.push(
                        player
                            .view()
                            .map(move |message| Message::PlayerMessage(i, message)),
                    )
                });

        let content = Column::new()
            .spacing(10)
            .padding(20)
            .max_width(600)
            .push(players);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}

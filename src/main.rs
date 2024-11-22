use items::Bow;
use player::Player;

pub mod items;
pub mod player;

fn main() {
    let player = Player {
        gold: 0,
        position: (50.0, 50.0),
        health: 100,
    };

    let bow = Bow {
        damage: 12,
        range: 18.3,
    };
}

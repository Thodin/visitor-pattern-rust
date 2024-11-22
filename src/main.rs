use items::Bow;
use persistence::{DummyPersister, TxtFileSaver};
use player::Player;

pub mod items;
pub mod persistence;
pub mod player;

fn main() -> std::io::Result<()> {
    let player = Player {
        position: (50.0, 50.0),
        health: 100,
    };

    let bow = Bow {
        damage: 12,
        range: 18.3,
    };

    let txt_file_saver = TxtFileSaver::new("../save_files".into())?;

    player.save_with(&txt_file_saver)?;
    bow.save_with(&txt_file_saver)?;

    let dummy_persister = DummyPersister {};

    player.save_with(&dummy_persister)?;
    bow.save_with(&dummy_persister)?;

    Ok(())
}

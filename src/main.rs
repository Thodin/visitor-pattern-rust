use items::Bow;
use persistence::{DummyPersister, Persistable, TxtFileSaver};
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

    let persistables: Vec<&dyn Persistable> = vec![&player, &bow];

    let txt_file_saver = TxtFileSaver::new("../save_files".into())?;
    let dummy_persister = DummyPersister {};

    for persistable in persistables {
        persistable.save_with(&txt_file_saver)?;
        persistable.save_with(&dummy_persister)?;
    }

    Ok(())
}

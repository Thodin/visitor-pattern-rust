use console_printer::ConsolePrinter;
use game_object::GameObject;
use items::Bow;
use player::Player;
use txt_file_saver::TxtFileSaver;

pub mod console_printer;
pub mod game_object;
pub mod items;
pub mod player;
pub mod txt_file_saver;

fn main() -> std::io::Result<()> {
    let player = Player {
        position: (50.0, 50.0),
        health: 100,
    };

    let bow = Bow {
        damage: 12,
        range: 18.3,
    };

    let game_objects: Vec<&dyn GameObject> = vec![&player, &bow];

    let txt_file_saver = TxtFileSaver::new("../save_files".into())?;
    let console_printer = ConsolePrinter {};

    for game_object in game_objects {
        game_object.accept(&txt_file_saver)?;
        game_object.accept(&console_printer)?;
    }

    Ok(())
}

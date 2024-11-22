use crate::{game_object::GameObjectVisitor, items::Bow, player::Player};

pub struct ConsolePrinter {}

impl GameObjectVisitor for ConsolePrinter {
    fn visit_player(&self, _player: &Player) -> std::io::Result<()> {
        println!("This is the player...");
        Ok(())
    }

    fn visit_bow(&self, _bow: &Bow) -> std::io::Result<()> {
        println!("That's a bow!");
        Ok(())
    }
}

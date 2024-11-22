use crate::{items::Bow, player::Player};

pub trait GameObjectVisitor {
    // TODO: you should do some error mapping on the return types -> Watch my custom errors video!
    fn visit_player(&self, player: &Player) -> std::io::Result<()>;
    fn visit_bow(&self, bow: &Bow) -> std::io::Result<()>;
}

pub trait GameObject {
    // accept visitor
    fn accept(&self, visitor: &dyn GameObjectVisitor) -> std::io::Result<()>;
}

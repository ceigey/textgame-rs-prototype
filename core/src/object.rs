use std::cell::RefCell;
use room::Room;

pub struct Object {
    pub name: str,
    pub alias: str,
    pub room: &RefCell<Room>,
}


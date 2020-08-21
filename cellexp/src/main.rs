use std::cell::Cell;
use std::vec::Vec;
use std::fmt::Debug;
use std::clone::Clone;
use std::marker::Copy;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    doors: Vec<String>,
}



impl Room {
    fn new(name: String) -> Room {
        let doors = Vec::new();
        Room { name, doors }
    }

    fn add_door(&self, room: &Room) -> Room {
        let mut new_room = self.clone();
        new_room.doors.push(room.name.clone());
        new_room
    }

    fn add_door_mut(&mut self, room: &Room) -> &mut Room {
        self.doors.push(room.name.clone());
        self
    }
}

fn main() {
    let r1 = Room::new(String::from("r1"));
    let r2 = Room::new(String::from("r2"));
    let r1 = r1
        .add_door(&r2)
        .add_door(&r2);

    let r1m = Rc::new(RefCell::new(r1.clone()));
    let r2m = Rc::new(RefCell::new(r2.clone()));
    r1m.borrow_mut()
        .add_door_mut(&r2m.borrow())
        .add_door_mut(&r2m.borrow());
    r2m.borrow_mut()
        .add_door_mut(&r1m.borrow());

    println!("Immut: {:?}", r1);
    println!("RcRef: {:?}", r1m);
    println!("Hello, world!");
}

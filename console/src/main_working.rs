use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec; 
use std::clone::Clone;

type Rcc<T> = Rc<RefCell<T>>;

fn rcc<T>(val: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(val))
}

struct Object {
    name: String,
    room: String,
}

impl Object {
    fn new(n: String) -> Self {
        Object {
            name: n,
            room: String::from("")
        }
    }

    fn enter_room(&mut self, r: &Room) -> &mut Self {
        self.room = r.name.clone();
        self
    }
}

struct Room {
    name: String,
    objects: Vec<String>,
    doors: Vec<String>,
}

impl Room {
    fn new(n: String) -> Room {
        Room {
            name: n,
            objects: Vec::new(),
            doors: Vec::new(),
        }
    }

    fn add_door(&mut self, r: &Room) -> &mut Self {
        self.doors.push(r.name.clone());
        self
    }
}

struct World {
    objects: Vec<Rcc<Object>>,
    rooms: Vec<Rc<RefCell<Room>>>,
}

impl World {
    fn new() -> World {
        World {
            objects: Vec::new(),
            rooms: Vec::new(),
        }
    }

    fn get_room(&mut self, s: String)
    -> Option<&Rc<RefCell<Room>>> {
        self.rooms.iter()
            .find(|x| x.borrow().name == s)
    }

    fn get_object(&mut self, s: String)
    -> Option<&Rc<RefCell<Object>>> {
        self.objects.iter()
            .find(|x| x.borrow().name == s)
    }
}

fn main() {
    let r1 =Rc::new(
        RefCell::new(
            Room::new(String::from("r1"))));
    let r2 = Rc::new(
        RefCell::new(
            Room::new(String::from("r2"))));
    let r3 = Rc::new(
        RefCell::new(
            Room::new(String::from("r3"))));

    let mut world = World::new();
    world.rooms = vec!(
        r1.clone(), r2.clone(), r3.clone()
    );

    // To solve another day!
    
    r1.borrow_mut().add_door(&r2.borrow()).;
    r1.borrow_mut().add_door(&r3.borrow());
    r2.borrow_mut().add_door(&r1.borrow());
    r2.borrow_mut().add_door(&r3.borrow());
    r3.borrow_mut().add_door(&r1.borrow());

    println!("Hello, world!");
    run_loop();
}

fn run_loop() {
    let mut playing = true;
    let mut echo = false;
    while playing {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("<error: stdin!>");
        if echo {
            println!("  echo: {}", input);
        }
        match input.trim() {
            "echo" => {
                echo = !echo;
                if echo {
                    println!("  (echo on!)");
                } else {
                    println!("  (echo off!)");
                }
            },
            "quit" => {
                playing = false;
                println!("  (quitting...)");
            },
            _ => {
                println!("  (unknown command...)");
            }
        }
    }
}

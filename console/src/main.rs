use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec; 
use std::clone::Clone;

// Data-driven approach

struct Object {
    name: String,
    room: String,
}

struct Room {
    name: String,
    objects: Vec<String>,
    doors: Vec<String>,
}

struct World {
    objects: Vec<Object>,
    rooms: Vec<Room>,
}

impl Object {
    fn new(name: &String) -> Self {
        let room = String::from("");
        Object { name: name.clone(), room }
    }
}

impl Room {
    fn new(name: &String) -> Self {
        let objects = Vec::new();
        let doors = Vec::new();
        Room { name: name.clone(), objects, doors }
    }
}

impl World {
    fn new() -> World {
        let objects = Vec::new();
        let rooms = Vec::new();
        World { objects, rooms }
    }

    fn get_room(&mut self, name: &String) -> Option<&mut Room> {
        self.rooms
            .iter_mut()
            .find(|x| x.name == name.clone())
    }

    fn add_room(&mut self, name: &String) -> &Self {
        match self.get_room(name) {
            None => {
                let room = Room::new(name);
                self.rooms.push(room)
            },
            Some(x) => eprintln!(
                "Already have room {}", x.name)
        }
        self
    }

    fn get_object(&mut self, name: &String) -> Option<&mut Object> {
        self.objects
            .iter_mut()
            .find(|x| x.name == name.clone())
    }

    fn add_object(&mut self, name: &String) -> &Self {
        match self.get_object(name) {
            None => {
                let obj = Object::new(name);
                self.objects.push(obj)
            }
            Some(x) => eprintln!(
                "Already have {}", x.name)
        }
        self
    }

   fn  add_door(&mut self, n1: &String, n2: &String) -> &Self {
        let m1 = self.get_room(n1);
        let m2 = self.get_room(n2);
        match (m1, m2) {
            (Some(r1), Some(r2)) =>
                r1.doors.push(r2.name.clone()),
            (Some(r1), None) =>
                eprintln!("Missing {}", n2),
            (None, Some(r2)) =>
                eprintln!("Missing {}", n1),
            (None, None) =>
                eprintln!("Missing {}, {}",
                    n1,n2)
        }
        self
    }
}

fn main() {
    let mut world = World::new();
    let r1 = String::from("r1");
    let r2 = String::from("r2");
    let r3 = String::from("r3");
    let player = String::from("player");

    world
        .add_room(&r1)
        .add_room(&r2)
        .add_room(&r3)
        .add_door(&r1, &r2)
        .add_door(&r1, &r3)
        .add_door(&r2, &r1)
        .add_door(&r2, &r3)
        .add_door(&r3, &r1)
        .add_object(&player.clone());
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

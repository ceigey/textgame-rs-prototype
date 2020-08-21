use std::collections::HashMap;


struct Entity {
    name: String,
    space: String
}

struct Portal {
    from: String,
    to: String
}

struct Space {
    name: String,
}

struct World {
    entities: HashMap<String, Entity>,
    portals: Vec<Portal>,
    spaces: HashMap<String, Space>
}

impl World {
    fn new() -> World {
        World {
            entities: HashMap::new(),
            portals: Vec::new(),
            spaces: HashMap::new()
        }
    }

    fn add_entity(&mut self, name: &str) -> &mut Self {
        self.entities.insert(
            name.to_string(), 
            Entity {
                name: name.to_string(),
                space: String::from("")
            });
        self
    }

    fn add_portal(&mut self, from: &str, to: &str) -> &mut Self {
        self.portals.push(
            Portal {
                from: from.to_string(),
                to: to.to_string()
            });
        self
    }

    fn add_space(&mut self, name: &str) -> &mut Self {
        self.spaces.insert(
            name.to_string(),
            Space { name: name.to_string() });
        self
    }

    fn teleport_entity_to_space(&mut self, name: &str, to: &str) -> Result<&mut Self, String> {
        let name = name.to_string();
        let to = to.to_string();
        let entity = self.entities
            .get_mut(&name)
            .ok_or(format!("Couldn't find entity ({})", &name))?;
        let space = self.spaces
            .get(&to)
            .ok_or(format!("Couldn't find space ({})", &to))?;
        entity.space = space.name.clone();
        Ok(self)
    }
}

fn game_loop() {
    let mut world = World::new();
    world
        .add_entity("player")
        .add_space("room1");
    
    world.teleport_entity_to_space("player", "room1").unwrap();
    
    println!("Player space is: {}", world.entities.get("player").map(|e| e.space.clone()).unwrap_or(String::from("Nowhere...")));
}


fn main() {
    println!("Hello, world!");
    game_loop();
}


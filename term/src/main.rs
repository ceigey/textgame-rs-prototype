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
            spaces: HashMap::new(),
            // Portals have no hashmap
            // access is almost always
            // done by searching :(
            portals: Vec::new()
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

    // TODO: use some metaprogramming!
    // TODO: I want filters, no?!
    fn find_mut_portal_to(&mut self, to: &str) -> Option<&mut Portal> {
        self.portals
            .iter_mut()
            .find(|p| p.to == to)
    }

    fn find_mut_portal_from(&mut self, from: &str) -> Option<&mut Portal> {
        self.portals
            .iter_mut()
            .find(|p| p.from == from)
    }

    fn find_portal_to(&self, to: &str) -> Option<&Portal> {
        self.portals
            .iter()
            .find(|p| p.to == to)
    }

    fn find_portal_from(&self, from: &str) -> Option<&Portal> {
        self.portals
            .iter()
            .find(|p| p.from == from)
    }
}

fn game_loop() {
    let mut world = World::new();
    world
        .add_entity("P")
        .add_space("R1")
        .add_space("R2")
        .add_space("R3")
        .add_portal("R1", "R2")
        .add_portal("R1", "R3")
        .add_portal("R2", "R3")
        .add_portal("R2", "R1")
        .add_portal("R3", "R1");
    
    world
        .teleport_entity_to_space("P", "R1")
        .unwrap();

    let mut def_p = Portal {
        to: String::from("VOID"),
        from: String::from("VOID")
    };

    let r3_mp = world
        .find_mut_portal_from("R3")
        .unwrap_or(&mut def_p);
    //We have to copy data before
    //passing world around again
    let r3_to = r3_mp.to.clone();
    let r3_from = r3_mp.from.clone();

    let player_space = world.entities
        .get_mut("player")
        .map(|e| e.space.clone())
        .unwrap_or(String::from("Nowhere..."));
    println!(
        "Player space is: {}",
        player_space
    );

    println!(
        "R3 first portal was: {}<-{}",
        r3_to, r3_from
    );
}


fn main() {
    println!("Hello, world!");
    game_loop();
}


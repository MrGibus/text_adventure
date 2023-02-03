struct Room {
    description: String,
    north: Option<usize>,
    south: Option<usize>,
    east: Option<usize>,
    west: Option<usize>,
    action: fn(Player) -> Player,
}

impl Room {
    fn empty_room() -> Self {
        Self {
            description: String::from("The room is void of anything interesting"),
            north: None,
            south: None,
            east: None,
            west: None,
            action: |player| player,
        }
    }

    fn enter_room(&self, player: Player) -> Player {
        println!("{}", self.description);
        (self.action)(player)
    }
}

struct App {
    player: Player,
    current_idx: usize,
    arena: Vec<Room>,
}

#[derive(Clone, Debug)]
struct Player {
    inventory: Vec<Object>,
    health: usize,
}

impl std::default::Default for Player {
    fn default() -> Self {
        Self {
            inventory: vec![],
            health: 100,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Object {
    Potion,
}

impl App {
    fn new(arena: Vec<Room>) -> Self {
        Self {
            current_idx: 0,
            arena,
            player: Player::default(),
        }
    }

    fn describe_current(&self) {
        println!("{}", self.current_room().unwrap().description)
    }

    fn current_room(&self) -> Option<&Room> {
        self.arena.get(self.current_idx)
    }

    fn enter_room(&mut self, idx: usize) {
        self.player = self.arena[idx].enter_room(std::mem::take(&mut self.player));
    }

    fn go_north(&mut self) {
        if let Some(i) = self.current_room().unwrap().north {
            self.current_idx = i;
            self.enter_room(i);
        } else {
            println!("There's no exit in that direction!")
        }
    }
}

fn main() {
    let rooms = vec![
        Room {
            description: String::from("Starting Hallway with exit up ahead"),
            north: Some(1),
            south: None,
            east: None,
            west: None,
            action: |p| p,
        },
        Room {
            description: String::from("another Hallway with exit up ahead. Hey you find a potion!"),
            north: Some(2),
            south: Some(1),
            east: None,
            west: None,
            action: |mut p| {
                p.inventory.push(Object::Potion);
                p
            },
        },
        Room {
            description: String::from("A garden with a door ahead, you cut yourself on a thorn, OUCH!"),
            north: Some(3),
            south: Some(2),
            east: None,
            west: None,
            action: |mut p| {
                p.health -= 5;
                p
            },
        },
        Room::empty_room(),
    ];

    let mut app = App::new(rooms);

    app.describe_current();
    println!("{:?}", app.player);
    // Simulated input
    app.go_north();
    app.go_north();
    app.go_north();
    app.go_north();
    println!("{:?}", app.player);
}

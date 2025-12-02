use ditto::{Counter, Error, Register};

/// Rotation direction and distance
#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(i32),  // L - subtract
    Right(i32), // R - add
}

impl Rotation {
    fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let distance: i32 = chars.collect::<String>().parse().unwrap();

        match direction {
            'L' => Rotation::Left(distance),
            'R' => Rotation::Right(distance),
            _ => unreachable!(),
        }
    }
}

/// Shared dial state using CRDTs
/// - position: Register<i32> for the current dial position
/// - zero_landings: Counter for part 1 (times we land on 0)
/// - zero_crossings: Counter for part 2 (times we cross 0)
struct SharedDial {
    position: Register<i32>,
    zero_landings: Counter,
    zero_crossings: Counter,
}

impl SharedDial {
    fn new() -> Self {
        Self {
            position: Register::new(50), // starts at 50
            zero_landings: Counter::new(0),
            zero_crossings: Counter::new(0),
        }
    }

    /// Clone state to another site
    fn clone_to_site(&self, site_id: u32) -> Self {
        let pos_state = serde_json::to_string(&self.position.state()).unwrap();
        let land_state = serde_json::to_string(&self.zero_landings.state()).unwrap();
        let cross_state = serde_json::to_string(&self.zero_crossings.state()).unwrap();

        Self {
            position: Register::from_state(serde_json::from_str(&pos_state).unwrap(), Some(site_id)).unwrap(),
            zero_landings: Counter::from_state(serde_json::from_str(&land_state).unwrap(), Some(site_id)).unwrap(),
            zero_crossings: Counter::from_state(serde_json::from_str(&cross_state).unwrap(), Some(site_id)).unwrap(),
        }
    }

    /// Apply a rotation and detect zero landings/crossings
    fn rotate(&mut self, rotation: Rotation) -> DialOps {
        let old_pos = *self.position.get();

        let new_pos = match rotation {
            Rotation::Left(d) => (old_pos - d).rem_euclid(100),
            Rotation::Right(d) => (old_pos + d) % 100,
        };

        // Count zero landings (part 1)
        let landing_op = if new_pos == 0 {
            Some(self.zero_landings.increment(1))
        } else {
            None
        };

        // Count zero crossings (part 2)
        let crossings = count_crossings(old_pos, &rotation);
        let crossing_op = if crossings > 0 {
            Some(self.zero_crossings.increment(crossings))
        } else {
            None
        };

        let position_op = self.position.update(new_pos);

        DialOps {
            position: position_op.unwrap(),
            landing: landing_op,
            crossing: crossing_op,
        }
    }

    /// Sync operations from another site
    fn sync(&mut self, ops: DialOps) {
        self.position.execute_op(ops.position);
        if let Some(op) = ops.landing {
            self.zero_landings.execute_op(&op.unwrap());
        }
        if let Some(op) = ops.crossing {
            self.zero_crossings.execute_op(&op.unwrap());
        }
    }

    fn get_position(&self) -> i32 {
        *self.position.get()
    }

    fn get_zero_landings(&self) -> i64 {
        self.zero_landings.get()
    }

    fn get_zero_crossings(&self) -> i64 {
        self.zero_crossings.get()
    }
}

/// Operations to sync between sites
struct DialOps {
    position: ditto::register::Op<i32>,
    landing: Option<Result<ditto::counter::Op, Error>>,
    crossing: Option<Result<ditto::counter::Op, Error>>,
}

/// Count how many times we cross zero when moving from old to new
/// Uses the same algorithm as lib.rs part2
fn count_crossings(old: i32, rotation: &Rotation) -> i64 {
    match rotation {
        Rotation::Left(distance) => {
            // Moving left (negative direction)
            if old == 0 {
                (*distance / 100) as i64
            } else if *distance >= old {
                (1 + (*distance - old) / 100) as i64
            } else {
                0
            }
        }
        Rotation::Right(distance) => {
            // Moving right (positive direction)
            ((old + *distance) / 100) as i64
        }
    }
}

fn main() {
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    let rotations: Vec<Rotation> = INPUT.lines().map(Rotation::parse).collect();

    // Site 1 creates the shared dial
    let mut dial1 = SharedDial::new();

    // Site 2 gets a copy of the state
    let mut dial2 = dial1.clone_to_site(2);

    println!("=== Distributed Dial Simulation ===");
    println!("Initial position: {}", dial1.get_position());
    println!();

    // Simulate distributed execution - alternating between sites
    for rotation in rotations.iter() {
        let site = match &rotation {
            Rotation::Left(_) => { 1 },
            Rotation::Right(_) => { 2 },
        };

        let (ops, pos) = if site == 1 {
            let ops = dial1.rotate(*rotation);
            let pos = dial1.get_position();
            (ops, pos)
        } else {
            let ops = dial2.rotate(*rotation);
            let pos = dial2.get_position();
            (ops, pos)
        };

        // Sync to the other site
        if site == 1 {
            dial2.sync(ops);
        } else {
            dial1.sync(ops);
        }

        println!(
            "Site {} applied {:?} -> position: {}",
            site, rotation, pos
        );
    }

    println!();
    println!("=== Final State ===");
    println!("Dial 1 - Position: {}, Zero Landings: {}, Zero Crossings: {}",
        dial1.get_position(),
        dial1.get_zero_landings(),
        dial1.get_zero_crossings()
    );
    println!("Dial 2 - Position: {}, Zero Landings: {}, Zero Crossings: {}",
        dial2.get_position(),
        dial2.get_zero_landings(),
        dial2.get_zero_crossings()
    );

    // Verify both sites have the same state (CRDT convergence)
    assert_eq!(dial1.get_position(), dial2.get_position());
    assert_eq!(dial1.get_zero_landings(), dial2.get_zero_landings());
    assert_eq!(dial1.get_zero_crossings(), dial2.get_zero_crossings());

    println!();
    println!("✓ Both sites converged to the same state!");
    println!();
    println!("Part 1 (zero landings): {}", dial1.get_zero_landings());
    println!("Part 2 (zero crossings): {}", dial1.get_zero_crossings());
}
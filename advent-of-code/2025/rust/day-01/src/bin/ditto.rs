use ditto::{Counter, Error, Register};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Rotation direction and distance
#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(i32),  // L - subtract (handled by Site 1)
    Right(i32), // R - add (handled by Site 2)
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

/// Serialized ops for sending over channels (JSON strings)
#[derive(Debug)]
struct SerializedOps {
    position: String,
    landing: Option<String>,
    crossing: Option<String>,
}

impl From<DialOps> for SerializedOps {
    fn from(ops: DialOps) -> Self {
        Self {
            position: serde_json::to_string(&ops.position).unwrap(),
            landing: ops.landing.map(|r| serde_json::to_string(&r.unwrap()).unwrap()),
            crossing: ops.crossing.map(|r| serde_json::to_string(&r.unwrap()).unwrap()),
        }
    }
}

impl SerializedOps {
    fn apply_to(self, dial: &mut SharedDial) {
        let pos_op: ditto::register::Op<i32> = serde_json::from_str(&self.position).unwrap();
        dial.position.execute_op(pos_op);

        if let Some(landing_json) = self.landing {
            let op: ditto::counter::Op = serde_json::from_str(&landing_json).unwrap();
            dial.zero_landings.execute_op(&op);
        }
        if let Some(crossing_json) = self.crossing {
            let op: ditto::counter::Op = serde_json::from_str(&crossing_json).unwrap();
            dial.zero_crossings.execute_op(&op);
        }
    }
}

/// Messages sent to worker threads
enum WorkerMsg {
    Rotate(Rotation),
    Sync(SerializedOps),
    GetState,
    Shutdown,
}

/// Responses from worker threads
#[derive(Debug)]
enum WorkerResponse {
    RotateComplete { ops: SerializedOps, position: i32 },
    SyncComplete,
    State { position: i32, landings: i64, crossings: i64 },
    ShutdownComplete,
}

/// Spawn a persistent worker thread for a site
fn spawn_worker(
    name: &'static str,
    dial: SharedDial,
    rx: Receiver<WorkerMsg>,
    tx: Sender<WorkerResponse>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name(name.to_string())
        .spawn(move || {
            let mut dial = dial;

            loop {
                match rx.recv() {
                    Ok(WorkerMsg::Rotate(rotation)) => {
                        let ops = dial.rotate(rotation);
                        let position = dial.get_position();
                        println!(
                            "[{}] Applied {:?} -> position: {}",
                            name, rotation, position
                        );
                        tx.send(WorkerResponse::RotateComplete {
                            ops: ops.into(),
                            position,
                        }).unwrap();
                    }
                    Ok(WorkerMsg::Sync(ops)) => {
                        ops.apply_to(&mut dial);
                        println!(
                            "[{}] Synced, position now: {}",
                            name, dial.get_position()
                        );
                        tx.send(WorkerResponse::SyncComplete).unwrap();
                    }
                    Ok(WorkerMsg::GetState) => {
                        tx.send(WorkerResponse::State {
                            position: dial.get_position(),
                            landings: dial.get_zero_landings(),
                            crossings: dial.get_zero_crossings(),
                        }).unwrap();
                    }
                    Ok(WorkerMsg::Shutdown) => {
                        println!("[{}] Shutting down", name);
                        tx.send(WorkerResponse::ShutdownComplete).unwrap();
                        break;
                    }
                    Err(_) => break,
                }
            }
        })
        .unwrap()
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

    // Create channels for left elf (handles Left rotations)
    let (tx1, rx1) = mpsc::channel::<WorkerMsg>();
    let (resp_tx1, resp_rx1) = mpsc::channel::<WorkerResponse>();

    // Create channels for right elf (handles Right rotations)
    let (tx2, rx2) = mpsc::channel::<WorkerMsg>();
    let (resp_tx2, resp_rx2) = mpsc::channel::<WorkerResponse>();

    // Create initial dial state and clone for right elf
    let dial1 = SharedDial::new();
    let dial2 = dial1.clone_to_site(2);

    println!("=== Distributed Dial Simulation (2 Persistent Workers) ===");
    println!("Initial position: {}", dial1.get_position());
    println!();

    // Spawn persistent worker threads
    let handle1 = spawn_worker("left elf", dial1, rx1, resp_tx1);
    let handle2 = spawn_worker("right elf", dial2, rx2, resp_tx2);

    // Process rotations in order
    for rotation in rotations {
        match rotation {
            Rotation::Left(_) => {
                // left elf handles Left rotations
                tx1.send(WorkerMsg::Rotate(rotation)).unwrap();

                // Wait for left elf to complete and get ops
                if let WorkerResponse::RotateComplete { ops, .. } = resp_rx1.recv().unwrap() {
                    // Sync ops to right elf
                    tx2.send(WorkerMsg::Sync(ops)).unwrap();
                    resp_rx2.recv().unwrap(); // wait for sync complete
                }
            }
            Rotation::Right(_) => {
                // right elf handles Right rotations
                tx2.send(WorkerMsg::Rotate(rotation)).unwrap();

                // Wait for right elf to complete and get ops
                if let WorkerResponse::RotateComplete { ops, .. } = resp_rx2.recv().unwrap() {
                    // Sync ops to left elf
                    tx1.send(WorkerMsg::Sync(ops)).unwrap();
                    resp_rx1.recv().unwrap(); // wait for sync complete
                }
            }
        }
    }

    // Get final state from both sites
    tx1.send(WorkerMsg::GetState).unwrap();
    tx2.send(WorkerMsg::GetState).unwrap();

    let state1 = resp_rx1.recv().unwrap();
    let state2 = resp_rx2.recv().unwrap();

    println!();
    println!("=== Final State ===");
    if let WorkerResponse::State { position, landings, crossings } = state1 {
        println!("left elf  - Position: {}, Landings: {}, Crossings: {}", position, landings, crossings);
    }
    if let WorkerResponse::State { position, landings, crossings } = state2 {
        println!("right elf - Position: {}, Landings: {}, Crossings: {}", position, landings, crossings);
    }

    // Shutdown workers
    tx1.send(WorkerMsg::Shutdown).unwrap();
    tx2.send(WorkerMsg::Shutdown).unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!();
    println!("Part 1 (zero landings): check above");
    println!("Part 2 (zero crossings): check above");
}
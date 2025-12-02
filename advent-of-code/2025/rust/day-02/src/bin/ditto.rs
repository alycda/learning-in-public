//! Distributed day-02 solution using CRDT Sets for p2p sync
//!
//! Demonstrates:
//! - Idempotent work distribution (overlapping ranges handled by Set dedup)
//! - Full mesh peer topology
//! - Random sync intervals
//! - Eventual consistency across all peers

use ditto::Set;
use rand::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

/// Parse input into ranges
fn parse_ranges(input: &str) -> Vec<(u128, u128)> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|id| {
            let (a, b) = id.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

/// Check if number has repeating halves (part 1 logic)
fn has_repeating_halves(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    s[..mid] == s[mid..]
}

/// Find largest repeating pattern (part 2 logic)
fn has_repeating_pattern(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    (1..=len / 2).rev().any(|pattern_len| {
        len % pattern_len == 0 && s[..pattern_len].repeat(len / pattern_len) == s
    })
}

/// Messages between peers
#[derive(Debug)]
enum PeerMessage {
    /// Sync request with serialized set state
    SyncState { from_peer: usize, state: String },
    /// Work assignment
    ProcessRange { range: (u128, u128) },
    /// Request current state for sync
    GetState,
    /// Request final results
    GetResults,
    /// Shutdown signal
    Shutdown,
}

/// Responses from peers
#[derive(Debug)]
enum PeerResponse {
    SyncComplete,
    RangeProcessed { range: (u128, u128), found: usize },
    /// Current state for syncing to other peers
    State { state: String },
    Results { part1_sum: u128, part2_sum: u128, part1_count: usize, part2_count: usize },
    ShutdownComplete,
}

/// A peer with its own CRDT sets
struct Peer {
    id: usize,
    part1_set: Set<u128>,
    part2_set: Set<u128>,
    rx: Receiver<PeerMessage>,
    tx: Sender<PeerResponse>,
}

impl Peer {
    fn new(id: usize, rx: Receiver<PeerMessage>, tx: Sender<PeerResponse>) -> Self {
        Self {
            id,
            part1_set: Set::new(),
            part2_set: Set::new(),
            rx,
            tx,
        }
    }

    /// Process a range and add matching numbers to sets
    fn process_range(&mut self, range: (u128, u128)) -> usize {
        let mut found = 0;
        for n in range.0..=range.1 {
            if has_repeating_halves(n) {
                self.part1_set.insert(n);
                found += 1;
            }
            if has_repeating_pattern(n) {
                self.part2_set.insert(n);
                found += 1;
            }
        }
        found
    }

    /// Get serialized state for syncing
    fn get_state(&self) -> String {
        serde_json::json!({
            "part1": self.part1_set.state(),
            "part2": self.part2_set.state(),
        })
        .to_string()
    }

    /// Merge state from another peer
    fn merge_state(&mut self, _from_peer: usize, state_json: &str) {
        let state: serde_json::Value = serde_json::from_str(state_json).unwrap();

        // Merge part1 set - pass the state directly
        if let Some(part1_state) = state.get("part1") {
            let remote_state = serde_json::from_value(part1_state.clone()).unwrap();
            self.part1_set.merge(remote_state);
        }

        // Merge part2 set
        if let Some(part2_state) = state.get("part2") {
            let remote_state = serde_json::from_value(part2_state.clone()).unwrap();
            self.part2_set.merge(remote_state);
        }
    }

    /// Run the peer's event loop
    fn run(mut self) {
        loop {
            match self.rx.recv() {
                Ok(PeerMessage::ProcessRange { range }) => {
                    let found = self.process_range(range);
                    println!(
                        "[Peer {}] Processed range {:?}, found {} matches",
                        self.id, range, found
                    );
                    self.tx.send(PeerResponse::RangeProcessed { range, found }).unwrap();
                }
                Ok(PeerMessage::SyncState { from_peer, state }) => {
                    self.merge_state(from_peer, &state);
                    println!(
                        "[Peer {}] Merged state from Peer {}, now has part1: {} items, part2: {} items",
                        self.id, from_peer,
                        self.part1_set.value().len(),
                        self.part2_set.value().len()
                    );
                    self.tx.send(PeerResponse::SyncComplete).unwrap();
                }
                Ok(PeerMessage::GetState) => {
                    self.tx.send(PeerResponse::State { state: self.get_state() }).unwrap();
                }
                Ok(PeerMessage::GetResults) => {
                    let part1_items: Vec<u128> = self.part1_set.value().iter().cloned().collect();
                    let part2_items: Vec<u128> = self.part2_set.value().iter().cloned().collect();

                    self.tx.send(PeerResponse::Results {
                        part1_sum: part1_items.iter().sum(),
                        part2_sum: part2_items.iter().sum(),
                        part1_count: part1_items.len(),
                        part2_count: part2_items.len(),
                    }).unwrap();
                }
                Ok(PeerMessage::Shutdown) => {
                    println!("[Peer {}] Shutting down", self.id);
                    self.tx.send(PeerResponse::ShutdownComplete).unwrap();
                    break;
                }
                Err(_) => break,
            }
        }
    }
}

/// Spawn a peer thread
fn spawn_peer(
    id: usize,
    rx: Receiver<PeerMessage>,
    tx: Sender<PeerResponse>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name(format!("peer-{}", id))
        .spawn(move || {
            let peer = Peer::new(id, rx, tx);
            peer.run();
        })
        .unwrap()
}

/// Coordinator facilitates sync between two random peers
fn sync_peers(
    rng: &mut ThreadRng,
    peer_txs: &[Sender<PeerMessage>],
    peer_rxs: &[Receiver<PeerResponse>],
    num_peers: usize,
) {
    // Pick two random different peers
    let peer_a = rng.gen_range(0..num_peers);
    let mut peer_b = rng.gen_range(0..num_peers);
    while peer_b == peer_a {
        peer_b = rng.gen_range(0..num_peers);
    }

    // Get state from peer A
    peer_txs[peer_a].send(PeerMessage::GetState).unwrap();
    let state_a = loop {
        if let Ok(PeerResponse::State { state }) = peer_rxs[peer_a].try_recv() {
            break state;
        }
        thread::sleep(Duration::from_micros(100));
    };

    // Get state from peer B
    peer_txs[peer_b].send(PeerMessage::GetState).unwrap();
    let state_b = loop {
        if let Ok(PeerResponse::State { state }) = peer_rxs[peer_b].try_recv() {
            break state;
        }
        thread::sleep(Duration::from_micros(100));
    };

    // Send A's state to B, and B's state to A (bidirectional sync)
    peer_txs[peer_a]
        .send(PeerMessage::SyncState {
            from_peer: peer_b,
            state: state_b,
        })
        .unwrap();
    peer_txs[peer_b]
        .send(PeerMessage::SyncState {
            from_peer: peer_a,
            state: state_a,
        })
        .unwrap();

    // Wait for sync to complete
    let mut synced = 0;
    while synced < 2 {
        for rx in peer_rxs {
            if let Ok(PeerResponse::SyncComplete) = rx.try_recv() {
                synced += 1;
            }
        }
        thread::sleep(Duration::from_micros(100));
    }

    println!("  [Sync] Peer {} <-> Peer {}", peer_a, peer_b);
}

fn main() {
    let mut rng = thread_rng();

    // Random number of peers (2-8)
    let num_peers: usize = rng.gen_range(2..=8);
    println!("=== Distributed Day-02 with {} Peers ===\n", num_peers);

    // Parse ranges
    let ranges = parse_ranges(INPUT);
    println!("Total ranges to process: {}", ranges.len());

    // Create channels for each peer
    let mut peer_txs: Vec<Sender<PeerMessage>> = Vec::new();
    let mut peer_rxs: Vec<Receiver<PeerResponse>> = Vec::new();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for id in 0..num_peers {
        let (tx, rx) = mpsc::channel::<PeerMessage>();
        let (resp_tx, resp_rx) = mpsc::channel::<PeerResponse>();

        peer_txs.push(tx);
        peer_rxs.push(resp_rx);

        handles.push(spawn_peer(id, rx, resp_tx));
    }

    // Randomly assign each range to 1-3 peers (tests idempotency)
    println!("\n--- Work Distribution ---");
    let mut assignments: HashMap<usize, Vec<(u128, u128)>> = HashMap::new();
    let mut overlap_count = 0;

    for range in &ranges {
        let num_assignees = rng.gen_range(1..=3.min(num_peers));
        if num_assignees > 1 {
            overlap_count += 1;
        }

        let mut assigned_peers: Vec<usize> = (0..num_peers).collect();
        assigned_peers.shuffle(&mut rng);

        for &peer_id in assigned_peers.iter().take(num_assignees) {
            assignments.entry(peer_id).or_default().push(*range);
        }
    }

    for (peer_id, ranges) in &assignments {
        println!("  Peer {}: {} ranges", peer_id, ranges.len());
    }
    println!("  ({} ranges assigned to multiple peers for idempotency test)", overlap_count);

    // Send work to peers and randomly sync during processing
    println!("\n--- Processing & Random Sync ---");
    let total_assignments: usize = assignments.values().map(|v| v.len()).sum();

    // Send all work
    for (peer_id, peer_ranges) in &assignments {
        for range in peer_ranges {
            peer_txs[*peer_id]
                .send(PeerMessage::ProcessRange { range: *range })
                .unwrap();
        }
    }

    // Process responses and randomly trigger syncs
    let mut processed = 0;
    let mut sync_count = 0;

    while processed < total_assignments {
        for rx in &peer_rxs {
            if let Ok(PeerResponse::RangeProcessed { .. }) = rx.try_recv() {
                processed += 1;

                // Random chance (20%) to sync after each range processed
                if num_peers > 1 && rng.gen_bool(0.2) {
                    sync_peers(&mut rng, &peer_txs, &peer_rxs, num_peers);
                    sync_count += 1;

                    // Random delay after sync (1-50ms)
                    let delay = rng.gen_range(1..50);
                    thread::sleep(Duration::from_millis(delay));
                }
            }
        }
        thread::sleep(Duration::from_micros(500));
    }

    println!("  Completed {} sync rounds during processing", sync_count);

    // Final sync phase: ensure full convergence with mesh sync
    println!("\n--- Final Mesh Sync ---");
    // Do N*(N-1)/2 syncs to ensure every pair has synced at least once
    let full_mesh_syncs = (num_peers * (num_peers - 1)) / 2;
    for i in 0..num_peers {
        for j in (i + 1)..num_peers {
            // Get state from peer i
            peer_txs[i].send(PeerMessage::GetState).unwrap();
            let state_i = loop {
                if let Ok(PeerResponse::State { state }) = peer_rxs[i].recv() {
                    break state;
                }
            };

            // Get state from peer j
            peer_txs[j].send(PeerMessage::GetState).unwrap();
            let state_j = loop {
                if let Ok(PeerResponse::State { state }) = peer_rxs[j].recv() {
                    break state;
                }
            };

            // Bidirectional sync
            peer_txs[i]
                .send(PeerMessage::SyncState { from_peer: j, state: state_j })
                .unwrap();
            peer_txs[j]
                .send(PeerMessage::SyncState { from_peer: i, state: state_i })
                .unwrap();

            // Wait for completion
            for _ in 0..2 {
                for rx in &peer_rxs {
                    while let Ok(resp) = rx.try_recv() {
                        if matches!(resp, PeerResponse::SyncComplete) {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("  Completed {} full mesh sync pairs", full_mesh_syncs);

    // Collect results from all peers
    println!("\n--- Final Results ---");
    let mut results: Vec<(u128, u128, usize, usize)> = Vec::new();

    for i in 0..num_peers {
        peer_txs[i].send(PeerMessage::GetResults).unwrap();
    }

    for (id, rx) in peer_rxs.iter().enumerate() {
        loop {
            match rx.recv().unwrap() {
                PeerResponse::Results { part1_sum, part2_sum, part1_count, part2_count } => {
                    println!(
                        "  Peer {}: Part1 = {} ({} items), Part2 = {} ({} items)",
                        id, part1_sum, part1_count, part2_sum, part2_count
                    );
                    results.push((part1_sum, part2_sum, part1_count, part2_count));
                    break;
                }
                _ => continue,
            }
        }
    }

    // Shutdown peers
    for tx in &peer_txs {
        tx.send(PeerMessage::Shutdown).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify convergence
    println!("\n--- Convergence Check ---");
    let first = &results[0];
    let converged = results.iter().all(|r| r == first);

    if converged {
        println!("All {} peers converged to same state!", num_peers);
        println!("\nFinal Answers:");
        println!("  Part 1: {} (sum of {} repeating-half numbers)", first.0, first.2);
        println!("  Part 2: {} (sum of {} repeating-pattern numbers)", first.1, first.3);
    } else {
        println!("Peers did NOT converge (unexpected for CRDTs!)");
        for (id, r) in results.iter().enumerate() {
            println!("  Peer {}: Part1={}, Part2={}", id, r.0, r.1);
        }
    }

    // Compare with expected
    println!("\n--- Validation ---");
    let expected_p1: u128 = 1227775554;
    let expected_p2: u128 = 4174379265;

    if first.0 == expected_p1 && first.1 == expected_p2 {
        println!("PASS: Results match expected values!");
    } else {
        println!("MISMATCH:");
        println!("  Part 1: got {}, expected {}", first.0, expected_p1);
        println!("  Part 2: got {}, expected {}", first.1, expected_p2);
    }
}

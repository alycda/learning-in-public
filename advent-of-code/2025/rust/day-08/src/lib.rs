use glam::IVec3;

// pub fn part1(input: &str) -> u32 {
//     let junction_boxes: Vec<IVec3> = input
//         .lines()
//         .map(|line| {
//             IVec3::from_slice(
//                 &line
//                     .split(',')
//                     .map(|s| s.parse::<i32>().unwrap())
//                     .collect::<Vec<_>>(),
//             )
//         })
//         .collect();

//     let mut min_distance = u32::MAX;

//     for i in 0..junction_boxes.len() {
//         for j in (i + 1)..junction_boxes.len() {
//             let dist = junction_boxes[i].manhattan_distance(junction_boxes[j]);
//             min_distance = min_distance.min(dist);
//         }
//     }

//     min_distance
// }

// use std::collections::{HashMap, HashSet};

// pub fn part1(input: &str) -> usize {
//     let points: Vec<IVec3> = input
//         .lines()
//         .map(|line| {
//             IVec3::from_slice(
//                 &line.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>(),
//             )
//         })
//         .collect();

//     // Step 1: Each point connects to its nearest neighbor
//     let mut edges: Vec<(usize, usize)> = Vec::new();
    
//     for i in 0..points.len() {
//         let mut nearest = 0;
//         let mut min_dist = u32::MAX;
        
//         for j in 0..points.len() {
//             if i != j {
//                 let dist = points[i].manhattan_distance(points[j]);
//                 if dist < min_dist {
//                     min_dist = dist;
//                     nearest = j;
//                 }
//             }
//         }
//         edges.push((i, nearest));
//     }

//     // Step 2: Build adjacency and find connected components (Union-Find)
//     let mut parent: Vec<usize> = (0..points.len()).collect();
    
//     fn find(parent: &mut Vec<usize>, i: usize) -> usize {
//         if parent[i] != i {
//             parent[i] = find(parent, parent[i]);
//         }
//         parent[i]
//     }
    
//     for (a, b) in edges {
//         let pa = find(&mut parent, a);
//         let pb = find(&mut parent, b);
//         if pa != pb {
//             parent[pa] = pb;
//         }
//     }

//     // Step 3: Count component sizes
//     let mut component_sizes: HashMap<usize, usize> = HashMap::new();
//     for i in 0..points.len() {
//         let root = find(&mut parent, i);
//         *component_sizes.entry(root).or_insert(0) += 1;
//     }

//     dbg!(&component_sizes);

//     // Step 4: Multiply 3 largest
//     let mut sizes: Vec<usize> = component_sizes.values().cloned().collect();
//     sizes.sort_by(|a, b| b.cmp(a)); // descending
    
//     sizes.iter().take(3).product()
// }

use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let points: Vec<IVec3> = input
        .lines()
        .map(|line| {
            IVec3::from_slice(
                &line.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>(),
            )
        })
        .collect();

    // Step 1: Calculate ALL pairwise distances (squared, to avoid sqrt)
    let mut pairs: Vec<(i32, usize, usize)> = Vec::new();
    
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let diff = points[i] - points[j];
            let dist_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            pairs.push((dist_sq, i, j));
        }
    }

    // Step 2: Sort by distance
    pairs.sort_by_key(|&(dist, _, _)| dist);

    // Step 3: Union-Find to track circuits
    let mut parent: Vec<usize> = (0..points.len()).collect();
    
    fn find(parent: &mut Vec<usize>, i: usize) -> usize {
        if parent[i] != i {
            parent[i] = find(parent, parent[i]);
        }
        parent[i]
    }

    // Connect the closest pairs
    for &(_, a, b) in pairs.iter().take(points.len() / 2) {
        let pa = find(&mut parent, a);
        let pb = find(&mut parent, b);
        if pa != pb {
            parent[pa] = pb;
        }
    }

    // Step 4: Count component sizes
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..points.len() {
        let root = find(&mut parent, i);
        *sizes.entry(root).or_insert(0) += 1;
    }

    // Step 5: Multiply 3 largest
    let mut sizes: Vec<usize> = sizes.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));
    
    sizes.iter().take(3).product()
}


pub fn part2(input: &str) -> usize {
    todo!()
}

pub const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 40);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE_INPUT), 0);
    // }
}
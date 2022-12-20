#![feature(iter_next_chunk)]

use aoc::{main, sample};
use std::collections::{HashMap, VecDeque};

main!(d16, "Proboscidea Volcanium");

fn d16(input: &str) -> (u32, u32) {
    let mut flows = HashMap::new();
    let mut edges = HashMap::new();
    for line in input.lines() {
        let [start, end] = line.split("; ").next_chunk().unwrap();
        let [valve, flow] = start.strip_prefix("Valve ").unwrap()
            .split(" has flow rate=").next_chunk().unwrap();
        let flow: u32 = flow.parse().unwrap();
        let end = end.strip_prefix("tunnels lead to valves ").unwrap_or(end);
        let end = end.strip_prefix("tunnel leads to valve ").unwrap_or(end);
        let tunnels: Vec<_> = end.split(", ").collect();
        flows.insert(valve, flow);
        edges.insert(valve, tunnels);
    }
    let targets: Vec<_> = flows.iter().filter_map(|(&valve, flow)| (*flow > 0).then_some(valve)).collect();
    let mut distances: HashMap<_, _> = targets.iter().map(|&valve| (valve, bfs_distances(valve, &edges))).collect();
    distances.insert(&"AA", bfs_distances(&"AA", &edges));
    let mut part1 = 0;
    dfs_part1(&mut part1, &["AA"], 30, 0, &targets, &flows, &distances);
    let mut part2 = 0;
    dfs_part2(&mut part2, &["AA"], 26, &["AA"], 26, 0, &targets, &flows, &distances);
    (part1, part2)
}

fn bfs_distances<'a>(from: &'a str, edges: &'a HashMap<&str, Vec<&str>>) -> HashMap<&'a str, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    while let Some((node, distance)) = queue.pop_front() {
        distances.insert(node, distance);
        for next in edges[node].iter() {
            if !distances.contains_key(next) {
                queue.push_back((next, distance + 1));
            }
        }
    }
    distances
}

fn dfs_part1(best: &mut u32, path: &[&str], time: u32, pressure: u32,
             targets: &Vec<&str>, flows: &HashMap<&str, u32>,
             distances: &HashMap<&str, HashMap<&str, u32>>) {
    if pressure > *best {
        *best = pressure;
    }
    for target in targets {
        if path.contains(target) {
            continue;
        }
        let distance = distances[path[path.len() - 1]][target];
        if distance + 1 >= time {
            continue;
        }
        let remaining = time - distance - 1;
        dfs_part1(best,
                  &[path, &[target]].concat(), remaining,
                  pressure + flows[target] * remaining,
                  targets, flows, distances);
    }
}

fn dfs_part2(best: &mut u32, path1: &[&str], time1: u32,
             path2: &[&str], time2: u32, pressure: u32,
             targets: &Vec<&str>, flows: &HashMap<&str, u32>,
             distances: &HashMap<&str, HashMap<&str, u32>>) {
    if pressure > *best {
        *best = pressure;
    }
    let path = if time1 < time2 { path2 } else { path1 };
    let time = if time1 < time2 { time2 } else { time1 };
    for target in targets {
        if path1.contains(target) || path2.contains(target) {
            continue;
        }
        let distance = distances[path[path.len() - 1]][target];
        if distance + 1 >= time {
            continue;
        }
        let remaining = time - distance - 1;
        if time1 < time2 {
            dfs_part2(best,
                      path1, time1,
                      &[path, &[target]].concat(), remaining,
                      pressure + flows[target] * remaining,
                      targets, flows, distances);
        } else {
            dfs_part2(best,
                      &[path, &[target]].concat(), remaining,
                      path2, time2,
                      pressure + flows[target] * remaining,
                      targets, flows, distances);
        }
    }
}

sample!(d16, 1651, 1707, "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
");

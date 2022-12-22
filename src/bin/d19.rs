#![feature(iter_next_chunk)]

use aoc::{main, sample};

main!(d19, "Not Enough Minerals");

struct Blueprint {
    id: u32,
    ore_ore: u32,
    clay_ore: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32
}

struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}

fn d19(input: &str) -> (u32, u32) {
    let blueprints: Vec<_> = input.lines().enumerate()
        .map(|(i, line)| {
            let [
                ore_ore, clay_ore,
                obsidian_ore, obsidian_clay,
                geode_ore, geode_obsidian
            ]: [u32; 6] = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .next_chunk().unwrap();
            Blueprint {
                id: i as u32 + 1,
                ore_ore, clay_ore,
                obsidian_ore, obsidian_clay,
                geode_ore, geode_obsidian
            }
        })
        .collect();
    let robots = Inventory { ore: 1, clay: 0, obsidian: 0, geode: 0 };
    let resources = Inventory { ore: 0, clay: 0, obsidian: 0, geode: 0 };
    let part1 = blueprints.iter().map(|blueprint| {
        let mut best = 0;
        dfs(&mut best, blueprint, 24, Inventory { ..robots }, Inventory { ..resources });
        best * blueprint.id
    }).sum();
    let part2 = blueprints.iter().take(3).map(|blueprint| {
        let mut best = 0;
        dfs(&mut best, blueprint, 32, Inventory { ..robots }, Inventory { ..resources });
        best
    }).product();
    (part1, part2)
}

fn dfs(best: &mut u32, blueprint: &Blueprint, minutes: u32, robots: Inventory, resources: Inventory) {
    if resources.geode > *best {
        *best = resources.geode;
    }
    if minutes == 0 {
        return;
    }
    if minutes * robots.geode + resources.geode + minutes * (minutes - 1) / 2 < *best {
        // hopeless
        return;
    }
    let collected = Inventory {
        ore: resources.ore + robots.ore,
        clay: resources.clay + robots.clay,
        obsidian: resources.obsidian + robots.obsidian,
        geode: resources.geode + robots.geode
    };
    if resources.ore >= blueprint.ore_ore {
        dfs(best, blueprint, minutes - 1,
            Inventory { ore: robots.ore + 1, ..robots },
            Inventory { ore: collected.ore - blueprint.ore_ore, ..collected });
    }
    if resources.ore >= blueprint.clay_ore {
        dfs(best, blueprint, minutes - 1,
            Inventory { clay: robots.clay + 1, ..robots },
            Inventory { ore: collected.ore - blueprint.clay_ore, ..collected });
    }
    if resources.ore >= blueprint.obsidian_ore && resources.clay >= blueprint.obsidian_clay {
        dfs(best, blueprint, minutes - 1,
            Inventory { obsidian: robots.obsidian + 1, ..robots },
            Inventory { ore: collected.ore - blueprint.obsidian_ore, clay: collected.clay - blueprint.obsidian_clay, ..collected });
    }
    if resources.ore >= blueprint.geode_ore && resources.obsidian >= blueprint.geode_obsidian {
        dfs(best, blueprint, minutes - 1,
            Inventory { geode: robots.geode + 1, ..robots },
            Inventory { ore: collected.ore - blueprint.geode_ore, obsidian: collected.obsidian - blueprint.geode_obsidian, ..collected });
    }
    dfs(best, blueprint, minutes - 1, robots, collected);
}

sample!(d19, 33, 3472, "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
");

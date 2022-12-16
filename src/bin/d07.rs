use aoc::{main, sample};
use std::collections::HashMap;

main!(d07, "No Space Left On Device");

struct Dir {
    files: HashMap<String, u32>,
    subdirs: HashMap<String, Dir>
}

impl Dir {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
            subdirs: HashMap::new()
        }
    }

    fn dfs_size(&self, part1: &mut u32) -> u32 {
        let size = self.files.values().sum::<u32>()
            + self.subdirs.values()
                .map(|subdir| subdir.dfs_size(part1))
                .sum::<u32>();
        if size <= 100_000 {
            *part1 += size;
        }
        size
    }

    fn dfs_delete(&self, part2: &mut u32, needed: u32) -> u32 {
        let size = self.files.values().sum::<u32>()
            + self.subdirs.values()
                .map(|subdir| subdir.dfs_delete(part2, needed))
                .sum::<u32>();
        if size > needed && (*part2 == 0 || size < *part2) {
            *part2 = size;
        }
        size
    }
}

fn d07(input: &str) -> (u32, u32) {
    let dir = parse(input);
    let mut part1 = 0;
    let used = dir.dfs_size(&mut part1);
    let needed = used - 40_000_000;
    let mut part2 = 0;
    dir.dfs_delete(&mut part2, needed);
    (part1, part2)
}

fn parse(input: &str) -> Dir {
    let mut root = Dir::new();
    let mut breadcrumbs: Vec<String> = Vec::new();
    for line in input.lines() {
        let mut pwd = &mut root;
        for breadcrumb in &breadcrumbs {
            pwd = &mut *pwd.subdirs.get_mut(breadcrumb).unwrap();
        }
        match &line.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => breadcrumbs.truncate(1),
            ["$", "cd", ".."] => drop(breadcrumbs.pop()),
            ["$", "cd", subdir] => breadcrumbs.push(subdir.to_string()),
            ["$", ..] => (),
            ["dir", dir] => {
                pwd.subdirs.entry(dir.to_string()).or_insert_with(Dir::new);
            },
            [size, file] => {
                pwd.files.entry(file.to_string()).or_insert_with(|| size.parse().unwrap());
            },
            _ => unreachable!()
        }
    }
    root
}

sample!(d07, 95437, 24933642, "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
");

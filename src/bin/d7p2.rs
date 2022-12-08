use std::collections::HashMap;

#[derive(Debug)]
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
}

fn main() {
    let mut root_dir = Dir::new();
    let mut breadcrumbs: Vec<String> = Vec::new();
    for line in std::io::stdin().lines() {
        let mut cur_dir = &mut root_dir;
        for breadcrumb in &breadcrumbs {
            cur_dir = &mut *cur_dir.subdirs.get_mut(breadcrumb).unwrap();
        }
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        match &parts[..] {
            ["$", "cd", "/"] => breadcrumbs.truncate(1),
            ["$", "cd", ".."] => drop(breadcrumbs.pop()),
            ["$", "cd", subdir] => breadcrumbs.push(subdir.to_string()),
            ["$", ..] => (),
            ["dir", dir] => {
                let dir = dir.to_string();
                if !cur_dir.subdirs.contains_key(&dir) {
                    cur_dir.subdirs.insert(dir, Dir::new());
                }
            },
            [size, file] => {
                let file = file.to_string();
                if !cur_dir.files.contains_key(&file) {
                    cur_dir.files.insert(file, size.parse().unwrap());
                }
            },
            [..] => ()
        }
    }
    let used = dir_dfs_size(&root_dir);
    let needed = used - 40_000_000;
    let mut answer = 0;
    dir_dfs_answer(&mut answer, needed, &root_dir);
    println!("{}", answer);
}

fn dir_dfs_size(dir: &Dir) -> u32 {
    dir.files.values().sum::<u32>()
        + dir.subdirs.values().map(|subdir| dir_dfs_size(subdir)).sum::<u32>()
}

fn dir_dfs_answer(answer: &mut u32, needed: u32, dir: &Dir) -> u32 {
    let size = dir.files.values().sum::<u32>()
        + dir.subdirs.values().map(|subdir| dir_dfs_answer(answer, needed, subdir)).sum::<u32>();
    if size > needed && (*answer == 0 || size < *answer) {
        *answer = size;
    }
    size
}

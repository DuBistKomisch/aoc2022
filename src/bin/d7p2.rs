#[derive(Debug)]
struct Dir {
    files: std::collections::HashMap<String, u32>,
    subdirs: std::collections::HashMap<String, Box<Dir>>
}

impl Dir {
    fn new() -> Self {
        Self {
            files: std::collections::HashMap::new(),
            subdirs: std::collections::HashMap::new()
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut root_dir = Dir::new();
    let mut breadcrumbs: Vec<String> = Vec::new();
    for line in std::io::stdin().lines() {
        let mut cur_dir = &mut root_dir;
        for breadcrumb in &breadcrumbs {
            cur_dir = &mut *cur_dir.subdirs.get_mut(breadcrumb).unwrap();
        }
        let line = line?;
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "$" => {
                if parts[1] == "cd" {
                    if parts[2] == "/" {
                        breadcrumbs.truncate(1);
                    } else if parts[2] == ".." {
                        breadcrumbs.pop();
                    } else {
                        breadcrumbs.push(parts[2].to_string());
                    }
                }
            },
            "dir" => {
                if !cur_dir.subdirs.contains_key(parts[1]) {
                    cur_dir.subdirs.insert(parts[1].to_string(), Box::new(Dir::new()));
                }
            },
            _ => {
                if !cur_dir.files.contains_key(parts[1]) {
                    cur_dir.files.insert(parts[1].to_string(), parts[0].parse().unwrap());
                }
            }
        }
    }
    let used = dir_dfs_size(&root_dir);
    let needed = used - 40_000_000;
    let mut answer = 0;
    dir_dfs_answer(&mut answer, needed, &root_dir);
    println!("{}", answer);
    Ok(())
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

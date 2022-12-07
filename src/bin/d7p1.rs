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
    let mut answer = 0;
    dir_dfs_size(&mut answer, &root_dir, &[]);
    println!("{}", answer);
    Ok(())
}

fn dir_dfs_size(answer: &mut u32, dir: &Dir, breadcrumbs: &[String]) -> u32 {
    let size = dir.files.values().sum::<u32>()
        + dir.subdirs.iter().map(|(name, subdir)| dir_dfs_size(answer, subdir, &[breadcrumbs, &[name.to_owned()]].concat())).sum::<u32>();
    if size <= 100_000 {
        *answer += size;
    }
    size
}

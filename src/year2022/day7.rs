use super::super::common::Result;

pub enum FsRecord {
    File(usize),
    Dir(Box<Directory>),
}

use std::collections::HashMap;

pub struct Directory {
    records: HashMap<String, FsRecord>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            records: HashMap::new(),
        }
    }
    fn add_file(&mut self, path: &[String], size: usize) {
        if path.len() == 1 {
            self.records
                .insert(path[0].to_string(), FsRecord::File(size));
        } else if let Some(FsRecord::Dir(dir)) = self.records.get_mut(&path[0])
        {
            dir.add_file(&path[1..], size);
        }
    }
    fn add_dir(&mut self, path: &[String]) {
        if path.len() == 1 {
            self.records
                .entry(path[0].to_string())
                .or_insert_with(|| FsRecord::Dir(Box::new(Directory::new())));
        } else if let Some(FsRecord::Dir(dir)) = self.records.get_mut(&path[0])
        {
            dir.add_dir(&path[1..]);
        }
    }
    fn get_sizes<P>(&self, func: P) -> (usize, usize)
    where
        P: Fn(usize) -> usize + Clone,
    {
        let mut inner_size = 0;
        let mut inner_filtered = 0;
        for record in self.records.values() {
            match record {
                &FsRecord::File(size) => inner_size += size,
                FsRecord::Dir(subdir) => {
                    let (total, filtered) = subdir.get_sizes(func.clone());
                    inner_size += total;
                    inner_filtered += filtered;
                }
            }
        }
        inner_filtered += func(inner_size);
        (inner_size, inner_filtered)
    }
    fn get_best_size(&self, target: usize) -> (usize, usize) {
        let mut inner_size = 0;
        let mut best = 0;
        for record in self.records.values() {
            match record {
                &FsRecord::File(size) => inner_size += size,
                FsRecord::Dir(subdir) => {
                    let (total, current_best) = subdir.get_best_size(target);
                    inner_size += total;
                    if current_best > target
                        && (best == 0 || current_best < best)
                    {
                        best = current_best;
                    }
                }
            }
        }
        if best == 0 || (inner_size > target && inner_size < best) {
            best = inner_size;
        }
        (inner_size, best)
    }
}

pub fn parse_input(input: &str) -> Result<Directory> {
    let mut root = Directory::new();
    let mut path = vec![];
    for instruction in input.lines() {
        if instruction.starts_with("$ cd") {
            match &instruction[5..] {
                "/" => {
                    path.clear();
                }
                ".." => {
                    path.pop();
                }
                name => {
                    path.push(name.to_string());
                    root.add_dir(&path);
                }
            }
        } else if instruction != "$ ls" {
            //executing ls
            let (typ, name) =
                scan_fmt::scan_fmt!(instruction, "{} {}", String, String)?;
            path.push(name);
            if typ == "dir" {
                root.add_dir(&path);
            } else {
                root.add_file(&path, typ.parse()?);
            }
            path.pop();
        }
    }
    Ok(root)
}

pub fn task1(root: &Directory) -> Result<usize> {
    Ok(root
        .get_sizes(|size| if size <= 100_000 { size } else { 0 })
        .1)
}

pub fn task2(root: &Directory) -> Result<usize> {
    let current_size = root.get_sizes(|_| 0).0;
    let unused = 70_000_000 - current_size;
    let needed = 30_000_000 - unused;
    Ok(root.get_best_size(needed).1)
}

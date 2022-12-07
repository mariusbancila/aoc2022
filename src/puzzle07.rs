use crate::utils;
use std::{path::Path, collections::BTreeMap};

type FsId = i32;

#[derive(Default, Debug, PartialEq)]
struct FsObject {
    // data
    name    : String,
    size    : i32,

    // metadata
    id      : FsId,
    parent  : Option<FsId>,
    entries : Vec<FsId>
}

impl FsObject {
    fn new(n : &str, s : i32, i : FsId, p : Option<FsId>) -> FsObject {
        FsObject { name: n.to_string(), size: s, id : i, parent: p, entries: Vec::new() }
    }

    fn add_child(&mut self, i : FsId) {
        self.entries.push(i);
    }

    fn is_directory(&self) -> bool {
        self.entries.len() > 0
    }
}

struct Filesystem {
    items : BTreeMap<FsId, FsObject>,
    root : FsId,

    current_id : FsId
}

impl Filesystem {
    fn new(r : &str) -> Filesystem {
        let index = 1;
        let item = FsObject::new(r, 0, index, None);

        let mut fs = Filesystem { items: BTreeMap::new(), root: index, current_id: index };
        fs.items.insert(index, item);

        fs
    }

    fn parent_of(&self, i: FsId) -> Option<FsId> {
        for e in &self.items {
            if e.1.id == i {
                return e.1.parent;
            }
        }

        None
    }

    fn child_id(&self, i : FsId, n: &str) -> Option<FsId> {
        if self.items.contains_key(&i) {
            let node = self.items.get(&i).unwrap();

            for e in &node.entries {
                let child = self.items.get(&e).unwrap();
                if child.name == n {
                    return Some(e.clone());
                }
            }
        }

        None
    }

    fn add(&mut self, n: &str, s: i32, p: FsId) {
        let new_node_id = self.current_id + 1;

        if self.items.contains_key(&p) {
            let new_node = FsObject::new(n, s, new_node_id, Some(p));            
            self.items.insert(new_node_id, new_node);

            self.items.entry(p).or_default().add_child(new_node_id);
            self.current_id = new_node_id;
        }        
    }

    fn size_of(&self, i : FsId) -> i32 {
        let mut total_size = 0;

        if let Some(node) = self.items.get(&i) {
            total_size = node.size;

            for cid in &node.entries {
                total_size += self.size_of(cid.clone());
            }
        }

        total_size
    }

    fn find_size_of_candidates(&self, i: FsId, total : &mut i32) -> i32 {
        let mut size = 0;

        if let Some(node) = self.items.get(&i) {
            size = node.size;

            for cid in &node.entries {
                size += self.find_size_of_candidates(*cid, total);
            }

            if node.is_directory() && size <= 100000 {
                *total += size;
            }
        }

        size
    }

    fn find_size_to_cleanup(&self, i : FsId, minimum : i32, found : &mut i32) -> i32 {
        let mut size = 0;        

        if let Some(node) = self.items.get(&i) {
            size = node.size;

            for cid in &node.entries {
                size += self.find_size_to_cleanup(*cid, minimum, found);
            }

            if node.is_directory() && size <= *found && size >= minimum {
                *found = size;
            }
        }

        size
    }

}

fn parse_input<P>(filename: P) -> Filesystem
where P : AsRef<Path> {
    let mut fs = Filesystem::new("/");
    let mut current_directory : FsId = fs.root;

    if let Ok(lines) = utils::read_lines(filename) {
        let mut is_listing = false;

        for line in lines {
            if let Ok(cmd) = line {
                if cmd.starts_with("$") {
                    if cmd == "$ ls" {
                        is_listing = true;
                    }
                    else {
                        is_listing = false;

                        if cmd.starts_with("$ cd ") {
                            let dirname = &cmd[5..];

                            if dirname == "/" {
                                current_directory = fs.root;
                            }
                            else if dirname == ".." {
                                if let Some(p) = fs.parent_of(current_directory) {
                                    current_directory = p;
                                }
                            }
                            else {
                                current_directory = fs.child_id(current_directory, dirname).unwrap();
                            }
                        }
                    }
                }
                else if is_listing {
                    let parts : Vec<&str> = cmd.split(' ').collect();

                    if parts[0] == "dir" {
                        
                        fs.add(parts[1], 0, current_directory);
                    }
                    else {
                        let size = parts[0].parse::<i32>().unwrap();
                        
                        fs.add(parts[1], size, current_directory);
                    }
                }
            }
        }
    }

    fs
}

pub fn execute() {
    println!("=== puzzle 7 ===");

    // part 1
    let fs_test = parse_input("./data/input07test.txt");    
    let mut candidate_size_test = 0;
    fs_test.find_size_of_candidates(fs_test.root, &mut candidate_size_test);
    assert_eq!(95437, candidate_size_test);

    let fs = parse_input("./data/input07.txt");    
    let mut candidate_size = 0;
    fs.find_size_of_candidates(fs.root, &mut candidate_size);
    println!("candidate size: {}", candidate_size);

    // part 2
    let total_disk_size_test = fs_test.size_of(fs_test.root);
    println!("[test] total size: {}", total_disk_size_test);

    let unused_test = 70000000 - total_disk_size_test;
    println!("[test] unused size: {}", unused_test);

    let necessary_test = 30000000 - unused_test;
    println!("[test] necessary size: {}", necessary_test);

    let mut cleanup_test = 70000000;
    fs_test.find_size_to_cleanup(fs_test.root, necessary_test, &mut cleanup_test);
    assert_eq!(24933642, cleanup_test);


    let total_disk_size = fs.size_of(fs.root);
    println!("total size: {}", total_disk_size);

    let unused = 70000000 - total_disk_size;
    println!("unused size: {}", unused);

    let necessary = 30000000 - unused;
    println!("necessary size: {}", necessary);

    let mut cleanup = 70000000;
    fs.find_size_to_cleanup(fs.root, necessary, &mut cleanup);
    
    println!("cleanup: {}", cleanup);

    println!();
}

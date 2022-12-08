use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use std::mem::swap;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Directory {
    files: HashMap<String, u64>,
    size: u64
}

impl Directory {
    fn new() -> Directory {
        Directory {
            files: HashMap::new(),
            size: 0
        }
    }
    fn add_file(&mut self, file: String, size: u64) {
        if let None = self.files.insert(file, size) {
            self.size += size;
        }
    }

    fn reset(&mut self) {
        self.size = 0;
        self.files.clear();
    }
}

struct FileSystem {
    directories: HashMap<String, Directory>,
    working_path: String,
    working: Directory
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            directories: HashMap::new(),
            working_path: "/".to_owned(),
            working: Directory::new()
        }
    }

    fn resolve_path(working_path: String, part: &str) -> String {
        match part {
            "/" => "/".to_owned(),
            ".." => {
                let new_path = &working_path[0..working_path.rfind("/").unwrap()];
                match new_path {
                    "" => "/".to_owned(),
                    _ => new_path.to_owned()
                }
            },
            _ if working_path == "/" => "/".to_owned() + part,
            _ => working_path.to_owned() + "/" + part,
        }
    }

    fn resolve_from_working(&self, part: &str) -> String {
        FileSystem::resolve_path(self.working_path.to_owned(), part)
    }

    fn change_directory(&mut self, dir: &str) {
        let new_path = self.resolve_from_working(dir);
        if new_path == self.working_path {
            return;
        }
        
        let mut swapped = match self.directories.contains_key(&new_path) {
            true => self.directories.remove(&new_path).unwrap(),
            false => Directory::new()
        };

        swap(&mut self.working, &mut swapped);

        self.directories.insert(self.working_path.to_owned(), swapped);
        self.working_path = new_path;
    }

    fn add_directory(&mut self, dir: &str) {
        let resolved = self.resolve_from_working(dir);

        if !self.directories.contains_key(&resolved) {
            self.directories.insert(resolved, Directory::new());
        }
    }

    fn reset_listing(&mut self) {
        self.working.reset();
    }
    
    fn add_file(&mut self, file: &str, size: &str) {
        let size = size.parse().unwrap();
        self.working.add_file(file.to_owned(), size);
    }

    fn read_output(&mut self, line: String) {
        let tokens = line.split(" ").collect::<Vec<_>>();
        match tokens[0] {
            "$" => match tokens[1] {
                "cd" => self.change_directory(tokens[2]),
                "ls" => self.reset_listing(),
                _ => panic!("Invalid command token: {} in: {}", tokens[1], line)
            },
            "dir" => self.add_directory(tokens[1]),
            _ => self.add_file(tokens[1], tokens[0])
        }
    }

    fn compute_total_sizes(&self) -> HashMap<String, u64> {
        let mut sizes = HashMap::<String, u64>::new();

        self.compute_total_size_for(&self.working_path, &self.working, &mut sizes);

        sizes
    }

    fn is_sub_dir(parent: &String, sub_dir: &String) -> bool {
        if !sub_dir.starts_with(parent) {
            false
        } else {
            match parent == "/" {
                true => sub_dir.rfind("/").unwrap() == 0,
                false => sub_dir.rfind("/").unwrap() == parent.len()
            }
        }
    }

    fn compute_total_size_for(&self, path: &String, dir: &Directory, sizes: &mut HashMap<String, u64>) -> u64 {
        if let Some(size) = sizes.get(path) {
            *size
        } else {
            let size = dir.size + self.directories.iter()
                .filter(|(sub_path, _)| FileSystem::is_sub_dir(path, &sub_path))
                .map(|(sub_path, sub_dir)| self.compute_total_size_for(sub_path, sub_dir, sizes))
                .sum::<u64>();
            
            sizes.insert(path.to_owned(), size);
            size
        }
    }

}

fn part_one(file_name: &str) {
    let mut file_system = FileSystem::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| file_system.read_output(line));
    
    file_system.change_directory("/");

    let sizes = file_system.compute_total_sizes();

    let total = sizes.values()
        .filter(|size| **size <= 100000)
        .sum::<u64>();

    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let mut file_system = FileSystem::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| file_system.read_output(line));
    
    file_system.change_directory("/");
    
    let sizes = file_system.compute_total_sizes();

    let total = 70000000;
    let target = 30000000;
    let free = total - sizes.get("/").unwrap();
    let to_free = target - free;
    
    let min_free_size = sizes.values()
        .filter(|size| **size > to_free)
        .min()
        .unwrap();
    
    println!("Part 2: {}", min_free_size);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}

#[cfg(test)]
mod tests {
    use crate::FileSystem;

    #[test]
    fn empty_file_system() {
        let fs = FileSystem::new();
        assert_eq!(0, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(0, fs.working.files.len());
        assert_eq!(0, fs.working.size);

        let sizes = fs.compute_total_sizes();
        assert_eq!(1, sizes.len());
        assert_eq!(Some(&0), sizes.get("/"));
    }

    #[test]
    fn add_file_to_root() {
        let mut fs = FileSystem::new();

        fs.add_file("a", "1111");
        fs.add_file("b", "2222");

        assert_eq!(0, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(2, fs.working.files.len());
        assert_eq!(3333, fs.working.size);

        let sizes = fs.compute_total_sizes();
        assert_eq!(1, sizes.len());
        assert_eq!(Some(&3333), sizes.get("/"));

        fs.reset_listing();

        assert_eq!(0, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(0, fs.working.files.len());
        assert_eq!(0, fs.working.size);

        let sizes = fs.compute_total_sizes();
        assert_eq!(1, sizes.len());
        assert_eq!(Some(&0), sizes.get("/"));

    }

    #[test]
    fn add_dir_to_root() {
        let mut fs = FileSystem::new();

        fs.add_directory("a");
        fs.add_directory("b");

        assert_eq!(2, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(0, fs.working.files.len());
        assert_eq!(0, fs.working.size);

        let mut keys = fs.directories.keys().collect::<Vec<_>>();
        keys.sort();

        assert_eq!(vec!["/a", "/b"], keys);

        let sizes = fs.compute_total_sizes();
        assert_eq!(3, sizes.len());
        assert_eq!(Some(&0), sizes.get("/"));
        assert_eq!(Some(&0), sizes.get("/a"));
        assert_eq!(Some(&0), sizes.get("/b"));
    }
    
    #[test]
    fn add_files_level_two() {
        let mut fs = FileSystem::new();

        fs.add_directory("a");
        fs.change_directory("a");

        fs.add_file("c", "1111");
        fs.add_file("d", "2222");

        fs.change_directory("..");
        fs.add_directory("b");
        fs.change_directory("b");
        
        fs.add_file("e", "3333");
        fs.add_file("f", "4444");

        fs.change_directory("/");

        assert_eq!(2, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(0, fs.working.files.len());
        assert_eq!(0, fs.working.size);

        fs.add_file("g", "5555");
        fs.add_file("h", "6666");

        let mut keys = fs.directories.keys().collect::<Vec<_>>();
        keys.sort();
        assert_eq!(vec!["/a", "/b"], keys);

        let sizes = fs.compute_total_sizes();
        assert_eq!(3, sizes.len());
        assert_eq!(Some(&23331), sizes.get("/"));
        assert_eq!(Some(&3333), sizes.get("/a"));
        assert_eq!(Some(&7777), sizes.get("/b"));
    }

    
    
    #[test]
    fn add_dir_level_two() {
        let mut fs = FileSystem::new();

        fs.add_directory("a");
        fs.change_directory("a");

        fs.add_file("c", "1111");
        fs.add_file("d", "2222");

        fs.add_directory("b");
        fs.change_directory("b");
        
        fs.add_file("e", "3333");
        fs.add_file("f", "4444");

        fs.change_directory("/");

        assert_eq!(2, fs.directories.len());
        assert_eq!("/", fs.working_path);
        assert_eq!(0, fs.working.files.len());
        assert_eq!(0, fs.working.size);

        fs.add_file("g", "5555");
        fs.add_file("h", "6666");

        assert_eq!(3333, fs.directories.get("/a").unwrap().size);
        assert_eq!(7777, fs.directories.get("/a/b").unwrap().size);

        let mut keys = fs.directories.keys().collect::<Vec<_>>();
        keys.sort();
        assert_eq!(vec!["/a", "/a/b"], keys);

        let sizes = fs.compute_total_sizes();
        assert_eq!(3, sizes.len());
        assert_eq!(Some(&23331), sizes.get("/"));
        assert_eq!(Some(&11110), sizes.get("/a"));
        assert_eq!(Some(&7777), sizes.get("/a/b"));
    }

    
    
}
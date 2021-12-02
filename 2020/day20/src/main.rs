use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::{HashSet, HashMap};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug, Clone)]
struct Edge {
    forward: u16,
    reverse: u16,
    border: bool
}
impl Edge {
    fn new(forward: u16) -> Edge {
        Edge { 
            forward,
            reverse: forward.reverse_bits() >> 6,
            border: false
        }
    }
    fn flip(&mut self) -> Edge {
        let swap = self.forward;
        self.forward = self.reverse;
        self.reverse = swap;
        self.clone()
    }
    fn is(&self, edge: u16) -> bool {
        self.forward == edge || self.reverse == edge
    }
}

#[derive(Debug)]
struct Tile {
    id: u32,
    top: Edge,
    bottom: Edge,
    left: Edge,
    right: Edge,
    borders: usize,
    rotations_from_orig: usize,
    flipped: bool,
    lines: Vec<Vec<char>>
}

impl Tile {
    fn new(id: u32, top: u16, bottom: u16, left: u16, right: u16, lines: Vec<Vec<char>>) -> Tile {
        Tile {
            id, 
            top: Edge::new(top), 
            bottom: Edge::new(bottom), 
            left: Edge::new(left), 
            right: Edge::new(right),
            borders: 0,
            rotations_from_orig: 0,
            flipped: false,
            lines
        }
    }
    fn has_adjacent_border(&self, edge: &u16) -> bool {
        let edges = vec![&self.top, &self.right, &self.bottom, &self.left];
        for i in 0..4 {
            let check = &edges[i];
            if check.is(*edge) {
                return edges[(i + 1) % 4].border || edges[(i + 3) % 4].border;
            }
        }
        false
    }
    fn has_adjacent_edge(&self, one: &u16, two: &u16) -> bool {
        let edges = vec![&self.top, &self.right, &self.bottom, &self.left];
        for i in 0..4 {
            let check = &edges[i];
            if check.is(*one) {
                return edges[(i + 1) % 4].is(*two) || edges[(i + 3) % 4].is(*two);
            }
        }
        false
    }
    fn parse_edge_string(edge: &String) -> u16 {
        Tile::parse_edge_chars(edge.chars().collect::<Vec<char>>())
    }
    fn parse_edge_chars(edge: Vec<char>) -> u16 {
        (0..10).fold(0, |sum, i| {
            let bit = (edge[i] == '#') as u16;
            sum | (bit << i)
        })
    }
    fn get_edges(&self) -> Vec<&Edge> {
        vec![&self.top, &self.bottom, &self.left, &self.right]
    }
    fn get_edges_mut(&mut self) -> Vec<&mut Edge> {
        vec![&mut self.top, &mut self.bottom, &mut self.left, &mut self.right]
    }
    fn rotate(&mut self) {
        let (top, bottom, left, right) = (
            self.left.flip(),
            self.right.flip(),
            self.bottom.clone(),
            self.top.clone()
        );
        self.top = top;
        self.bottom = bottom;
        self.left = left;
        self.right = right;
        self.rotations_from_orig = (self.rotations_from_orig + 1) % 4;
    }
    fn flip(&mut self) {
        let (top, bottom, left, right) = (
            self.top.flip(),
            self.bottom.flip(),
            self.right.clone(),
            self.left.clone()
        );
        self.top = top;
        self.bottom = bottom;
        self.left = left;
        self.right = right;
        self.flipped = !self.flipped;
    }
    fn from_ascii(lines: &mut Input) -> Option<Tile> {
        let id: u32;
        if let Some(Ok(line)) = lines.next() {
            id = line.split(" ")
                .last().unwrap().strip_suffix(':')
                .unwrap().parse::<u32>().unwrap();
        } else {
            return None;
        }
        let tile_strings = lines.flat_map(|line| line.ok())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();

        let top = Tile::parse_edge_string(&tile_strings[0]);
        let bottom = Tile::parse_edge_string(&tile_strings[9]);

        let left = tile_strings.iter()
            .flat_map(|line| line.chars().nth(0))
            .collect::<Vec<char>>();
        let right = tile_strings.iter()
            .flat_map(|line| line.chars().last())
            .collect::<Vec<char>>();
        
        let left = Tile::parse_edge_chars(left);
        let right = Tile::parse_edge_chars(right);

        let lines = tile_strings.iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        Some(Tile::new(id, top, bottom, left, right, lines))
    }
}

struct TileInfo {
    tiles: HashMap<u32, Tile>,
    edges: HashMap<u16, HashSet<u32>>,
}

impl TileInfo {
    fn new() -> TileInfo {
        TileInfo { 
            tiles: HashMap::new(), 
            edges: HashMap::new()
        }
    }
    
    fn add(&mut self, tile: Tile) {
        let id = tile.id;
        self.tiles.insert(id, tile);
        let tile = self.tiles.get(&id).unwrap();

        for side in tile.get_edges() {
            self.edges.entry(side.forward)
                .or_insert_with(|| HashSet::new())
                .insert(id);
            self.edges.entry(side.reverse)
                .or_insert_with(|| HashSet::new())
                .insert(id);
        }
    }

    fn assign_borders(&mut self) -> Vec<u32> {
        let mut corners = Vec::new();
        for tile in self.tiles.values_mut() {
            let mut borders = 0;
            for edge in tile.get_edges_mut() {
                if self.edges.get(&edge.forward).unwrap().len() == 1 {
                    edge.border = true;
                    borders += 1;
                }
            }
            tile.borders = borders;
            if borders == 2 {
                corners.push(tile.id);
            }
        }
        corners
    }

    fn get_tile_with_edge<F>(&self, edge: &u16, borders: usize, placed: &HashSet<u32>, filter: F) -> u32
    where F: Fn(&&Tile) -> bool {
        self.edges.get(&edge).unwrap().iter()
            .filter(|id| !placed.contains(id))
            .map(|id| self.tiles.get(&id).unwrap())
            .filter(|tile| tile.borders == borders)
            .find(filter)
            .expect(
                format!(
                    "No tile found with edge {} and borders {} - all edges: {:?} - tiles: {:?}", 
                    edge, borders, self.edges.get(&edge), 
                    self.edges.get(&edge).unwrap().iter().map(|id| self.tiles.get(&id).unwrap()).collect::<Vec<&Tile>>()
                ).as_str()
            )
            .id
    }

    fn get_tile_with_adjacent_edges<F>(&self, one: &u16, two: &u16, borders: usize, placed: &HashSet<u32>, filter: F) -> u32
    where F: Fn(&&Tile) -> bool {
        self.edges.get(&one).unwrap().iter()
            .filter(|id| !placed.contains(id))
            .map(|id| self.tiles.get(&id).unwrap())
            .filter(|tile| tile.borders == borders)
            .filter(|tile| tile.has_adjacent_edge(one, two))
            .find(filter)
            .expect(
                format!(
                    "No tile found with edges {}, {} and borders {} - edges one: {:?} - edges two: {:?} - tiles one: {:?} - tiles two: {:?}", 
                    one, two, borders, self.edges.get(&one), self.edges.get(&two), 
                    self.edges.get(&one).unwrap().iter().map(|id| self.tiles.get(&id).unwrap()).collect::<Vec<&Tile>>(),
                    self.edges.get(&two).unwrap().iter().map(|id| self.tiles.get(&id).unwrap()).collect::<Vec<&Tile>>()
                ).as_str()
            )
            .id
    }

    fn align_until<F>(&mut self, id: &u32, prev: u16, f: F) -> &mut Tile where
        F: Fn(&Tile) -> bool {
        let tile = self.tiles.get_mut(id).unwrap();

        // first try to rotate to fit
        for _ in 0..4 {
            if f(tile) {
                return tile;
            }
            tile.rotate();
        }

        // now try to flip then rotate to fit
        tile.flip();
        for _ in 0..4 {
            if f(tile) {
                return tile;
            }
            tile.rotate();
        }

        panic!("Rotated and flipped tile but still no match found for {:?} - prev: {}", tile, prev);
    }

}

struct TempImage {
    grid: Vec<Vec<u32>>,
    placed: HashSet<u32>,
    square: usize
}
impl TempImage {
    fn new(square: usize) -> TempImage {
        TempImage {
            grid: vec![Vec::new()],
            placed: HashSet::new(),
            square
        }
    }

    fn add(&mut self, tile: u32) {
        self.grid.last_mut().unwrap().push(tile);
        self.placed.insert(tile);
        assert_ne!(self.grid.last().unwrap().len(), self.square + 1);
    }

    fn next_row(&mut self) {
        self.grid.push(Vec::new());
        assert_ne!(self.grid.len(), self.square + 1);
    }

}

const TILE_SIZE: usize = 8;

struct CompleteImage {
    points: HashSet<(usize, usize)>,
    size: usize
}
impl CompleteImage {

    fn lines_to_points(lines: &Vec<Vec<char>>, start: usize, end: usize) -> HashSet<(usize, usize)> {
        let mut points = HashSet::new();

        for y in start..end {
            let line = &lines[y];
            for x in start..end {
                if line[x] == '#' {
                    points.insert((x - start, y - start));
                }
            }
        }
        points
    }

    fn count_instances(&self, points: &HashSet<(usize, usize)>, search_region: (usize, usize)) -> usize {
        let (x_max, y_max) = search_region;
        let mut instances = 0;
        for x_image in 0..x_max {
            for y_image in 0..y_max {
                instances += points.iter().find(|point| {
                    let (x, y) = point;
                    let new_point = (x_image + x, y_image + y);
                    !self.points.contains(&new_point)
                })
                .is_none() as usize;
            }
        };
        instances
    }
    
    fn to_points(tile: &Tile, x_offset: usize, y_offset: usize) -> HashSet<(usize, usize)> {

        let lines = &tile.lines;
        let square = lines.len() - 1;
        let mut points = CompleteImage::lines_to_points(lines, 1, square);

        if tile.flipped {
            points = CompleteImage::flip(&points, square - 1);
        }

        for _ in 0..tile.rotations_from_orig {
            points = CompleteImage::rotate(&points, square - 1);
        }

        points.into_iter().map(|(x, y)| {
            (x + x_offset, y + y_offset)
        }).collect::<HashSet<(usize, usize)>>()
    }

    fn flip(points: &HashSet<(usize, usize)>, size: usize) -> HashSet<(usize, usize)> {
        let max = size - 1;
        points.into_iter().map(|(x, y)| {
            (max - *x, *y)
        }).collect::<HashSet<(usize, usize)>>()
    }
    //            rotate
    // y            |   y
    //   0 1 2 3 4  =>    0 1 2 3 4 <--x
    // 0       x    =>  0 
    // 1    (3,0)   =>  1 
    // 2            =>  2      (4,3)
    // 3            =>  3         x
    // 4            =>  4 
    fn rotate(points: &HashSet<(usize, usize)>, size: usize) -> HashSet<(usize, usize)> {
        let max = size - 1;
        points.into_iter().map(|(x, y)| {
            let x_new = max - *y;
            let y_new = *x;
            (x_new, y_new)
        }).collect::<HashSet<(usize, usize)>>()
    }
    
    fn new(temp: TempImage, tiles: TileInfo) -> CompleteImage {
        let mut points = HashSet::new();

        let mut y = 0;
        for row in temp.grid {
            let mut x = 0;
            for col in row {
                let tile = tiles.tiles.get(&col).unwrap();
                for point in CompleteImage::to_points(tile, x, y) {
                    points.insert(point);
                }
                x += TILE_SIZE;
            }
            y += TILE_SIZE;
        }

        CompleteImage {
            points,
            size: y
        }
    }

    fn rotate_self(&mut self) {
        self.points = CompleteImage::rotate(&self.points, self.size);
    }

    fn flip_self(&mut self) {
        self.points = CompleteImage::flip(&self.points, self.size);
    }

    fn _print(&self) {
        let max = self.points.iter().fold((0, 0), |max, point| {
            let (x_max, y_max) = max;
            let (x, y) = point;

            (x_max.max(*x), y_max.max(*y))
        });

        let (x_max, y_max) = max;
        for y in 0..(y_max + 1) {
            if y % TILE_SIZE == 0 {
                println!("");
            }
            let mut point = (0, y);
            for x in 0..(x_max + 1) {
                if x % TILE_SIZE == 0 {
                    print!(" ");
                }
                point.0 = x;
                match self.points.contains(&point) {
                    true => print!("#"),
                    false => print!(".")
                }
            }
            println!("");
        }
    }
}

fn calc_required_borders(i: usize, size: usize, last: usize, rest: usize) -> usize {
    match i == size - 1 {
        true => last,
        false => rest
    }
}

fn align_and_add<F>(id: u32, image: &mut TempImage, tiles: &mut TileInfo, below_edges: &mut Vec<u16>, align_until: F, prev: u16) -> u16 
where F: Fn(&Tile) -> bool {
    let tile = tiles.align_until(&id, prev, align_until);
    below_edges.push(tile.bottom.forward);
    image.add(id);
    tile.right.forward
}

fn test_input(file_name: &str) {
    let mut lines = get_file_lines(file_name);
    let mut tiles = TileInfo::new();

    while let Some(tile) = Tile::from_ascii(&mut lines) {
        tiles.add(tile);
    }

    println!("For {}, Have {} tiles", file_name, tiles.tiles.len());

    let corners = tiles.assign_borders();
    assert_eq!(4, corners.len());

    let corner_product = corners.iter()
        .fold(1_u64, |product, id| product * *id as u64);
    
    println!("For {}, corner product is: {}", file_name, corner_product);
    
    let mut image = TempImage::new((tiles.tiles.len() as f64).sqrt() as usize);
    println!("Square is {}", image.square);

    let mut below_edges = vec![];

    // add the first tile (the top left corner)
    let mut prev = align_and_add(corners[0], &mut image, &mut tiles, &mut below_edges, |tile| tile.top.border && tile.left.border, 0);
    
    // the rest of the tiles for the first row
    for i in 1..image.square {
        let num_borders = calc_required_borders(i, image.square, 2, 1);
        let id = tiles.get_tile_with_edge(&prev, num_borders, &image.placed, |tile| tile.has_adjacent_border(&prev));
        prev = align_and_add(id, &mut image, &mut tiles, &mut below_edges, |tile| tile.left.forward == prev, prev);
    }
    
    // do the middle rows
    for _ in 2..image.square {
        image.next_row();

        let mut new_below_edges = vec![];
        // first tile
        {
            let above_edge = below_edges[0];
            let id = tiles.get_tile_with_edge(&above_edge, 1, &image.placed, |tile| tile.has_adjacent_border(&above_edge));
            prev = align_and_add(id, &mut image, &mut tiles, &mut new_below_edges, |tile| tile.left.border && tile.top.forward == above_edge, prev);
        }

        // rest of the tiles
        for i in 1..image.square {
            let num_borders = calc_required_borders(i, image.square, 1, 0);
            let above_edge = below_edges[i];
            let id = tiles.get_tile_with_adjacent_edges(&prev, &above_edge, num_borders, &image.placed, |_| true);
            prev = align_and_add(id, &mut image, &mut tiles, &mut new_below_edges, |tile| tile.left.forward == prev, prev);
        }

        below_edges = new_below_edges;
    }

    // do the last row
    image.next_row();

    let mut _new_below_edges = Vec::new();

    // first tile 
    {
        let above_edge = below_edges[0];
        let id = tiles.get_tile_with_edge(&above_edge, 2, &image.placed, |_| true);
        prev = align_and_add(id, &mut image, &mut tiles, &mut _new_below_edges, |tile| tile.top.forward == above_edge, prev);
    }
    
    // rest of the tiles
    for i in 1..image.square {
        let num_borders = calc_required_borders(i, image.square, 2, 1);
        let above_edge = below_edges[i];
        let id = tiles.get_tile_with_adjacent_edges(&prev, &above_edge, num_borders, &image.placed, |_| true);
        prev = align_and_add(id, &mut image, &mut tiles, &mut _new_below_edges, |tile| tile.left.forward == prev, prev);
    }

    let mut complete = CompleteImage::new(image, tiles);

    let sea_monster_lines = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
    ].iter()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut sea_monster = HashSet::new();
    for y in 0..3 {
        let line = &sea_monster_lines[y];
        for x in 0..line.len() {
            if line[x] == '#' {
                sea_monster.insert((x, y));
            }
        }
    }

    let (width, height) = sea_monster.iter().fold((0, 0), |max, point| {
        (max.0.max(point.0), max.1.max(point.1))
    });
    let search_region = (complete.size - width, complete.size - height);

    fn rotate_and_find_monster(file_name: &str, complete: &mut CompleteImage, sea_monster: &HashSet<(usize, usize)>, search_region: (usize, usize)) -> bool {
        for _ in 0..4 {
            let instances = complete.count_instances(&sea_monster, search_region);
            if instances > 0 {
                let roughness = complete.points.len() - (instances * sea_monster.len());
                println!("For {}, roughness is {} with {} monsters", file_name, roughness, instances);
                return true;
            }
            complete.rotate_self();
        }
        false
    }
    
    if rotate_and_find_monster(file_name, &mut complete, &sea_monster, search_region) {
        return;
    }
    complete.flip_self();
    rotate_and_find_monster(file_name, &mut complete, &sea_monster, search_region);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}

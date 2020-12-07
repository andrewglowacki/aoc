use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contents: Vec<BagContent>
}

#[derive(Debug, Clone)]
struct BagContent {
    count: u32,
    color: String
}

impl Bag {
    fn from_description(description: String) -> Bag {
        let pieces = description.split(" bags contain ").collect::<Vec<&str>>();
        let color = pieces[0].to_owned();

        let contents = pieces[1].split(',')
            .map(|piece| piece.to_owned())
            .flat_map(|content_desc| BagContent::from_description(content_desc))
            .collect::<Vec<BagContent>>();

        Bag {
            color,
            contents
        }
    }

    fn get_num_bags(bag: &Bag, bags_by_color: &HashMap<String, &Bag>) -> u32 {
        bag.contents.iter()
            .map(|content| BagContent::get_num_bags(content, bags_by_color))
            .sum::<u32>() + 1
    }
}

impl BagContent {

    fn from_description(description: String) -> Option<BagContent> {
        let parts = description.trim().split(' ').collect::<Vec<&str>>();
        match parts.len() {
            3 => None,
            4 => Some(BagContent::from_parts(parts)),
            _ => panic!("Invalid number of bag content parts in description: '{}' - {}", description, parts.len())
        }
    }
    
    fn from_parts(parts: Vec<&str>) -> BagContent {
        let count = parts[0].parse::<u32>().unwrap();
        let color = parts[1].to_owned() + " " + parts[2];
        BagContent::new(count, color)
    }

    fn new(count: u32, color: String) -> BagContent {
        BagContent {
            count,
            color
        }
    }
    
    fn get_num_bags(content: &BagContent, bags_by_color: &HashMap<String, &Bag>) -> u32 {
        let bag = bags_by_color.get(&content.color).unwrap();
        content.count * Bag::get_num_bags(bag, bags_by_color)
    }
}

fn get_transitive_map(bags: &Vec<Bag>, bags_by_contents: HashMap::<String, Vec<String>>) -> HashMap::<String, Vec<String>> {
    let mut transitive_bags_by_contents = HashMap::new();
    for bag in bags {
        let color = &bag.color;
        if let Some(parents) = bags_by_contents.get(color) {
            let mut explored = HashSet::<String>::new();
            let mut explore_parents = parents.clone();

            while explore_parents.len() > 0 {
                explore_parents = explore_parents.iter()
                    .filter(|parent| explored.insert((*parent).clone()))
                    .flat_map(|parent| bags_by_contents.get(parent))
                    .flat_map(|parents| parents)
                    .map(|parent| parent.clone())
                    .collect::<Vec<String>>();
            }

            transitive_bags_by_contents.insert(color.clone(), explored.iter()
                .cloned()
                .collect::<Vec<String>>());
            
        } else {
            transitive_bags_by_contents.insert(color.clone(), vec![]);
        }
    }
    transitive_bags_by_contents
}

fn get_bags_by_contents(bags: &Vec<Bag>) -> HashMap::<String, Vec<String>> {
    let mut bags_by_contents = HashMap::<String, Vec<String>>::new();
    bags.iter()
        .flat_map(|bag| {
            bag.contents.iter()
                .map(|content| (content.color.clone(), bag.color.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .for_each(|pair| {
            let (content, container) = pair;
            if let Some(bags) = bags_by_contents.get_mut(&content) {
                bags.push(container);
            } else {
                bags_by_contents.insert(content, vec![container]);
            }
        });
    bags_by_contents
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let bags = BufReader::new(file).lines()
        .flat_map(|line| line.ok())
        .map(|line| Bag::from_description(line))
        .collect::<Vec<Bag>>();

    let bags_by_contents = get_bags_by_contents(&bags);
    let transitive_bags_by_contents = get_transitive_map(&bags, bags_by_contents);
    
    let bags_that_can_hold_shiny_gold = transitive_bags_by_contents.get("shiny gold").unwrap().len();
    
    println!("{} bags can hold at least one shiny gold bag", bags_that_can_hold_shiny_gold);

    let bags_by_color = bags.iter()
        .map(|bag| (bag.color.clone(), bag))
        .collect::<HashMap<String, &Bag>>();
    
    let shiny_bag = bags_by_color.get("shiny gold").unwrap();
    let num_bags_in_shiny_bag = Bag::get_num_bags(&shiny_bag, &bags_by_color);
    println!("Num bags in shiny bag: {}", num_bags_in_shiny_bag - 1);

}

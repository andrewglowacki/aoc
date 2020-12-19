use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

type Rules = HashMap<String, Box<dyn Rule>>;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

trait Rule {
    fn matches(&self, input: VecIter, rule_lookup: &Rules) -> HashSet<VecIter>;
}

#[derive(Clone, Debug)]
struct OrRule {
    rules: Vec<String>
}
impl OrRule {
    fn new(rules: Vec<String>) -> Box<dyn Rule> {
        Box::new(OrRule { rules })
    }
}
impl Rule for OrRule {
    fn matches(&self, input: VecIter, rule_lookup: &Rules) -> HashSet<VecIter> {
        let mut results = HashSet::new();
        for id in self.rules.iter() {
            let rule = rule_lookup.get(id).unwrap();
            for result in rule.matches(input.clone(), rule_lookup) {
                results.insert(result);
            }
        }
        results
    }
}

#[derive(Clone, Debug)]
struct AndRule {
    rules: Vec<String>
}
impl AndRule {
    fn new(rules: Vec<String>) -> Box<dyn Rule> {
        Box::new(AndRule { rules })
    }
}
impl Rule for AndRule {
    fn matches(&self, input: VecIter, rule_lookup: &Rules) -> HashSet<VecIter> {
        let mut results = HashSet::new();
        results.insert(input);
        for id in self.rules.iter() {
            let rule = rule_lookup.get(id).unwrap();
            let mut new_results = HashSet::new();
            for iter in results {
                for result in rule.matches(iter, rule_lookup) {
                    new_results.insert(result);
                }
            }
            results = new_results;
            if results.is_empty() {
                break;
            }
        }
        results
    }
}

#[derive(Clone, Debug)]
struct LiteralRule {
    literal: char
}
impl LiteralRule {
    fn new(literal: char) -> Box<dyn Rule> {
        Box::new(LiteralRule { literal })
    }
}
impl Rule for LiteralRule {
    fn matches(&self, input: VecIter, _: &Rules) -> HashSet<VecIter> {
        let mut input = input;
        let mut set = HashSet::new();
        if let Some(c) = input.next() {
            if self.literal == c {
                set.insert(input);
            }
        }
        set
    }
}

#[derive(Clone, Debug, Eq)]
struct VecIter {
    chars: Vec<char>,
    pos: usize
}
impl PartialEq for VecIter {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Hash for VecIter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
impl VecIter {
    fn next(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            self.pos += 1;
            Some(self.chars[self.pos - 1])
        } else {
            None
        }
    }
    fn new(chars: Vec<char>) -> VecIter {
        VecIter {
            chars,
            pos: 0
        }
    }
    fn is_end(&self) -> bool {
        self.pos >= self.chars.len()
    }
}

fn parse_individual(text: &String, parsed: &mut Rules) {
    let rules = text.split(" ")
        .map(|id| id.to_owned())
        .collect::<Vec<String>>();

    // don't let rules overwriting themselves
    if rules.len() > 1 {
        parsed.insert(text.clone(), AndRule::new(rules));
    }
}

fn parse_or(id: &String, text: &String, parsed: &mut Rules) {
    if text.chars().nth(0).unwrap() == '"' {
        let literal = text.chars().nth(1).unwrap();
        parsed.insert(id.clone(), LiteralRule::new(literal));
    } else {
        let rules = text.split(" | ")
            .map(|text| text.to_owned())
            .collect::<Vec<String>>();
        
        let to_parse = rules.iter().
            filter(|text| !parsed.contains_key(*text))
            .map(|text| text.to_owned())
            .collect::<Vec<String>>();

        parsed.insert(id.clone(), OrRule::new(rules));

        to_parse.iter().for_each(|text| parse_individual(text, parsed));
    }
}

fn parse_rules(rule_strings: &HashMap<String, String>) -> Rules {
    let mut parsed = Rules::new();
    for (id, text) in rule_strings {
        parse_or(id, text, &mut parsed);
    }
    parsed
}

fn test_input(file_name: &str, part_one: bool) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    let mut rule_strings = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            (parts[0].to_owned(), parts[1].to_owned())
        })
        .collect::<HashMap<String, String>>();
    
    if !part_one {
        rule_strings.insert(String::from("8"), String::from("42 | 42 8"));
        rule_strings.insert(String::from("11"), String::from("42 31 | 42 11 31"));
    }
    
    let rules = parse_rules(&rule_strings);

    let zero = rules.get("0").unwrap();

    let matches = lines.map(|line| line.chars().collect::<Vec<char>>())
        .filter(|line| {
            let iter = VecIter::new(line.clone());
            let matches = zero.matches(iter, &rules)
                .iter()
                .filter(|iter| iter.is_end())
                .count();
            matches > 0
        })
        .count();
    
    println!("For {}, matches is {}", file_name, matches);
}

fn main() {
    test_input("sample.txt", true);
    test_input("input.txt", true);
    test_input("sample.txt", false);
    test_input("input.txt", false);
}

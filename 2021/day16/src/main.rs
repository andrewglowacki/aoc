use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;
type Packets = Vec<Box<dyn Packet>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct PacketString {
    nibbles: Vec<u8>,
    str_pos: usize,
    nibble_pos: usize
}

impl PacketString {
    fn parse(mut line: String) -> PacketString {
        line.push('0'); // make parsing easier
        let nibbles = line.chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect::<Vec<u8>>();
        PacketString {
            nibbles,
            str_pos: 0,
            nibble_pos: 0
        }
    }

    fn bit_pos(&self) -> u32 {
        (self.str_pos as u32 * 4) + self.nibble_pos as u32
    }

    fn next_bit(&mut self) -> bool {
        let nibble = self.nibbles[self.str_pos] >> (3 - self.nibble_pos);
        let is_set = (nibble & 0x01) == 1;
        self.nibble_pos += 1;
        if self.nibble_pos >= 4 {
            self.str_pos += 1;
            self.nibble_pos = 0;
        }
        is_set
    }
    
    fn read_number(&mut self, mut bit_len: u8) -> u64 {
        let mut number: u64 = 0;

        let mut nibble = self.nibbles[self.str_pos];
        while bit_len > 0 {
            number = (number << 1) | ((nibble >> (3 - self.nibble_pos)) & 0x01) as u64;
            self.nibble_pos += 1;
            bit_len -= 1;
            if self.nibble_pos >= 4 {
                self.str_pos += 1;
                self.nibble_pos = 0;
                nibble = self.nibbles[self.str_pos];
            }
        }
        number
    }
}

fn parse(file_name: &str) -> Box<dyn Packet> {
    let line = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    let mut packet_str = PacketString::parse(line);
    let packet = parse_nibbles(&mut packet_str);
    packet
}

fn parse_nibbles(packet_str: &mut PacketString) -> Box<dyn Packet> {
    let version = packet_str.read_number(3) as u32;
    let type_id = packet_str.read_number(3);

    match type_id {
        4 => {
            // type_id 4 means it's a literal
            let mut number = 0;
            while packet_str.next_bit() {
                number = (number << 4) | packet_str.read_number(4);
            }
            number = (number << 4) | packet_str.read_number(4);
            let literal = Literal { version, number, sub_packets: Vec::with_capacity(0) };
            return Box::new(literal);
        }
        _ => {
            // all others are operators
            let mode = packet_str.next_bit();
            let mut sub_packets = Vec::new();
            match mode {
                false => {
                    // fixed packets length mode
                    let packet_len = packet_str.read_number(15);
                    let end_pos = packet_str.bit_pos() + packet_len as u32;
                    while packet_str.bit_pos() < end_pos {
                        let packet = parse_nibbles(packet_str);
                        sub_packets.push(packet);
                    }
                },
                true => {
                    // number of sub-packets mode
                    let count = packet_str.read_number(11);
                    for _ in 0..count {
                        let packet = parse_nibbles(packet_str);
                        sub_packets.push(packet);
                    }
                }
            }
            let label = match type_id {
                0 => "sum",
                1 => "prod",
                2 => "min",
                3 => "max",
                5 => "greater",
                6 => "less",
                7 => "equal",
                _ => panic!("Invalid operator type: {}", type_id)
            };
            let label = label.to_owned();

            let op: Box<dyn Fn(Vec<u64>) -> u64> = match type_id {
                0 => Box::new(|results| results.iter().sum::<u64>()),
                1 => Box::new(|results| results.iter().fold(1, |product,num| product * *num)),
                2 => Box::new(|results| *results.iter().min().unwrap()),
                3 => Box::new(|results| *results.iter().max().unwrap()),
                5 => Box::new(|results| {
                    match results[0] > results[1] {
                        true => 1,
                        false => 0
                    }
                }),
                6 => Box::new(|results| {
                    match results[0] < results[1] {
                        true => 1,
                        false => 0
                    }
                }),
                7 => Box::new(|results| {
                    match results[0] == results[1] {
                        true => 1,
                        false => 0
                    }
                }),
                _ => panic!("Invalid operator type: {}", type_id)
            };
            let operator = Operator { version, op, label, sub_packets };
            return Box::new(operator);
        }
    }
}

trait Packet {
    fn get_version(&self) -> u32;
    fn get_sub_packets(&self) -> &Packets;
    fn execute(&self) -> u64;
    fn sum_versions(&self) -> u32;
    fn print(&self, indent: String);
}

struct Operator {
    version: u32,
    op: Box<dyn Fn(Vec<u64>) -> u64>,
    sub_packets: Packets,
    label: String
}

impl Packet for Operator {
    fn get_version(&self) -> u32 {
        self.version
    }
    fn execute(&self) -> u64 { 
        let results = self.sub_packets.iter()
            .map(|packet| packet.execute())
            .collect::<Vec<u64>>();
        (*self.op)(results)
    }

    fn get_sub_packets(&self) -> &Packets { 
        &self.sub_packets
    }
    
    fn sum_versions(&self) -> u32 {
        self.version + self.sub_packets.iter()
            .map(|packet| packet.sum_versions())
            .sum::<u32>()
    }
    fn print(&self, indent: String) {
        println!("{}{} of = {}", indent, self.label, self.execute());
        println!("{}[", indent);
        self.sub_packets.iter()
            .for_each(|packet| packet.print(indent.clone() + "  "));
        println!("{}]", indent);
    }
}

struct Literal {
    version: u32,
    number: u64,
    sub_packets: Vec<Box<dyn Packet>>
}

impl Packet for Literal {
    fn get_version(&self) -> u32 {
        self.version
    }
    fn execute(&self) -> u64 { 
        self.number
    }

    fn get_sub_packets(&self) -> &Packets { 
        &self.sub_packets 
    }
    
    fn sum_versions(&self) -> u32 {
        self.version
    }
    fn print(&self, indent: String) {
        println!("{}{}", indent, self.number);
    }
}

fn part_one(file_name: &str) {
    let packet = parse(file_name);

    let sum = packet.sum_versions();
    
    println!("Part 1: {}", sum);
}

fn part_two(file_name: &str) {
    let packet = parse(file_name);
    
    // packet.print("".to_owned());

    let result = packet.execute();
    
    println!("Part 2: {}", result);
}

fn samples(file_name: &str) {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| (line.clone(), PacketString::parse(line.clone())))
        .for_each(|(line, mut packet_str)| {
            let packet = parse_nibbles(&mut packet_str);
            let sum = packet.execute();
            println!("Sample {} = {}", line, sum);
        });
}

fn main() {
    samples("sample.txt");
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}

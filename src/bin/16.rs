#[derive(Debug)]
enum Packet {
    LiteralPacket(LiteralPacket),
    OperatorPacket(OperatorPacket),
}

#[derive(Debug)]
struct LiteralPacket {
    version: i32,
    type_id: i32,
    value: i32,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct OperatorPacket {
    version: i32,
    type_id: i32,
    length_type_id: i32,
    length: i32,
    subpackets: Vec<Packet>,
    start: usize,
    end: usize,
}

fn hex_to_binary(c: char) -> Option<String>{
    match c {
        '0' => Some("0000".to_string()),
        '1' => Some("0001".to_string()),
        '2' => Some("0010".to_string()),
        '3' => Some("0011".to_string()),
        '4' => Some("0100".to_string()),
        '5' => Some("0101".to_string()),
        '6' => Some("0110".to_string()),
        '7' => Some("0111".to_string()),
        '8' => Some("1000".to_string()),
        '9' => Some("1001".to_string()),
        'A' => Some("1010".to_string()),
        'B' => Some("1011".to_string()),
        'C' => Some("1100".to_string()),
        'D' => Some("1101".to_string()),
        'E' => Some("1110".to_string()),
        'F' => Some("1111".to_string()),
        _ => None,
    }
}

fn binary_to_decimal(s:String) -> i32 {
    let mut decimal = 0;
    for i in 0..s.len() {
        if &s[i..i+1] == "1" {decimal += 2_i32.pow(s.len() as u32 - i as u32)}

    }
    return decimal
}

fn parse_input(input: &str) -> String {
    let packet_vec: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let packet_string: String = packet_vec[0].chars().map(|x| hex_to_binary(x).unwrap()).collect();
    return packet_string
}

fn parse_packet(packet_string: &str, location: usize) -> Packet {
    let type_id = i32::from_str_radix(&packet_string[location+3..location+6], 2).unwrap();
    match type_id {
        4 => Packet::LiteralPacket(parse_literal_packet(packet_string, location)),
        _ => Packet::OperatorPacket(parse_operator_packet(packet_string, location)),
    }
}

fn parse_literal_packet(packet_string: &str, location: usize) -> LiteralPacket {
    let version = i32::from_str_radix(&packet_string[location..location+3], 2).unwrap();
    let type_id = i32::from_str_radix(&packet_string[location+3..location+6], 2).unwrap();
    let mut new_location: usize = location + 6;
    let mut value: String = "".to_string();
    while packet_string[new_location..new_location+1] == "1".to_string() {
        value.push_str(&packet_string[new_location+1..new_location+5]);
        new_location += 5;
    }
    value.push_str(&packet_string[new_location+1..new_location+5]);
    new_location += 5;
    let value = i32::from_str_radix(&value, 2).unwrap();
    LiteralPacket{version, type_id, value, start: location, end: new_location}
}

fn parse_operator_packet(packet_string: &str, location: usize) -> OperatorPacket {

    let version = i32::from_str_radix(&packet_string[location..location+3], 2).unwrap();
    let type_id = i32::from_str_radix(&packet_string[location+3..location+6], 2).unwrap();
    let length_type_id = i32::from_str_radix(&packet_string[location+6..location+7], 2).unwrap();
    let mut length = 0;
    let mut subpackets_location = 0;
    let mut subpackets = vec![];
    let mut new_location = 0;

    if length_type_id == 0 {
        length = i32::from_str_radix(&packet_string[location+7..location+22], 2).unwrap();
        subpackets_location = location + 22;
        new_location = subpackets_location;
        while new_location < subpackets_location + length as usize {
            let new_packet = parse_packet(packet_string, new_location);
            new_location = match new_packet {
                Packet::LiteralPacket(ref p) => p.end,
                Packet::OperatorPacket(ref p) => p.end,
            };
            dbg!(new_location);
            subpackets.push(new_packet);
    }

    } else if length_type_id == 1 {
        length = i32::from_str_radix(&packet_string[location+7..location+18], 2).unwrap();
        dbg!(length);
        subpackets_location = location + 18;
        new_location = subpackets_location;
        for i in 0..length {
            let new_packet = parse_packet(packet_string, new_location);
            new_location = match new_packet {
                Packet::LiteralPacket(ref p) => p.end,
                Packet::OperatorPacket(ref p) => p.end,
            };
            subpackets.push(new_packet);
        }
    }


    OperatorPacket{version, type_id, length_type_id, length, subpackets,
    start: location, end: new_location}
}


pub fn part_one(input: &str) -> Option<u32> {
    let packet_string = parse_input(input);
    let packet = parse_packet(&packet_string, 0);
    dbg!(packet);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(16));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}

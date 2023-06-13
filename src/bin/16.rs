#[derive(Debug)]
enum Packet {
    LiteralPacket(LiteralPacket),
    OperatorPacket(OperatorPacket),
}

#[derive(Debug)]
struct LiteralPacket {
    version: i32,
    type_id: i32,
    value: i64,
    end: usize,
}

#[derive(Debug)]
struct OperatorPacket {
    version: i32,
    type_id: i32,
    subpackets: Vec<Packet>,
    end: usize,
}

fn hex_to_binary(c: char) -> Option<String> {
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

fn parse_input(input: &str) -> String {
    let packet_vec: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let packet_string: String = packet_vec[0]
        .chars()
        .map(|x| hex_to_binary(x).unwrap())
        .collect();
    packet_string
}

fn parse_packet(packet_string: &str, location: usize) -> Packet {
    let type_id = i32::from_str_radix(&packet_string[location + 3..location + 6], 2).unwrap();
    match type_id {
        4 => Packet::LiteralPacket(parse_literal_packet(packet_string, location)),
        _ => Packet::OperatorPacket(parse_operator_packet(packet_string, location)),
    }
}

fn parse_literal_packet(packet_string: &str, location: usize) -> LiteralPacket {
    let version = i32::from_str_radix(&packet_string[location..location + 3], 2).unwrap();
    let type_id = i32::from_str_radix(&packet_string[location + 3..location + 6], 2).unwrap();
    let mut new_location: usize = location + 6;
    let mut value: String = "".to_string();
    while packet_string[new_location..new_location + 1] == *"1".to_string() {
        value.push_str(&packet_string[new_location + 1..new_location + 5]);
        new_location += 5;
    }
    value.push_str(&packet_string[new_location + 1..new_location + 5]);
    new_location += 5;
    let value = i64::from_str_radix(&value, 2).unwrap();
    LiteralPacket {
        version,
        type_id,
        value,
        end: new_location,
    }
}

fn parse_operator_packet(packet_string: &str, location: usize) -> OperatorPacket {
    let version = i32::from_str_radix(&packet_string[location..location + 3], 2).unwrap();
    let type_id = i32::from_str_radix(&packet_string[location + 3..location + 6], 2).unwrap();
    let length_type_id =
        i32::from_str_radix(&packet_string[location + 6..location + 7], 2).unwrap();
    let mut subpackets = vec![];
    let mut new_location = 0;

    if length_type_id == 0 {
        let length = i32::from_str_radix(&packet_string[location + 7..location + 22], 2).unwrap();
        let subpackets_location = location + 22;
        new_location = subpackets_location;
        while new_location < subpackets_location + length as usize {
            let new_packet = parse_packet(packet_string, new_location);
            new_location = match new_packet {
                Packet::LiteralPacket(ref p) => p.end,
                Packet::OperatorPacket(ref p) => p.end,
            };
            subpackets.push(new_packet);
        }
    } else if length_type_id == 1 {
        let length = i32::from_str_radix(&packet_string[location + 7..location + 18], 2).unwrap();
        let subpackets_location = location + 18;
        new_location = subpackets_location;
        for _i in 0..length {
            let new_packet = parse_packet(packet_string, new_location);
            new_location = match new_packet {
                Packet::LiteralPacket(ref p) => p.end,
                Packet::OperatorPacket(ref p) => p.end,
            };
            subpackets.push(new_packet);
        }
    }

    OperatorPacket {
        version,
        type_id,
        subpackets,
        end: new_location,
    }
}

fn evaluate_packet(packet: &Packet) -> i64 {
    let type_id = match packet {
        Packet::LiteralPacket(ref p) => p.type_id,
        Packet::OperatorPacket(ref p) => p.type_id,
    };

    if type_id == 4 {
        return match packet {
            Packet::LiteralPacket(ref p) => p.value,
            Packet::OperatorPacket(_) => 0,
        };
    };

    let values = match packet {
        Packet::LiteralPacket(_) => vec![],
        Packet::OperatorPacket(ref p) => p.subpackets.iter().map(evaluate_packet).collect(),
    };

    return match type_id {
        0 => values.iter().sum::<i64>(),
        1 => values.iter().product::<i64>(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => (values[0] > values[1]) as i64,
        6 => (values[0] < values[1]) as i64,
        7 => (values[0] == values[1]) as i64,
        _ => 0,
    };
}

fn sum_versions(packet: &Packet) -> i32 {
    match packet {
        Packet::LiteralPacket(ref p) => p.version,
        Packet::OperatorPacket(ref p) => {
            p.version + p.subpackets.iter().map(sum_versions).sum::<i32>()
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let packet_string = parse_input(input);
    let packet = parse_packet(&packet_string, 0);
    Some(sum_versions(&packet).try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let packet_string = parse_input(input);
    let packet = parse_packet(&packet_string, 0);
    let evaluation = evaluate_packet(&packet);
    Some(evaluation.try_into().unwrap())
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
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(54));
    }
}

use std::collections::VecDeque;

type Bit = u8;

#[derive(Debug, PartialEq)]
struct Packet {
    version: u32,
    kind: u32,
    payload: PacketPayload,
}

#[derive(Debug, PartialEq)]
enum PacketPayload {
    Literal(u32),
    Operation(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(hex: &str) -> Packet {
        Packet::from(&parse_bitvec(hex))
    }
}

impl From<&Vec<Bit>> for Packet {
    fn from(bits: &Vec<Bit>) -> Packet {
        read_packet(bits).0
    }
}

fn read_packet(bits: &[Bit]) -> (Packet, usize) {
    let version = load_u32(&bits[0..3]);
    let kind = load_u32(&bits[3..6]);
    let mut bits_read = 6;

    let payload = if kind == 4 {
        let (payload, read) = parse_literal(&bits[6..]);
        bits_read += read;
        payload
    } else if bits[bits_read] == 0 {
        bits_read += 1;
        let length = load_u32(&bits[bits_read..bits_read + 15]) as usize;
        bits_read += 15;

        let end = bits_read + length;

        let mut sub_packets = vec![];
        while bits_read < end {
            let (p, r) = read_packet(&bits[bits_read..]);
            bits_read += r;
            sub_packets.push(p);
        }

        PacketPayload::Operation(sub_packets)
    } else {
        bits_read += 1;
        let count = load_u32(&bits[bits_read..bits_read + 11]) as usize;
        bits_read += 11;

        let mut sub_packets = Vec::with_capacity(count);
        for _ in 0..count {
            let (p, r) = read_packet(&bits[bits_read..]);
            bits_read += r;
            sub_packets.push(p);
        }

        PacketPayload::Operation(sub_packets)
    };

    (
        Packet {
            version,
            kind,
            payload,
        },
        bits_read,
    )
}

fn parse_literal(bits: &[Bit]) -> (PacketPayload, usize) {
    let mut offset = 0;
    let mut value = 0;

    loop {
        value <<= 4;
        value += load_u32(&bits[offset + 1..offset + 5]);
        offset += 5;

        if bits[offset - 5] == 0 {
            break;
        }
    }
    (PacketPayload::Literal(value), offset)
}

fn load_u32(bits: &[Bit]) -> u32 {
    bits.iter().fold(0, |x, &b| b as u32 + (x << 1))
}

fn parse_bitvec(hex: &str) -> Vec<Bit> {
    hex.trim().chars().fold(Default::default(), |mut v, c| {
        match c {
            '0' => v.extend([0, 0, 0, 0]),
            '1' => v.extend([0, 0, 0, 1]),
            '2' => v.extend([0, 0, 1, 0]),
            '3' => v.extend([0, 0, 1, 1]),
            '4' => v.extend([0, 1, 0, 0]),
            '5' => v.extend([0, 1, 0, 1]),
            '6' => v.extend([0, 1, 1, 0]),
            '7' => v.extend([0, 1, 1, 1]),
            '8' => v.extend([1, 0, 0, 0]),
            '9' => v.extend([1, 0, 0, 1]),
            'A' => v.extend([1, 0, 1, 0]),
            'B' => v.extend([1, 0, 1, 1]),
            'C' => v.extend([1, 1, 0, 0]),
            'D' => v.extend([1, 1, 0, 1]),
            'E' => v.extend([1, 1, 1, 0]),
            'F' => v.extend([1, 1, 1, 1]),
            _ => panic!("'{}' is not hex", c),
        };
        v
    })
}

fn parse_input(s: &str) -> Packet {
    s.into()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let mut queue = VecDeque::new();
    queue.push_front(input);

    let mut output = 0;
    while let Some(p) = queue.pop_back() {
        output += p.version;

        if let PacketPayload::Operation(ps) = p.payload {
            queue.extend(ps);
        }
    }

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day16.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day16 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_16_1_1, part_one, 16, 1, 1);
    test_example!(example_16_1_2, part_one, 16, 1, 2);
    test_example!(example_16_1_3, part_one, 16, 1, 3);
    test_example!(example_16_1_4, part_one, 16, 1, 4);
    //test_example!(example_16_2_1, part_two, 16, 2, 1);

    #[test]
    fn parse_bitvec_from_hex() {
        assert_eq!(
            parse_bitvec("D2FE28"),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn parse_packet_literal() {
        assert_eq!(
            Packet::from("D2FE28"),
            Packet {
                version: 6,
                kind: 4,
                payload: PacketPayload::Literal(2021),
            },
        );
    }

    #[test]
    fn parse_packet() {
        assert_eq!(
            read_packet(&parse_bitvec("38006F45291200")),
            (Packet {
                version: 1,
                kind: 6,
                payload: PacketPayload::Operation(vec![
                    Packet {
                        version: 6,
                        kind: 4,
                        payload: PacketPayload::Literal(10),
                    },
                    Packet {
                        version: 2,
                        kind: 4,
                        payload: PacketPayload::Literal(20),
                    }
                ]),
            }, 49)
        );

        assert_eq!(
            read_packet(&parse_bitvec("EE00D40C823060")),
            (Packet {
                version: 7,
                kind: 3,
                payload: PacketPayload::Operation(vec![
                    Packet {
                        version: 2,
                        kind: 4,
                        payload: PacketPayload::Literal(1),
                    },
                    Packet {
                        version: 4,
                        kind: 4,
                        payload: PacketPayload::Literal(2),
                    },
                    Packet {
                        version: 1,
                        kind: 4,
                        payload: PacketPayload::Literal(3),
                    }
                ]),
            }, 51)
        );
    }
}

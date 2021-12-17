use common::io::stdin;

type Version = u8;

#[derive(Debug, Clone, Copy)]
enum OperatorMode {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(Version, u64),
    Operator(Version, OperatorMode, Vec<Packet>),
}

const OPERATORS: [Option<OperatorMode>; 8] = [
    Some(OperatorMode::Sum),
    Some(OperatorMode::Product),
    Some(OperatorMode::Minimum),
    Some(OperatorMode::Maximum),
    None,
    Some(OperatorMode::GreaterThan),
    Some(OperatorMode::LessThan),
    Some(OperatorMode::Equal),
];

fn bit_at(bytes: &[u8], bit: usize) -> u8 {
    let i = bit / 8;
    let j = bit % 8;

    1 & (bytes[i] >> (7 - j))
}

fn read_n_bits(bytes: &[u8], start_bit: usize, n: usize) -> u64 {
    (0..n).fold(0, |acc, v| {
        acc | (bit_at(bytes, start_bit + v) as u64) << (n - 1 - v)
    })
}

fn parse(bytes: &[u8], start_bit: usize, packets: &mut Vec<Packet>) -> usize {
    let mut bits_consumed = 0;

    let version = (bit_at(bytes, start_bit) << 2)
        | (bit_at(bytes, start_bit + 1) << 1)
        | bit_at(bytes, start_bit + 2);
    let type_id = (bit_at(bytes, start_bit + 3) << 2)
        | (bit_at(bytes, start_bit + 4) << 1)
        | bit_at(bytes, start_bit + 5);

    bits_consumed += 6;

    match type_id {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            if let Some(operator_mode) = OPERATORS[type_id as usize] {
                let mode_bit = bit_at(bytes, start_bit + 6);
                bits_consumed += 1;

                let mut sub_packets = vec![];
                match mode_bit {
                    0 => {
                        let num_bits = read_n_bits(bytes, start_bit + bits_consumed, 15) as usize;

                        bits_consumed += 15;

                        let mut sub_packet_bits_consumed = 0;

                        while sub_packet_bits_consumed < num_bits {
                            sub_packet_bits_consumed += parse(
                                bytes,
                                start_bit + bits_consumed + sub_packet_bits_consumed,
                                &mut sub_packets,
                            );
                        }

                        bits_consumed += num_bits;
                    }
                    1 => {
                        let mut value = read_n_bits(bytes, start_bit + bits_consumed, 11) as usize;

                        bits_consumed += 11;
                        while 0 < value {
                            bits_consumed +=
                                parse(bytes, start_bit + bits_consumed, &mut sub_packets);
                            value -= 1;
                        }
                    }
                    _ => {}
                }
                let packet = Packet::Operator(version, operator_mode, sub_packets);
                packets.push(packet);
            } else {
                let mut val = 0u64;
                let mut should_continue = true;
                while should_continue {
                    let test_bit = bit_at(bytes, start_bit + bits_consumed);
                    val <<= 4;

                    val |= read_n_bits(bytes, start_bit + bits_consumed + 1, 4);

                    bits_consumed += 5;

                    should_continue = test_bit == 1;
                }
                let packet = Packet::Literal(version, val);
                packets.push(packet);
            }
        }
        _ => unreachable!(),
    }

    bits_consumed
}

fn eval(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, x) => *x,
        Packet::Operator(_, OperatorMode::Sum, sub_packets) => {
            sub_packets.iter().map(|packet| eval(packet)).sum::<u64>()
        }
        Packet::Operator(_, OperatorMode::Product, sub_packets) => sub_packets
            .iter()
            .map(|packet| eval(packet))
            .product::<u64>(),
        Packet::Operator(_, OperatorMode::Minimum, sub_packets) => {
            sub_packets.iter().map(|packet| eval(packet)).min().unwrap()
        }
        Packet::Operator(_, OperatorMode::Maximum, sub_packets) => {
            sub_packets.iter().map(|packet| eval(packet)).max().unwrap()
        }
        Packet::Operator(_, OperatorMode::GreaterThan, sub_packets) => {
            let a = &sub_packets[0];
            let b = &sub_packets[1];
            if eval(b) < eval(a) {
                1
            } else {
                0
            }
        }
        Packet::Operator(_, OperatorMode::LessThan, sub_packets) => {
            let a = &sub_packets[0];
            let b = &sub_packets[1];
            if eval(a) < eval(b) {
                1
            } else {
                0
            }
        }
        Packet::Operator(_, OperatorMode::Equal, sub_packets) => {
            let a = &sub_packets[0];
            let b = &sub_packets[1];
            if eval(a) == eval(b) {
                1
            } else {
                0
            }
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(v, _) => *v as u64,
        Packet::Operator(v, _, sub_packets) => {
            *v as u64
                + sub_packets
                    .iter()
                    .map(|packet| version_sum(packet))
                    .sum::<u64>()
        }
    }
}

fn main() {
    let input = stdin::read_line_with(|line| {
        line.trim()
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let hex_byte = match std::str::from_utf8(chunk) {
                    Ok(hex_byte) => hex_byte,
                    _ => return None,
                };
                u8::from_str_radix(hex_byte, 16).ok()
            })
            .collect::<Option<Vec<_>>>()
    })
    .unwrap();

    let mut packets = vec![];

    parse(&input, 0, &mut packets);

    // Part 1
    println!("{:?}", version_sum(&packets[0]));

    // Part 2
    println!("{:?}", eval(&packets[0]));
}

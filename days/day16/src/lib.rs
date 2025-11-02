use std::{str, vec};

#[derive(PartialEq, Debug)]
enum LenghtTypeId {
    TotalLengthInBits(i32),
    TotalSubPackets(i32),
}

#[derive(PartialEq, Debug)]
enum SubPacket {
    LITERAL(i32),
    OPERATOR(LenghtTypeId, Vec<SubPacket>),
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: i32,
    subpackets: Vec<SubPacket>,
}

fn decode_hex_to_binary(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).expect("Invalid input token")))
        //.inspect(|f| println!("{}", f))
        .collect()
}

fn decode_bits_to_packet(input: &str) -> Option<Packet> {
    let version = i32::from_str_radix(&input[0..3], 2).ok()?;

    Some(Packet {
        version,
        subpackets: decode_bits_to_sub_packets(&input[3..]),
    })
}

fn decode_bits_to_sub_packets(input: &str) -> Vec<SubPacket> {
    let mut result = vec![];

    let mut offset = 0usize;
    while let Some((sub_packet, len)) =
        match i32::from_str_radix(&input[offset..offset + 3], 2).unwrap() {
            4 => decode_bits_to_literal(&input[offset + 3..]),
            _ => decode_bits_to_operator(&input[offset + 3..]),
        }
    {
        offset += 3 + len;

        result.push(sub_packet);
    }

    result
}

fn decode_bits_to_literal(input: &str) -> Option<(SubPacket, usize)> {
    let mut ss = String::new();

    let mut len = 0usize;

    for i in (0..input.len()).step_by(5) {
        let token = &input[i..(i + 5) % input.len()];

        ss += &token.chars().skip(1).take(4).collect::<String>();

        len += token.len();

        if token.chars().nth(0) == Some('0') {
            break;
        }
    }

    Some((SubPacket::LITERAL(i32::from_str_radix(&ss, 2).ok()?), len))
}

fn decode_bits_to_operator(input: &str) -> Option<(SubPacket, usize)> {
    let id = if input.chars().nth(0)? == '0' {
        LenghtTypeId::TotalLengthInBits(i32::from_str_radix(&input[1..16], 2).ok()?)
    } else {
        LenghtTypeId::TotalSubPackets(i32::from_str_radix(&input[1..12], 2).ok()?)
    };

    match id {
        LenghtTypeId::TotalLengthInBits(_) => todo!(),
        LenghtTypeId::TotalSubPackets(_) => todo!(),
        _ => None,
    }

    for i in (0..input.len()).step_by(5) {}
    // if input.chars().nth(0)? == '0' {
    //     Some(SubPacket::OPERATOR(
    //         ,
    //         decode_bits_to_sub_packets(&input[16..]),
    //     ))
    // } else if input.chars().nth(0)? == '1' {
    //     Some(SubPacket::OPERATOR(
    //         ),
    //         decode_bits_to_sub_packets(&input[12..]),
    //     ))
    // } else {
    //     None
    // }
}

fn part1() {
    println!("Day 16 part 1 : ");
}

fn part2() {
    println!("Day 16 part 2 : ");
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::{decode_bits_to_packet, decode_hex_to_binary, LenghtTypeId, Packet, SubPacket};

    #[test]
    fn part1_test1() {
        let bits = decode_hex_to_binary("D2FE28");
        assert_eq!(bits, "110100101111111000101000");

        let packet = decode_bits_to_packet(&bits);

        assert_eq!(
            packet,
            Some(Packet {
                version: 6,
                subpackets: vec![SubPacket::LITERAL(2021)]
            })
        );
    }

    #[test]
    fn part1_test2() {
        let bits = decode_hex_to_binary("38006F45291200");
        assert_eq!(
            bits,
            "00111000000000000110111101000101001010010001001000000000"
        );

        let packet = decode_bits_to_packet(&bits);

        assert_eq!(
            packet,
            Some(Packet {
                version: 1,
                subpackets: vec![SubPacket::OPERATOR(
                    LenghtTypeId::TotalLengthInBits(27),
                    vec![SubPacket::LITERAL(10), SubPacket::LITERAL(20),]
                )]
            })
        );
    }
}

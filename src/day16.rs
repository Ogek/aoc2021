use std::convert::TryFrom;
use std::str::from_utf8;

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Literal(u32),
    Operator(Vec<Packet>),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u32,
    payload: Payload,
}

fn parse(input: &str) -> Packet {
    let binary = input
        .chars()
        .map(|c| c.to_digit(16).expect("Cannot parse hex digit"))
        .map(|hex| format!("{:04b}", hex))
        .collect::<String>();

    Packet::try_from(binary.as_str()).expect("Cannot parse main packet")
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet.payload {
        Payload::Literal(_) => packet.version,
        Payload::Operator(ref sub_packets) => {
            packet.version + sub_packets.iter().map(sum_versions).sum::<u32>()
        }
    }
}

pub fn p1(input: &str) -> u32 {
    sum_versions(&parse(input))
}

pub fn p2(input: &str) -> usize {
    0
}

fn parse_literal(binary: &str) -> Payload {
    let mut bin_idx = String::new();
    let mut last: bool = false;

    'out: for chunk in binary.as_bytes().chunks(5) {
        if chunk[0] != b'1' {
            last = true;
        }
        bin_idx.push_str(from_utf8(&chunk[1..]).unwrap());
        if last {
            break 'out;
        }
    }

    Payload::Literal(u32::from_str_radix(bin_idx.as_str(), 2).unwrap())
}

fn parse_operator(binary: &str) -> Payload {
    let sub_packets = match binary.chars().next().unwrap() {
        '0' => {
            let length = usize::from_str_radix(&binary[1..16], 2).unwrap();
            let binary = &binary[16..];

            let mut got = 0;
            let mut sub_packets = Vec::new();

            while got != length {
                if length.checked_sub(got + 22).is_none() {
                    sub_packets.push(Packet::try_from(&binary[got..length]).unwrap());
                    got = length;
                } else {
                    sub_packets.push(Packet::try_from(&binary[got..got + 11]).unwrap());
                    got += 11;
                }
            }

            sub_packets
        }
        '1' => {
            let n_sub_packets = u32::from_str_radix(&binary[1..12], 2).unwrap();

            binary[12..]
                .as_bytes()
                .chunks_exact(11)
                .take(n_sub_packets as usize)
                .map(|chunk| Packet::try_from(from_utf8(chunk).unwrap()).unwrap())
                .collect()
        }
        _ => unreachable!(),
    };

    Payload::Operator(sub_packets)
}

impl<'a> TryFrom<&'a str> for Packet {
    type Error = &'a str;

    fn try_from(binary: &str) -> Result<Self, Self::Error> {
        //println!("Parsing {:?}", binary);
        let version = u32::from_str_radix(&binary[0..3], 2).unwrap();
        let typ = u32::from_str_radix(&binary[3..6], 2).unwrap();

        let payload = match typ {
            4 => parse_literal(&binary[6..]),
            _ => parse_operator(&binary[6..]),
        };

        let packet = Packet { version, payload };

        Ok(packet)
    }
}

#[test]
fn test_literal_parsing() {
    assert_eq!(
        parse("D2FE28"),
        Packet {
            version: 6,
            payload: Payload::Literal(2021)
        }
    )
}

#[test]
fn test_operator_0_parsing() {
    assert_eq!(
        parse("38006F45291200"),
        Packet {
            version: 1,
            payload: Payload::Operator(vec![
                Packet {
                    version: 6,
                    payload: Payload::Literal(10)
                },
                Packet {
                    version: 2,
                    payload: Payload::Literal(20)
                }
            ])
        }
    )
}

#[test]
fn test_operator_1_parsing() {
    assert_eq!(
        parse("EE00D40C823060"),
        Packet {
            version: 7,
            payload: Payload::Operator(vec![
                Packet {
                    version: 2,
                    payload: Payload::Literal(1)
                },
                Packet {
                    version: 4,
                    payload: Payload::Literal(2)
                },
                Packet {
                    version: 1,
                    payload: Payload::Literal(3)
                },
            ])
        }
    )
}

#[test]
fn test_p1() {
    assert_eq!(p1("8A004A801A8002F478"), 16);
    // assert_eq!(p1("620080001611562C8802118E34"), 12);
    // assert_eq!(p1("C0015000016115A2E0802F182340"), 23);
    // assert_eq!(p1("A0016C880162017C3686B18A3D4780"), 31);
}

// #[test]
// fn test_p2() {
//     assert_eq!(p2("D2FE28"), 315);
// }

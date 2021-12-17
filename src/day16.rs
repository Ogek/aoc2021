type Bits<'a> = dyn Iterator<Item = char> + 'a;

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u32,
    typ: u8,
    payload: Payload,
}

fn parse(input: &str) -> Packet {
    let mut bits = input
        .chars()
        .map(|c| c.to_digit(16).expect("Cannot parse hex digit"))
        .flat_map(|hex| format!("{:04b}", hex).chars().collect::<Vec<char>>());

    Packet::parse(&mut bits)
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet.payload {
        Payload::Literal(_) => packet.version,
        Payload::Operator(ref sub_packets) => {
            packet.version + sub_packets.iter().map(sum_versions).sum::<u32>()
        }
    }
}

fn eval(packet: &Packet) -> u64 {
    match packet.payload {
        Payload::Literal(n) => n,
        Payload::Operator(ref sub_packets) => match packet.typ {
            0 => sub_packets.iter().map(eval).sum(),
            1 => sub_packets.iter().map(eval).product(),
            2 => sub_packets.iter().map(eval).min().unwrap(),
            3 => sub_packets.iter().map(eval).max().unwrap(),
            5 => (eval(&sub_packets[0]) > eval(&sub_packets[1])) as u64,
            6 => (eval(&sub_packets[0]) < eval(&sub_packets[1])) as u64,
            7 => (eval(&sub_packets[0]) == eval(&sub_packets[1])) as u64,
            _ => unreachable!(),
        },
    }
}

pub fn p1(input: &str) -> u32 {
    sum_versions(&parse(input))
}

pub fn p2(input: &str) -> u64 {
    eval(&parse(input))
}

fn parse_literal(bits: &mut Bits) -> Payload {
    let mut literal_bits = Vec::new();

    loop {
        let last = bits.next().unwrap() == '0';
        literal_bits.extend(bits.take(4));
        if last {
            return Payload::Literal(
                u64::from_str_radix(&literal_bits.iter().collect::<String>(), 2).unwrap(),
            );
        }
    }
}

fn parse_operator(bits: &mut Bits) -> Payload {
    let sub_packets = match bits.next().unwrap() {
        '0' => {
            let length = usize::from_str_radix(&bits.take(15).collect::<String>(), 2).unwrap();
            let mut sub_packets = Vec::new();

            let mut subpacket_bits = bits.take(length).peekable();
            while let Some(_) = subpacket_bits.peek() {
                sub_packets.push(Packet::parse(&mut subpacket_bits));
            }

            sub_packets
        }
        '1' => {
            let n_sub_packets = u32::from_str_radix(&bits.take(11).collect::<String>(), 2).unwrap();

            (0..n_sub_packets).map(|_| Packet::parse(bits)).collect()
        }
        _ => unreachable!(),
    };

    Payload::Operator(sub_packets)
}

impl Packet {
    fn parse(bits: &mut Bits) -> Self {
        let version = u32::from_str_radix(&bits.take(3).collect::<String>(), 2).unwrap();
        let typ = u8::from_str_radix(&bits.take(3).collect::<String>(), 2).unwrap();

        let payload = match typ {
            4 => parse_literal(bits),
            _ => parse_operator(bits),
        };

        let packet = Packet {
            version,
            typ,
            payload,
        };

        packet
    }
}

#[test]
fn test_literal_parsing() {
    assert_eq!(
        parse("D2FE28"),
        Packet {
            version: 6,
            typ: 4,
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
            typ: 6,
            payload: Payload::Operator(vec![
                Packet {
                    version: 6,
                    typ: 4,
                    payload: Payload::Literal(10)
                },
                Packet {
                    version: 2,
                    typ: 4,
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
            typ: 3,
            payload: Payload::Operator(vec![
                Packet {
                    version: 2,
                    typ: 4,
                    payload: Payload::Literal(1)
                },
                Packet {
                    version: 4,
                    typ: 4,
                    payload: Payload::Literal(2)
                },
                Packet {
                    version: 1,
                    typ: 4,
                    payload: Payload::Literal(3)
                },
            ])
        }
    )
}

#[test]
fn test_p1_1() {
    assert_eq!(p1("8A004A801A8002F478"), 16);
    assert_eq!(p1("620080001611562C8802118E34"), 12);
    assert_eq!(p1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(p1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_p2() {
    assert_eq!(p2("C200B40A82"), 3);
    assert_eq!(p2("04005AC33890"), 54);
    assert_eq!(p2("880086C3E88112"), 7);
    assert_eq!(p2("CE00C43D881120"), 9);
    assert_eq!(p2("D8005AC2A8F0"), 1);
    assert_eq!(p2("F600BC2D8F"), 0);
    assert_eq!(p2("9C005AC2F8F0"), 0);
    assert_eq!(p2("9C0141080250320F1802104A08"), 1);
}

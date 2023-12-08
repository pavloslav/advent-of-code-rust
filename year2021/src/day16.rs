pub enum PacketContent {
    Literal(usize),
    Operator(Vec<Packet>),
}

pub struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

const LITERAL_ID: u8 = 4;
const BIT_LENGTH_SIZE: usize = 15;
const PACKETS_COUNT_SIZE: usize = 11;

#[derive(Debug)]
enum Error {
    UnexpectedEndOfStream,
}

fn get_number(input: &[u8]) -> usize {
    let mut number = 0_usize;
    for &bit in input {
        number = number << 1 | bit as usize;
    }
    number
}

fn parse_packet(input: &[u8]) -> anyhow::Result<(Packet, &[u8]), Error> {
    if input.len() < 7 {
        return Err(Error::UnexpectedEndOfStream);
    }
    let version = get_number(&input[..3]) as u8;
    let type_id = get_number(&input[3..6]) as u8;
    if type_id == LITERAL_ID {
        let mut number = 0;
        let mut ptr = 6;
        loop {
            let next_part = get_number(&input[ptr..ptr + 5]);
            ptr += 5;
            number = (number << 4) | (next_part & 0xF);
            if next_part & 0x10 == 0 {
                break;
            }
        }
        Ok((
            Packet {
                version,
                type_id,
                content: PacketContent::Literal(number),
            },
            &input[ptr..],
        ))
    } else {
        let length_type_id = get_number(&input[6..7]);
        match length_type_id {
            0 => {
                let header_end = 7 + BIT_LENGTH_SIZE;
                let bit_length = get_number(&input[7..header_end]);
                let mut subpackets = vec![];
                let mut own_input = &input[header_end..header_end + bit_length];
                while !own_input.is_empty() {
                    let (packet, rest) = parse_packet(own_input)?;
                    subpackets.push(packet);
                    own_input = rest;
                }
                Ok((
                    Packet {
                        version,
                        type_id,
                        content: PacketContent::Operator(subpackets),
                    },
                    &input[header_end + bit_length..],
                ))
            }
            1 => {
                let header_end = 7 + PACKETS_COUNT_SIZE;
                let packet_count = get_number(&input[7..header_end]);
                let mut subpackets = Vec::with_capacity(packet_count);
                let mut own_input = &input[header_end..];
                for _ in 0..packet_count {
                    let (packet, rest) = parse_packet(own_input)?;
                    subpackets.push(packet);
                    own_input = rest;
                }
                Ok((
                    Packet {
                        version,
                        type_id,
                        content: PacketContent::Operator(subpackets),
                    },
                    own_input,
                ))
            }
            _ => panic!("Wrong length type id: {}", length_type_id),
        }
    }
}

pub fn parse_input(input: &str) -> Packet {
    let bit_stream: Vec<u8> = input
        .chars()
        .flat_map(|hex_digit| {
            (0..4)
                .rev()
                .map(move |i| ((hex_digit.to_digit(16).unwrap() >> i) & 1) as u8)
        })
        .collect();
    parse_packet(&bit_stream).unwrap().0
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.content {
                PacketContent::Literal(_) => 0,
                PacketContent::Operator(subpackets) => {
                    subpackets.iter().map(|p| p.version_sum()).sum()
                }
            }
    }
    fn calculate(&self) -> usize {
        match (&self.content, self.type_id) {
            (PacketContent::Operator(subpackets), 0) => {
                subpackets.iter().map(|p| p.calculate()).sum()
            }
            (PacketContent::Operator(subpackets), 1) => {
                subpackets.iter().map(|p| p.calculate()).product()
            }
            (PacketContent::Operator(subpackets), 2) => {
                subpackets.iter().map(|p| p.calculate()).min().unwrap()
            }
            (PacketContent::Operator(subpackets), 3) => {
                subpackets.iter().map(|p| p.calculate()).max().unwrap()
            }
            (&PacketContent::Literal(value), 4) => value,
            (PacketContent::Operator(subpackets), 5) => {
                usize::from(subpackets[0].calculate() > subpackets[1].calculate())
            }
            (PacketContent::Operator(subpackets), 6) => {
                usize::from(subpackets[0].calculate() < subpackets[1].calculate())
            }
            (PacketContent::Operator(subpackets), 7) => {
                usize::from(subpackets[0].calculate() == subpackets[1].calculate())
            }
            _ => panic!("Wrong type_id: {}", self.type_id),
        }
    }
}

pub fn task1(input: &Packet) -> usize {
    input.version_sum()
}

pub fn task2(input: &Packet) -> usize {
    input.calculate()
}

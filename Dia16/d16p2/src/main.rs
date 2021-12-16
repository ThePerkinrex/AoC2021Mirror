#[derive(Debug)]
pub struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator(OperatorKind, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
enum OperatorKind {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl OperatorKind {
    fn from(type_id: u8) -> Self {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => unreachable!(),
        }
    }

    fn apply(&self, data: &[Packet]) -> u64 {
        // println!("{:?} {:?} {}", self, data, data.len());
        match self {
            OperatorKind::Sum => data.iter().map(Packet::res).sum(),
            OperatorKind::Product => data.iter().map(Packet::res).product(),
            OperatorKind::Min => data.iter().map(Packet::res).min().unwrap(),
            OperatorKind::Max => data.iter().map(Packet::res).max().unwrap(),
            OperatorKind::Gt => {
                if data[0].res() > data[1].res() {
                    1
                } else {
                    0
                }
            }
            OperatorKind::Lt => {
                if data[0].res() < data[1].res() {
                    1
                } else {
                    0
                }
            }
            OperatorKind::Eq => {
                if data[0].res() == data[1].res() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Packet {
    fn parse(data: &[u8], start_bit: usize) -> (usize, Self) {
        let mut index = start_bit;
        println!("Data left: {}", data[start_bit..].iter().map(|x| (*x + '0' as u8) as char).collect::<String>());
        
        println!("Reading version");
        let version = read_num(data, &mut index, 3) as u8;
        
        println!("Reading type_id");
        let type_id = read_num(data, &mut index, 3) as u8;
        println!("Version: {}, Type ID: {}", version, type_id);
        if type_id == 4 {
            let mut last_bit = data[index];
            index += 1;
            let mut sections = Vec::new();
            while last_bit == 1 {
                
                println!("Reading section");
                let n = read_num(data, &mut index, 4);
                sections.push(n);
                last_bit = data[index];
                index += 1;
            }
            
            println!("Reading last_section");
            let n = read_num(data, &mut index, 4);
            sections.push(n);
            let mut n: u64 = 0;
            for (i, sect) in sections.into_iter().rev().enumerate() {
                n += (sect as u64) << (4 * i as u64);
            }

            (
                index,
                Self {
                    version,
                    data: PacketData::Literal(n),
                },
            )
        } else {
            let mode = data[index];
            let mut packets = Vec::new();
            index += 1;
            if mode == 0 {
                println!("Reading n_bits");
                let n_bits = read_num(data, &mut index, 15) as usize;
                println!("{} bits to read from {} left", n_bits, data.len() - index);
                let start = index;
                while index < start+n_bits {
                    println!("@>>>>>>>>>>>> {}", index);
                    let (new_idx, packet) = Packet::parse(data, index);
                    println!("@<<<<<<<<<<<<");
                    index = new_idx;
                    packets.push(packet);
                }
            } else {
                println!("Reading n_packets");
                let n_packets = read_num(data, &mut index, 11);
                for _ in 0..n_packets {
                    println!(">>>>>>>>>>>> {}", index);
                    let (new_idx, packet) = Packet::parse(data, index);
                    println!("<<<<<<<<<<<<");

                    index = new_idx;
                    packets.push(packet);
                }
            }
            (
                index,
                Self {
                    version,
                    data: PacketData::Operator(OperatorKind::from(type_id), packets),
                },
            )
        }
    }

    fn sum(&self) -> usize {
        self.version as usize
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Operator(_, v) => v.iter().map(Packet::sum).sum(),
            }
    }

    fn res(&self) -> u64 {
        match &self.data {
            PacketData::Literal(x) => *x,
            PacketData::Operator(op, data) => op.apply(data),
        }
    }
}

fn read_num(data: &[u8], index: &mut usize, bit_size: u8) -> u16 {
    println!("read_num: {} bits to read from {} left", bit_size, data.len() - *index);
    let mut r = 0;
    for i in (0..bit_size).rev() {
        r |= (data[*index] as u16) << i as u16;
        *index += 1;
    }
    r
}

fn main() {
    let data = include_str!("../../input.txt")
        .trim()
        .chars()
        .flat_map(|x| {
            let mut s = String::new();
            s.push(x);
            let r = u8::from_str_radix(&s, 16).unwrap();
            format!("{:<04b}", r)
                .chars()
                .map(|x| (x as u8) - ('0' as u8))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>();
    println!();
    let (_, parsed) = Packet::parse(&data, 0);
    println!("{:?}", parsed);
    println!("Sum: {}", parsed.sum());
    println!("Res: {}", parsed.res());
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

impl Packet {
    fn parse(data: &[u8], start_byte: usize, start_bit: u8) -> Option<(usize, Self)> {
        let mut read = start_byte;
        let mut bit_pos = start_bit;

        println!("start: {} {}", read, bit_pos as usize);
        let mut version = 0;
        for i in (0..3).rev() {
            bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                read += 1;
                7
            });
            version |= ((data.get(read)? & (1 << bit_pos)) >> bit_pos) << i;
        }

        let mut type_id = 0;
        for i in (0..3).rev() {
            bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                read += 1;
                7
            });
            type_id |= ((data.get(read)? & (1 << bit_pos)) >> bit_pos) << i;
        }
        println!("Version: {}, Type ID: {}", version, type_id);
        bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
            read += 1;
            7
        });
        if type_id == 4 {
            let mut sections = Vec::new();
            loop {
                let mut current_section = 0;
                let prev_read = read;
                let prev_bit_pos = bit_pos;
                for i in (0..4).rev() {
                    bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                        read += 1;
                        7
                    });
                    println!("{} Current: {} {}", i, read, bit_pos as usize);
                    current_section |= ((data.get(read)? & (1 << bit_pos)) >> bit_pos) << i;
                }
                sections.push(current_section);
                bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                    read += 1;
                    7
                });
                // println!("{}", (data[prev_read-1] & (1 << prev_bit_pos)) >> prev_bit_pos);
                if (data[prev_read] & (1 << prev_bit_pos)) == 0 {
                    break;
                }
            }
            let mut res = 0u64;
            for (i, v) in sections.into_iter().rev().enumerate() {
                res |= (v as u64) << ((i * 4) as u64);
            }
            println!("read: {} {}", read, bit_pos as usize);
            Some((
                (read * 8 + bit_pos as usize),
                Self {
                    version,
                    data: PacketData::Literal(res),
                },
            ))
        } else {
            let mode = data.get(read)? & (1 << bit_pos);
            if mode == 0 {
                let mut len_packets: u16 = 0;
                for i in (0..15).rev() {
                    bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                        read += 1;
                        7
                    });
                    print!(
                        "{}",
                        (((data.get(read)? & (1 << bit_pos)) >> bit_pos) as u16)
                    );
                    len_packets |=
                        (((data.get(read)? & (1 << bit_pos)) >> bit_pos) as u16) << i as u16;
                }

                println!();
                println!("Have to parse {} bits", len_packets);
                let start = read * 8 + bit_pos as usize;
                let mut packets = Vec::new();
                while (read * 8 + bit_pos as usize - start) < len_packets as usize {
                    if let Some((bits_read, packet)) = Packet::parse(data, read, bit_pos) {
                        read = bits_read / 8;
                        bit_pos = (bits_read % 8) as u8;
                        // println!("Read {} {}: {:?}", read, bit_pos, packet);
                        bit_pos += 1;
                        if bit_pos == 8 {
                            bit_pos = 0;
                            read -= 1;
                        }
                        println!(
                            "Current idx: {} < {} = {}",
                            (read * 8 + bit_pos as usize) - start,
                            len_packets,
                            (read * 8 + bit_pos as usize - start) < len_packets as usize
                        );
                        packets.push(packet);
                    } else {
                        break;
                    }
                }
                bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                    read += 1;
                    7
                });
                Some((
                    (read * 8 + bit_pos as usize),
                    Self {
                        version,
                        data: PacketData::Operator(mode, packets),
                    },
                ))
            } else {
                let mut n_packets: u16 = 0;
                for i in (0..11).rev() {
                    bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                        read += 1;
                        7
                    });
                    n_packets |=
                        (((data.get(read)? & (1 << bit_pos)) >> bit_pos) as u16) << i as u16;
                }
                println!("Have to parse {} packets", n_packets);
                let mut packets = Vec::new();
                for _ in 0..n_packets {
                    if let Some((bits_read, packet)) = Packet::parse(data, read, bit_pos) {
                        read = bits_read / 8;
                        bit_pos = (bits_read % 8) as u8;
                        println!("Read {} {}: {:?}", read, bit_pos, packet);
                        bit_pos += 1;
                        if bit_pos == 8 {
                            bit_pos = 0;
                            read -= 1;
                        }
                        packets.push(packet);
                    } else {
                        // panic!("No more bytes to read")
                    }
                }
                bit_pos = bit_pos.checked_sub(1).unwrap_or_else(|| {
                    read += 1;
                    7
                });
                Some((
                    (read * 8 + bit_pos as usize),
                    Self {
                        version,
                        data: PacketData::Operator(mode, packets),
                    },
                ))
            }
        }
    }

    fn sum(&self) -> usize {
        self.version as usize
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Operator(_, v) => v.iter().map(Packet::sum).sum(),
            }
    }
}

fn main() {
    let data = include_str!("../../input.txt")
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|x| {
            let mut s = String::new();
            s.push(x[0]);
            s.push(x[1]);
            let r = u8::from_str_radix(&s, 16).unwrap();
            print!("{:<08b}", r);
            r
        })
        .collect::<Vec<_>>();
    println!();
    println!("{:<03b} {:<08b}", data[3] & 0b111, data[4]);
    let (_, parsed) = Packet::parse(&data, 0, 8).unwrap();
    println!("{:?}", parsed);
    println!("{:?}", parsed.sum());
}

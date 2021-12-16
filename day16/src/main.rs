#[derive(Debug,Clone,PartialEq)]
enum Body {
    Operator(OperatorBody),
    Literal(u64),
}

#[derive(Debug,Clone)]
struct Packet {
    version:u32,
    type_id:u32,
    body:Body,
}

#[derive(Debug,Clone)]
struct OperatorBody {
    packets:Vec<Packet>,
}

impl PartialEq for OperatorBody {
    fn eq(&self, other: &Self) -> bool {
        self.packets.len() == other.packets.len()
    }
}

impl Body {
    fn literal(binary:&str) -> (Body, usize) {
        let mut offset = 0;
        let mut value_bin = String::new();
        loop {
            value_bin.push_str(&binary[offset+1..offset+5]);

            if &binary[offset..=offset] == "1" {
                offset += 5;
            } else {
                offset += 5;
                break;
            }
        }

        let value = u64::from_str_radix(&value_bin[..], 2).unwrap(); 

        (Body::Literal(value), offset-1)
    }

    fn operator(binary:&str) -> (Body,usize) {
        let length_type_id = &binary[0..=0] == "1";

        let length;
        let mut packets = vec![];
        let mut offset: usize;
        
        if length_type_id {
            length = u32::from_str_radix(&binary[1..=11], 2).unwrap() as usize;
            offset = 12;

            for _ in 0..length {
                if let Some((packet,size)) = Packet::from_binary(&binary[offset..]) {
                    packets.push(packet);
                    offset += size+1;
                } else {
                    break;
                }
            }
        } else {
            length = u32::from_str_radix(&binary[1..=15], 2).unwrap() as usize;

            offset = 16;
            while offset < length+16 {
                if let Some((packet, size)) = Packet::from_binary(&binary[offset..]) {
                    packets.push(packet.clone());
                    offset+=size+1;
                } else {
                    break;
                }
            }
        }

        (Body::Operator(OperatorBody{
            packets
        }), offset-1)
        
    }
}

impl Packet {
    fn from(raw:&str) -> Option<(Packet,usize)> {
        let mut binary = String::new();
        for ch in raw.chars() {
            binary.push_str(&format!("{:04b}", ch.to_digit(16).unwrap()));
        }

        Packet::from_binary(&binary)
    }

    fn from_binary(binary:&str) -> Option<(Packet,usize)> {
        let hdr = 6;
        let version = u32::from_str_radix(&binary[0..3], 2).unwrap();
        let type_id = u32::from_str_radix(&binary[3..6], 2).unwrap();

        if type_id == 4 {
            let (body,size) = Body::literal(&binary[hdr..]);
            return Some((Packet {
                type_id,
                version,
                body
            }, size+hdr))
        }

        let (body,size) = Body::operator(&binary[hdr..]);
        return Some((Packet{
            type_id,
            version,
            body,
        }, size+hdr))
    }

    #[allow(dead_code)]
    fn literal(version:u32,value:u64) -> Packet {
        Packet{
            version,
            type_id: 4,
            body: Body::Literal(value),
        }
    }
}

fn main() {
    let version_sum = decode(input());
    println!("answer1: {}", version_sum.unwrap());

    let calc = calc(input());
    println!("answer2: {}", calc);
}

fn calc(s:&str) -> u64 {
    let (packet,_) = Packet::from(s).unwrap();
    calculate(packet)
}

fn calculate(packet:Packet) -> u64 {
    match packet.body {
        Body::Literal(value) => {
            return value;
        }
        Body::Operator(op) => {
            match packet.type_id {
                0 => return op.packets.into_iter().map(|x| calculate(x)).sum(),
                1 => return op.packets.into_iter().map(|x| calculate(x)).fold(1, |tot,x| tot*x),
                2 => return op.packets.into_iter().map(|x| calculate(x)).min().unwrap(),
                3 => return op.packets.into_iter().map(|x| calculate(x)).max().unwrap(),
                5 => return if calculate(op.packets[0].clone()) > calculate(op.packets[1].clone()) { 1 } else { 0 },
                6 => return if calculate(op.packets[0].clone()) < calculate(op.packets[1].clone()) { 1 } else { 0 },
                7 => return if calculate(op.packets[0].clone()) == calculate(op.packets[1].clone()) { 1 } else { 0 },
                _ => panic!("unknown operation {}", packet.type_id),
            }
            
        }
    }
}

fn decode(packet:&str) -> Option<u32> {
    if let Some((packet,_)) = Packet::from(packet) {
       return Some(visit(packet))
    }
    
    None
}

fn visit(packet:Packet) -> u32 {
    match packet.body {
        Body::Literal(_) => {
            return packet.version
        }
        Body::Operator(op) => {
            return packet.version + op.packets.into_iter().map(|x| visit(x)).sum::<u32>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let (packet,size) = Packet::from("D2FE28").unwrap();

        assert_eq!(size, 20);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.body, Body::Literal(2021));
    }

    #[test]
    fn test_parse_operator() {
        let (packet,size) = Packet::from("38006F45291200").unwrap();

        assert_eq!(size, 49);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.body, Body::Operator(
            OperatorBody{
                packets:vec![
                   Packet::literal(1, 10), 
                   Packet::literal(1, 20), 
                ],
            }
        ));
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("8A004A801A8002F478"), Some(16));
        assert_eq!(decode("38006F45291200"), Some(9));
        assert_eq!(decode("620080001611562C8802118E34"), Some(12));
        assert_eq!(decode("C0015000016115A2E0802F182340"), Some(23));
        assert_eq!(decode("A0016C880162017C3686B18A3D4780"), Some(31));
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calc("C200B40A82"), 3);
        assert_eq!(calc("04005AC33890"), 54);
        assert_eq!(calc("880086C3E88112"), 7);
        assert_eq!(calc("CE00C43D881120"), 9);
        assert_eq!(calc("D8005AC2A8F0"), 1);
        assert_eq!(calc("F600BC2D8F"), 0);
        assert_eq!(calc("9C005AC2F8F0"), 0);
        assert_eq!(calc("9C0141080250320F1802104A08"), 1);
    }
}

fn input() -> &'static str {
"620D79802F60098803B10E20C3C1007A2EC4C84136F0600BCB8AD0066E200CC7D89D0C4401F87104E094FEA82B0726613C6B692400E14A305802D112239802125FB69FF0015095B9D4ADCEE5B6782005301762200628012E006B80162007B01060A0051801E200528014002A118016802003801E2006100460400C1A001AB3DED1A00063D0E25771189394253A6B2671908020394359B6799529E69600A6A6EB5C2D4C4D764F7F8263805531AA5FE8D3AE33BEC6AB148968D7BFEF2FBD204CA3980250A3C01591EF94E5FF6A2698027A0094599AA471F299EA4FBC9E47277149C35C88E4E3B30043B315B675B6B9FBCCEC0017991D690A5A412E011CA8BC08979FD665298B6445402F97089792D48CF589E00A56FFFDA3EF12CBD24FA200C9002190AE3AC293007A0A41784A600C42485F0E6089805D0CE517E3C493DC900180213D1C5F1988D6802D346F33C840A0804CB9FE1CE006E6000844528570A40010E86B09A32200107321A20164F66BAB5244929AD0FCBC65AF3B4893C9D7C46401A64BA4E00437232D6774D6DEA51CE4DA88041DF0042467DCD28B133BE73C733D8CD703EE005CADF7D15200F32C0129EC4E7EB4605D28A52F2C762BEA010C8B94239AAF3C5523CB271802F3CB12EAC0002FC6B8F2600ACBD15780337939531EAD32B5272A63D5A657880353B005A73744F97D3F4AE277A7DA8803C4989DDBA802459D82BCF7E5CC5ED6242013427A167FC00D500010F8F119A1A8803F0C62DC7D200CAA7E1BC40C7401794C766BB3C58A00845691ADEF875894400C0CFA7CD86CF8F98027600ACA12495BF6FFEF20691ADE96692013E27A3DE197802E00085C6E8F30600010882B18A25880352D6D5712AE97E194E4F71D279803000084C688A71F440188FB0FA2A8803D0AE31C1D200DE25F3AAC7F1BA35802B3BE6D9DF369802F1CB401393F2249F918800829A1B40088A54F25330B134950E0"
}

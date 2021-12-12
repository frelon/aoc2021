use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Pos(u32,u32);

impl Pos {
    fn north(&self) -> Pos {
        Pos(self.0,self.1-1)
    }
    fn northwest(&self) -> Pos {
        Pos(self.0-1,self.1-1)
    }
    fn northeast(&self) -> Pos {
        Pos(self.0+1,self.1-1)
    }
    fn south(&self) -> Pos {
        Pos(self.0,self.1+1)
    }
    fn southwest(&self) -> Pos {
        Pos(self.0-1,self.1+1)
    }
    fn southeast(&self) -> Pos {
        Pos(self.0+1,self.1+1)
    }
    fn west(&self) -> Pos {
        Pos(self.0-1,self.1)
    }
    fn east(&self) -> Pos {
        Pos(self.0+1,self.1)
    }
    fn neighbors(&self) -> Vec<Pos> {
        vec![self.northwest(),self.north(),self.northeast(),self.west(),self.east(),self.southwest(),self.south(),self.southeast()]
    }
}

fn main() {
    let input = input();
    let mut map = HashMap::new();

    let mut y = 1;
    for line in input {
        let mut x = 1;
        for ch in line.chars() {
            map.insert(Pos(x,y), ch.to_digit(10).unwrap()); 
            x += 1;
        }
        y += 1;
    }

    let mut step = 0;
    'outer: for s in 1..=10000 {
        step = s;

        for energy in map.values_mut() {
           *energy += 1;
        }

        let flashing:Vec<Pos> = map.iter().filter(|(_,energy)| *energy > &9).map(|(pos,_)| Pos(pos.0,pos.1)).collect();

        for pos in flashing {
            flash(&mut map, &pos);
        }

        for (_, e) in &map {
            if *e != 0 {
                continue 'outer;
            }
        }

        step = s;
        break;
    }

    println!("answer: {}", step)
}

fn flash(m:&mut HashMap<Pos,u32>, curr:&Pos) -> u32 {
    let mut total = 0;
    if let Some(energy) = m.get_mut(curr) {
        if *energy == 0 {
            return 0;
        }

        total += 1;
        *energy = 0;

        for neighbor in curr.neighbors() {
            if let Some(neighbor_energy) = m.get_mut(&neighbor) {
                if *neighbor_energy == 0 {
                    continue;
                }

                *neighbor_energy += 1;

                if *neighbor_energy > 9 {
                    total += flash(m,&neighbor);
                }
            }
        }

    }

    return total
}

fn input() -> Vec<&'static str> {
    vec![
"5433566276",
"6376253438",
"8458636316",
"6253254525",
"7211137138",
"1411526532",
"5788761424",
"8677841514",
"1622331631",
"5876712227",
    ]
}

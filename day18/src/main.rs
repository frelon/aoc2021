use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
struct Sfn {
    numbers: Vec<(u32,u32)>,
}

impl FromStr for Sfn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depth:u32 = 0;
        let mut nums = vec![];

        for ch in s.chars() {
            match ch {
                '[' => {depth+=1},
                ']' => {depth-=1},
                ',' => {},
                d => { nums.push((depth, d.to_digit(10).expect("wanted digit")))},
            }
        }

        Ok(Sfn{numbers:nums})
    }
}

impl Sfn {
    fn magnitude(mut self) -> u32 {
        let mut i = 0;
        while self.numbers.len() != 1 {
            if self.numbers[i].0 != self.numbers[i+1].0 {
                i+=1;
                continue;
            }

            self.numbers[i].0 -= 1;
            self.numbers[i].1 = 3*self.numbers[i].1 + 2*self.numbers[i+1].1;

            self.numbers.remove(i+1);
            i = 0;
        }

        self.numbers[0].1
    }

    fn add(&mut self, sfn:Sfn) {
        self.numbers.extend(sfn.numbers);

        for (d,_) in self.numbers.iter_mut() {
            *d += 1;
        }

        self.reduce();
    }

    fn reduce(&mut self) {
        loop {
            if let Some(()) = self.explode() {
                continue;
            }

            if let Some(()) = self.split() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) -> Option<()> {
        let mut i = 0;

        while i < self.numbers.len()-1 {
            if self.numbers[i].0 != self.numbers[i+1].0 || self.numbers[i].0 <= 4 {
                i += 1;
                continue;
            }

            if i >= 1 {
                self.numbers[i-1].1 += self.numbers[i].1
            }

            if i < self.numbers.len()-2 {
                self.numbers[i+2].1 += self.numbers[i+1].1
            }

            self.numbers[i].1 = 0;
            self.numbers[i].0 -= 1;
            self.numbers.remove(i+1);

            return Some(())
        }

        None
    }

    fn split(&mut self) -> Option<()> {
        let mut i = 0;

        while i < self.numbers.len() {
            if self.numbers[i].1 < 10 {
                i += 1;
                continue
            }

            let num:f64 = self.numbers[i].1 as f64 / 2.0;

            self.numbers[i].0 += 1;
            self.numbers[i].1 = num.floor() as u32;
            self.numbers.insert(i+1, (self.numbers[i].0, num.ceil() as u32));

            return Some(())
        }

        None
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut numbers:Vec<Sfn> = vec![];
    
    for n in input.lines() {
        numbers.push(n.parse().unwrap());
    }

    let mut num = numbers.remove(0);

    for n in numbers.clone() {
        num.add(n);
    }

    println!("Step 1: {}", num.magnitude());

    let mut largest = 0;
    for l in numbers.clone().iter_mut() {
        for r in numbers.clone() {
            let mut ll = l.clone();
            ll.add(r);
            let magnitude = ll.magnitude();

            if magnitude > largest {
                largest = magnitude;
            }
        }
    }

    println!("Step 2: {}", largest);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse() {
        assert!("[1,2]".parse::<Sfn>().is_ok());
        assert!("[[1,2],3]".parse::<Sfn>().is_ok());
        let m = Sfn{numbers:vec![(2,9),(2,1),(2,1),(2,9)]};
        assert!(matches!("[[9,1],[1,9]]".parse::<Sfn>(), Ok(m)));
    }

    #[test]
    fn magnitude() {
        assert_eq!("[9,1]".parse::<Sfn>().unwrap().magnitude(), 29);
        assert_eq!("[[9,1],[1,9]]".parse::<Sfn>().unwrap().magnitude(), 129);
        assert_eq!("[[1,2],[[3,4],5]]".parse::<Sfn>().unwrap().magnitude(), 143);
        assert_eq!("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse::<Sfn>().unwrap().magnitude(), 4140);
    }

    #[test]
    fn explode() {
        let mut num = "[[[[[9,8],1],2],3],4]".parse::<Sfn>().unwrap();
        let opt = num.explode();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[[[[0,9],2],3],4]".parse::<Sfn>().unwrap());

        let mut num = "[7,[6,[5,[4,[3,2]]]]]".parse::<Sfn>().unwrap();
        let opt = num.explode();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[7,[6,[5,[7,0]]]]".parse::<Sfn>().unwrap());

        let mut num = "[[6,[5,[4,[3,2]]]],1]".parse::<Sfn>().unwrap();
        let opt = num.explode();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[[6,[5,[7,0]]],3]".parse::<Sfn>().unwrap());
        
        let mut num = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse::<Sfn>().unwrap();
        let opt = num.explode();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse::<Sfn>().unwrap());

        let mut num = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse::<Sfn>().unwrap();
        let opt = num.explode();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn split() {
        let mut num = Sfn{numbers:vec![(0,10)]};
        let opt = num.split();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[5,5]".parse::<Sfn>().unwrap());

        let mut num = Sfn{numbers:vec![(0,11)]};
        let opt = num.split();
        assert_eq!(opt, Some(()));
        assert_eq!(num, "[5,6]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn add() {
        let mut num = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse::<Sfn>().unwrap();
        num.reduce();
        assert_eq!(num, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn more_add() {
        let mut nums = vec![
            "[1,1]".parse::<Sfn>().unwrap(),
            "[2,2]".parse::<Sfn>().unwrap(),
            "[3,3]".parse::<Sfn>().unwrap(),
            "[4,4]".parse::<Sfn>().unwrap(),
            "[5,5]".parse::<Sfn>().unwrap(),
        ];

        let mut num = nums.remove(0);
        for n in nums {
            println!("adding {:?} and {:?}", num, n);
            num.add(n);
        }

        assert_eq!(num, "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn more_add2() {
        let mut nums = vec![
            "[1,1]".parse::<Sfn>().unwrap(),
            "[2,2]".parse::<Sfn>().unwrap(),
            "[3,3]".parse::<Sfn>().unwrap(),
            "[4,4]".parse::<Sfn>().unwrap(),
            "[5,5]".parse::<Sfn>().unwrap(),
            "[6,6]".parse::<Sfn>().unwrap(),
        ];

        let mut num = nums.remove(0);
        for n in nums {
            num.add(n);
        }

        assert_eq!(num, "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn big_add() {
        let mut nums = vec![
"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse::<Sfn>().unwrap(),
"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse::<Sfn>().unwrap(),
"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".parse::<Sfn>().unwrap(),
"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".parse::<Sfn>().unwrap(),
"[7,[5,[[3,8],[1,4]]]]".parse::<Sfn>().unwrap(),
"[[2,[2,2]],[8,[8,1]]]".parse::<Sfn>().unwrap(),
"[2,9]".parse::<Sfn>().unwrap(),
"[1,[[[9,3],9],[[9,0],[0,7]]]]".parse::<Sfn>().unwrap(),
"[[[5,[7,4]],7],1]".parse::<Sfn>().unwrap(),
"[[[[4,2],2],6],[8,7]]".parse::<Sfn>().unwrap(),
        ];

        let mut num = nums.remove(0);

        for n in nums {
            num.add(n);
        }

        assert_eq!(num, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<Sfn>().unwrap());
    }

    #[test]
    fn everything() {
        let mut nums: Vec<Sfn> = vec![
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".parse::<Sfn>().unwrap(),
"[[[5,[2,8]],4],[5,[[9,9],0]]]".parse::<Sfn>().unwrap(),
"[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse::<Sfn>().unwrap(),
"[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".parse::<Sfn>().unwrap(),
"[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".parse::<Sfn>().unwrap(),
"[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".parse::<Sfn>().unwrap(),
"[[[[5,4],[7,7]],8],[[8,3],8]]".parse::<Sfn>().unwrap(),
"[[9,3],[[9,9],[6,[4,9]]]]".parse::<Sfn>().unwrap(),
"[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".parse::<Sfn>().unwrap(),
"[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".parse::<Sfn>().unwrap(),
        ];

        let mut total = nums.remove(0);

        for n in nums {
            total.add(n);
        }

        assert_eq!(total, "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse::<Sfn>().unwrap());
        assert_eq!(4140, total.magnitude());
    }
}

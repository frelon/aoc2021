use std::collections::HashMap;

fn main() {
    let input = input();
    let mut rules = HashMap::new();

    let template = input[0];

    for line in input.iter().skip(2) {
        let mut split = line.split(" -> ");
        rules.insert(split.next().unwrap(), split.next().unwrap());
    }

    let mut pairs = HashMap::new();
    let mut counts = HashMap::new();

    let chars:Vec<char> = template.chars().collect();

    for w in chars[..].windows(2) {
        let mut s = String::from(w[0]);
        s.push(w[1]);

        *pairs.entry(s).or_insert(0) += 1;
        *counts.entry(w[0]).or_insert(0) += 1;
        *counts.entry(w[1]).or_insert(0) += 1;
    }

    for _ in 1..=40 {
        step(&mut pairs, &rules, &mut counts);
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("answer: {}",max-min)
}

fn step(pairs:&mut HashMap<String,u64>,rules:&HashMap<&str,&str>,chars:&mut HashMap<char,u64>) {
    for (pair,count) in pairs.clone().iter() {
        if count == &0 {
            continue;
        }

        match rules.get(&pair[..]) {
            Some(rule) => {
                *pairs.entry(pair.to_string()).or_insert(0) -= count;

                let mut pair1 = String::from(&pair[0..1]);
                pair1.push_str(rule);
                *pairs.entry(pair1).or_insert(0) += count;

                let mut pair2 = String::from(*rule);
                pair2.push_str(&pair[1..2]);
                *pairs.entry(pair2).or_insert(0) += count;

                *chars.entry(rule.chars().nth(0).unwrap()).or_insert(0) += count;
            },
            None => {},
        }
    }
}


fn input() -> Vec<&'static str> {
vec![
"VCOPVNKPFOOVPVSBKCOF",
"",
"NO -> K",
"PO -> B",
"HS -> B",
"FP -> V",
"KN -> S",
"HV -> S",
"KC -> S",
"CS -> B",
"KB -> V",
"OB -> V",
"HN -> S",
"OK -> N",
"PC -> H",
"OO -> P",
"HF -> S",
"CB -> C",
"SB -> V",
"FN -> B",
"PH -> K",
"KH -> P",
"NB -> F",
"KF -> P",
"FK -> N",
"FB -> P",
"FO -> H",
"CV -> V",
"CN -> P",
"BN -> N",
"SC -> N",
"PB -> K",
"VS -> N",
"BP -> P",
"CK -> O",
"PS -> N",
"PF -> H",
"HB -> S",
"VN -> V",
"OS -> V",
"OC -> O",
"BB -> F",
"SK -> S",
"NF -> F",
"FS -> S",
"SN -> N",
"FC -> S",
"BH -> N",
"HP -> C",
"VK -> F",
"CC -> N",
"SV -> H",
"SO -> F",
"HH -> C",
"PK -> P",
"NV -> B",
"KS -> H",
"NP -> H",
"VO -> C",
"BK -> V",
"VV -> P",
"HK -> B",
"CF -> B",
"BF -> O",
"OV -> B",
"OH -> C",
"PP -> S",
"SP -> S",
"CH -> B",
"OF -> F",
"NK -> F",
"FV -> F",
"KP -> O",
"OP -> O",
"SS -> P",
"CP -> H",
"BO -> O",
"KK -> F",
"HC -> N",
"KO -> V",
"CO -> F",
"NC -> P",
"ON -> P",
"KV -> C",
"BV -> K",
"HO -> F",
"PV -> H",
"VC -> O",
"NH -> B",
"PN -> H",
"VP -> O",
"NS -> N",
"NN -> S",
"BS -> H",
"SH -> P",
"VB -> V",
"VH -> O",
"FH -> K",
"FF -> H",
"SF -> N",
"BC -> H",
"VF -> P",
]
}

// 10002813279338 too high

use std::collections::HashMap;

fn main() {
    let input = input();
    let mut rules = HashMap::new();

    let pairs = input[0];

    for line in input.iter().skip(2) {
        let mut split = line.split(" -> ");
        rules.insert(split.next().unwrap(), split.next().unwrap());
    }

    let mut state = String::from(pairs);
    for _ in 1..=10 {
        state = step(state, &rules);
    }

    let mut map = HashMap::new();
    for ch in state.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut min = 10000;
    let mut max = 0;
    for (_, count) in map {
        if count < min {
            min = count;
        }

        if count > max {
            max = count;
        }
    }

    println!("answer1: {}", max-min);
}

fn step(pairs:String,rules:&HashMap<&str,&str>) -> String {
    let mut result = String::new();
    let chars:Vec<char> = pairs.chars().collect();
    for w in chars[..].windows(2) {
        let mut s = String::from(w[0]);
        s.push(w[1]);

        result.push(w[0]);

        match rules.get(&s[..]) {
            Some(rule) => { result.push_str(rule); },
            None => {},
        }
    }

    result.push(*chars.last().unwrap());

    result
}


fn input() -> Vec<&'static str> {
    // vec![
// "NNCB",
// "",
// "CH -> B",
// "HH -> N",
// "CB -> H",
// "NH -> C",
// "HB -> C",
// "HC -> B",
// "HN -> C",
// "NN -> C",
// "BH -> H",
// "NC -> B",
// "NB -> B",
// "BN -> B",
// "BB -> N",
// "BC -> B",
// "CC -> N",
// "CN -> C",
    // ]
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

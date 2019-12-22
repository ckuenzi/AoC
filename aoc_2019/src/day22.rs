use itertools::Itertools;

#[aoc_generator(day22)]
fn get_shuffles(input: &str) -> Vec<Operation> {
    let mut out = vec![];
    for line in input.lines() {
        let words = line.split(' ').collect_vec();
        match words[1] {
            "into" => out.push(Operation {
                shuffle: Shuffle::Reverse,
                amount: 0,
            }),
            "with" => out.push(Operation {
                shuffle: Shuffle::Increment,
                amount: words[3].parse::<i128>().unwrap(),
            }),
            _ => out.push(Operation {
                shuffle: Shuffle::Cut,
                amount: words[1].parse::<i128>().unwrap(),
            }),
        }
    }
    out
}

#[aoc(day22, part1)]
fn part1(shuffles: &Vec<Operation>) -> usize {
    let mut deck = (0..10007).into_iter().collect_vec();

    for shuffle in shuffles {
        match shuffle.shuffle {
            Shuffle::Increment => increment(&mut deck, &shuffle),
            Shuffle::Cut => cut(&mut deck, &shuffle),
            Shuffle::Reverse => reverse(&mut deck),
        }
    }
    deck.iter()
        .enumerate()
        .find(|(_, &card)| card == 2019)
        .unwrap()
        .0
}

fn increment(deck: &mut Vec<i128>, shuffle: &Operation) {
    let original = deck.clone();
    let mut pos = 0;
    for card in original {
        deck[pos] = card;
        pos = (pos + shuffle.amount as usize) % deck.len();
    }
}

fn cut(deck: &mut Vec<i128>, shuffle: &Operation) {
    let mut new_deck = vec![];
    if shuffle.amount > 0 {
        for i in shuffle.amount as usize..deck.len() {
            new_deck.push(deck[i]);
        }
        for i in 0..shuffle.amount as usize {
            new_deck.push(deck[i]);
        }
    } else {
        for i in (deck.len() as i128 + shuffle.amount) as usize..deck.len() {
            new_deck.push(deck[i]);
        }
        for i in 0..(deck.len() as i128 + shuffle.amount) as usize {
            new_deck.push(deck[i]);
        }
    }
    deck.clear();
    deck.append(&mut new_deck);
}

fn reverse(deck: &mut Vec<i128>) {
    deck.reverse();
}

#[aoc(day22, part2)]
fn part2(shuffles: &Vec<Operation>) -> i128 {
    const D: i128 = 119315717514047;
    const N: i128 = 101741582076661;
    let mut a = 1;
    let mut b = 0;

    for shuffle in shuffles.iter().rev() {
        match shuffle.shuffle {
            Shuffle::Cut => b += shuffle.amount + [0, D][(shuffle.amount > 0) as usize],
            Shuffle::Increment => {
                let i = mod_inv(shuffle.amount, D);
                a = a * i % D;
                b = b * i % D;
            }
            Shuffle::Reverse => {
                a *= -1;
                b = -b - 1;
            }
        }

        a %= D;
        b %= D;
        if a < 0 {
            a += D;
        }
        if b < 0 {
            b += D;
        }
    }

    (mod_pow(a, N, D) * 2020 % D
        + (b * (mod_pow(a, N, D) + D - 1) % D % D) * mod_pow(a - 1, D - 2, D))
        % D
}

fn mod_inv(a: i128, base: i128) -> i128 {
    let mut mn = (base, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += base;
    }
    xy.0
}

fn mod_pow(mut b: i128, mut e: i128, m: i128) -> i128 {
    let mut out = 1;
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            out = out * b % m;
        }
        e = e >> 1;
        b = b * b % m;
    }
    out
}

struct Operation {
    shuffle: Shuffle,
    amount: i128,
}

enum Shuffle {
    Reverse,
    Cut,
    Increment,
}

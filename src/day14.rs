use std::collections::BTreeMap;

type PairInsertions = BTreeMap<(char, char), char>;

fn parse(input: &str) -> (String, PairInsertions) {
    let (polymer_template, pair_insertions) = input.split_once("\n\n").unwrap();

    (
        polymer_template.to_string(),
        pair_insertions
            .lines()
            .map(|l| {
                let (pair, insertion) = l.split_once(" -> ").unwrap();
                let mut pair_chars = pair.chars();
                let first = pair_chars.next().unwrap();
                let second = pair_chars.next().unwrap();

                ((first, second), insertion.chars().next().unwrap())
            })
            .collect(),
    )
}

fn solve(template: &String, pair_insertions: &PairInsertions, steps: usize) -> usize {
    let mut pairs_count: BTreeMap<(char, char), usize> =
        template
            .as_bytes()
            .windows(2)
            .fold(BTreeMap::new(), |mut acc, w| {
                *acc.entry((w[0] as char, w[1] as char)).or_default() += 1;
                acc
            });

    let mut letters_count: BTreeMap<char, usize> =
        template.chars().fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c).or_default() += 1;
            acc
        });

    for _ in 0..steps {
        let mut new_pairs_count = BTreeMap::new();

        for ((first, second), count) in pairs_count {
            let inter = pair_insertions[&(first, second)];

            *new_pairs_count.entry((first, inter)).or_default() += count;
            *new_pairs_count.entry((inter, second)).or_default() += count;

            *letters_count.entry(inter).or_default() += count;
        }

        pairs_count = new_pairs_count;
    }

    let max = *letters_count.values().max().unwrap();
    let min = *letters_count.values().min().unwrap();

    max - min
}

pub fn p1(input: &str) -> usize {
    let (template, pair_insertions) = parse(input);
    solve(&template, &pair_insertions, 10)
}

pub fn p2(input: &str) -> usize {
    let (template, pair_insertions) = parse(input);
    solve(&template, &pair_insertions, 40)
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"),
        1588
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"),
        2188189693529
    );
}

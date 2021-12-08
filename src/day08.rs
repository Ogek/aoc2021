fn parse(input: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    input.lines().map(|l| l.split_once("|").unwrap())
}

pub fn p1(input: &str) -> usize {
    parse(input)
        .map(|(_, digits)| digits.split_whitespace())
        .flatten()
        .filter(|d| [2, 4, 3, 7].contains(&d.len()))
        .count()
}

struct Decoder<'a> {
    one: &'a str,
    four: &'a str,
}

pub fn p2(input: &str) -> usize {
    parse(input)
        .map(|(patterns, digits)| Decoder::new(patterns).decode(digits))
        .sum()
}

impl<'a> Decoder<'_> {
    fn new(patterns: &str) -> Decoder {
        let one = patterns
            .split_whitespace()
            .filter(|s| s.len() == 2)
            .next()
            .unwrap();

        let four = patterns
            .split_whitespace()
            .filter(|s| s.len() == 4)
            .next()
            .unwrap();

        Decoder { one, four }
    }

    fn count_segments_matching_with_one(&self, s: &str) -> usize {
        self.one.chars().filter(|c| s.contains(*c)).count()
    }

    fn count_segments_matching_with_four(&self, s: &str) -> usize {
        self.four.chars().filter(|c| s.contains(*c)).count()
    }

    fn decode_single(&self, s: &str) -> &str {
        match (
            s.len(),
            self.count_segments_matching_with_one(s),
            self.count_segments_matching_with_four(s),
        ) {
            (6, 2, 3) => "0",
            (2, 2, 2) => "1",
            (5, 1, 2) => "2",
            (5, 2, 3) => "3",
            (4, 2, 4) => "4",
            (5, 1, 3) => "5",
            (6, 1, 3) => "6",
            (3, 2, 2) => "7",
            (7, 2, 4) => "8",
            (6, 2, 4) => "9",
            (_, _, _) => unreachable!(),
        }
    }

    fn decode(&self, digits: &str) -> usize {
        digits
            .split_whitespace()
            .map(|digit| self.decode_single(digit))
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

#[test]
fn test_p1() {
    assert_eq!(
        p1(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ),
        26
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ),
        61229
    );
}

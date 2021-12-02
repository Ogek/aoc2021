fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines().map(|n| n.parse().unwrap())
}

pub fn p1(input: &str) -> usize {
    let mut ans = 0;
    let mut last: Option<usize> = None;

    for n in parse(input) {
        if let Some(prev) = last {
            if n > prev {
                ans += 1;
            }
        }
        last = Some(n);
    }
    ans
}

pub fn p2(input: &str) -> usize {
    let mut ans = 0;
    let mut last: Option<usize> = None;
    let mut last2: Option<usize> = None;
    let mut last_sum: Option<usize> = None;

    for n in parse(input) {
        if let Some(prev) = last {
            if let Some(prev2) = last2 {
                let sum = n + prev + prev2;
                if let Some(prev_sum) = last_sum {
                    if sum > prev_sum {
                        ans += 1;
                    }
                }
                last_sum = Some(sum)
            }
        }
        last2 = last;
        last = Some(n);
    }
    ans
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("199
200
208
210
200
207
240
269
260
263"),
        7
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("199
200
208
210
200
207
240
269
260
263"),
        5
    );
}

#[derive(Debug)]
enum LineError {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn mirror(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn corrupted_score(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn incomplete_score(remaining: &Vec<char>) -> usize {
    remaining.iter().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    })
}

fn check_line(line: &str) -> Result<(), LineError> {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '{' | '[' | '(' | '<' => stack.push(mirror(&c)),
            _ => match stack.pop() {
                Some(expected) => {
                    if c != expected {
                        return Err(LineError::Corrupted(c));
                    }
                }
                None => return Err(LineError::Corrupted(c)),
            },
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        return Err(LineError::Incomplete(stack));
    }

    Ok(())
}

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(check_line)
        .filter_map(|res| match res {
            Err(LineError::Corrupted(c)) => Some(corrupted_score(&c)),
            _ => None,
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .map(check_line)
        .filter_map(|res| match res {
            Err(LineError::Incomplete(remaining)) => Some(incomplete_score(&remaining)),
            _ => None,
        })
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"),
        26397
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"),
        288957
    );
}

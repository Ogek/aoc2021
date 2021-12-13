use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Graph {
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(HashMap::new(), |mut acc, (s, e)| {
            if e != "start" && s != "end" {
                acc.entry(s).or_insert_with(Vec::new).push(e);
            }

            if e != "end" && s != "start" {
                acc.entry(e).or_insert_with(Vec::new).push(s);
            }

            acc
        })
}

fn count_paths(graph: &Graph, start: &str, end: &str) -> usize {
    fn do_count<'a>(
        graph: &'a Graph,
        start: &'a str,
        end: &'a str,
        visited: &mut HashMap<&'a str, usize>,
        count: &mut usize,
        path: &mut Vec<&'a str>,
    ) {
        *visited.entry(start).or_default() += 1;
        path.push(start);
        if start == end {
            *count += 1;
        } else if let Some(q) = graph.get(start) {
            for next in q {
                let visited_count = *visited.entry(next).or_default();

                if !next.chars().all(char::is_uppercase) && visited_count >= 1 {
                    continue;
                }

                do_count(graph, next, end, visited, count, path);
            }
        } else {
            unreachable!();
        }
        path.pop();
        *visited.entry(start).or_default() -= 1;
    }
    let mut count = 0;
    let mut visited: HashMap<&str, usize> = HashMap::with_capacity(graph.len());
    let mut path: Vec<&str> = Vec::new();

    do_count(graph, start, end, &mut visited, &mut count, &mut path);

    count
}

fn count_paths2(graph: &Graph, start: &str, end: &str) -> usize {
    fn do_count<'a>(
        graph: &'a Graph,
        start: &'a str,
        end: &'a str,
        visited: &mut HashMap<&'a str, usize>,
        count: &mut usize,
        path: &mut Vec<&'a str>,
    ) {
        *visited.entry(start).or_default() += 1;

        path.push(start);
        if start == end {
            *count += 1;
        } else if let Some(q) = graph.get(start) {
            for next in q {
                let visited_count = *visited.entry(next).or_default();
                if !next.chars().all(char::is_uppercase) {
                    if let Some(single_small) = visited
                        .iter()
                        .find(|(k, v)| !k.chars().all(char::is_uppercase) && **v >= 2)
                    {
                        if next == single_small.0 {
                            continue;
                        }

                        if visited_count >= 1 {
                            continue;
                        }
                    }
                }

                do_count(graph, next, end, visited, count, path);
            }
        } else {
            unreachable!();
        }
        path.pop();
        *visited.entry(start).or_default() -= 1;
    }
    let mut count = 0;
    let mut visited: HashMap<&str, usize> = HashMap::with_capacity(graph.len());
    let mut path: Vec<&str> = Vec::new();

    do_count(graph, start, end, &mut visited, &mut count, &mut path);

    count
}

pub fn p1(input: &str) -> usize {
    let graph = parse(input);

    count_paths(&graph, "start", "end")
}

pub fn p2(input: &str) -> usize {
    let graph = parse(input);

    count_paths2(&graph, "start", "end")
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("start-A
start-b
A-c
A-b
b-d
A-end
b-end"),
        10
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("start-A
start-b
A-c
A-b
b-d
A-end
b-end"),
        36
    );
}

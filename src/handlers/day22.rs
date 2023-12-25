use crate::prelude::*;
use std::collections::HashMap;

pub async fn get_gift_emojis(numbers: String) -> String {
    let counter = numbers
        .trim()
        .lines()
        .fold(HashMap::<&str, usize>::new(), |mut counter, l| {
            counter.entry(l).and_modify(|e| *e += 1).or_insert(1);
            counter
        });
    let n = counter
        .iter()
        .filter(|(_, v)| **v < 2)
        .flat_map(|(k, _)| k.parse::<u64>().ok())
        .take(1)
        .collect::<Vec<_>>();

    if n.is_empty() {
        return String::new();
    }

    (0..n[0]).map(|_| "🎁").collect::<String>()
}

pub async fn rocket(body: String) -> String {
    tracing::info!("rocket request body: {body}");

    let (stars, portals) = parse_stars_and_portals(&body);
    let Some(path) = get_path_from_portal(&portals, &stars) else {
        return String::new();
    };
    let distance = calculate_distance(&stars, &path);

    let result = f!("{} {:.3}", path.len(), distance);
    tracing::info!("rocket response: {result}");
    result
}

type Stars = Vec<(isize, isize, isize)>;
type Portals = HashMap<usize, Vec<usize>>;

fn parse_stars_and_portals(input: &str) -> (Stars, Portals) {
    let mut stars = Stars::new();
    let mut portals = Portals::new();
    let mut stars_num = 0usize;
    let mut portals_num = 0usize;
    for (i, line) in input.trim().lines().enumerate() {
        if i == 0 {
            if let Ok(n) = line.trim().parse::<usize>() {
                stars_num = n;
            } else {
                break;
            }
            continue;
        }

        let mut r = line.split_whitespace();
        if r.clone().count() == 1 {
            let Ok(n) = line.trim().parse::<usize>() else {
                break;
            };
            portals_num = n;
            continue;
        }

        if stars.len() != stars_num {
            let Some(x) = r.next().and_then(|el| el.parse::<isize>().ok()) else {
                break;
            };
            let Some(y) = r.next().and_then(|el| el.parse::<isize>().ok()) else {
                break;
            };
            let Some(z) = r.next().and_then(|el| el.parse::<isize>().ok()) else {
                break;
            };
            stars.push((x, y, z));
            continue;
        }

        if portals.values().flatten().count() != portals_num {
            let Some(x) = r.next().and_then(|el| el.parse::<usize>().ok()) else {
                break;
            };
            let Some(y) = r.next().and_then(|el| el.parse::<usize>().ok()) else {
                break;
            };
            portals.entry(x).or_default().push(y);
        }
    }

    (stars, portals)
}

fn get_path_from_portal(portals: &Portals, stars: &Stars) -> Option<Vec<(usize, usize)>> {
    let end = stars.len() - 1;
    let Some(path) = pathfinding::directed::bfs::bfs(
        &0,
        |p| portals.get(p).cloned().unwrap_or_default(),
        |p| *p == end,
    ) else {
        tracing::error!("No path found");
        return None;
    };

    let path = path.windows(2).map(|c| (c[0], c[1])).collect::<Vec<_>>();
    Some(path)
}

fn calculate_distance(stars: &Stars, path: &Vec<(usize, usize)>) -> f32 {
    let mut distance = 0f32;

    for p in path {
        let (x1, y1, z1) = stars[p.0];
        let (x2, y2, z2) = stars[p.1];

        distance += ((x2 as f32 - x1 as f32).powi(2)
            + (y2 as f32 - y1 as f32).powi(2)
            + (z2 as f32 - z1 as f32).powi(2))
        .sqrt();
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
5
0 4
0 1
1 2
2 0
0 3",
        "1 6.708"
    )]
    #[case(
        "5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
4
0 1
2 4
3 4
1 2",
        "3 26.123"
    )]
    #[case(
        "5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
5
0 1
1 3
3 4
0 2
2 4",
        "2 18.776"
    )]
    #[case(
        "21
570 -435 923
672 -762 -218
707 16 640
311 902 47
-963 -399 -773
788 532 -704
703 475 -145
-303 -394 -369
699 -640 952
-341 -221 743
740 -146 544
-424 655 179
-630 161 690
789 -848 -517
-14 -893 551
-48 815 962
528 552 -96
337 983 165
-565 459 -90
81 -476 301
-685 -319 698
24
0 2
2 4
4 6
6 10
10 17
17 20
20 18
18 11
11 7
7 5
5 3
3 0
0 1
1 12
12 13
13 19
19 20
20 16
16 14
14 15
15 9
9 8
8 6
11 16",
        "5 7167.055"
    )]
    #[tokio::test]
    async fn test_rocket(#[case] input: &str, #[case] expected: &str) {
        let result = rocket(input.to_string()).await;
        assert_eq!(result, expected);
    }
}

/* Don't look this is gross */
fn tree_at(x: isize, y: isize, trees: &Vec<Vec<isize>>) -> isize {
    return trees[y as usize][x as usize];
}

/* Check to see if a tree is visible */
fn is_visible(x: isize, y: isize, trees: &Vec<Vec<isize>>) -> bool {
    let height: isize = trees.len() as isize;
    let width : isize = trees[0].len() as isize;

    /* Check each direction from a tree */
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (mut cx, mut cy) = (x + dx, y + dy);

        /* Scan from the tree to the edge, stop if the tree is lower */
        while (cx >= 0 && cx < width) && (cy >= 0 && cy < height) &&
              (tree_at(cx, cy, trees) < tree_at(x, y, trees)) {
            cx += dx;
            cy += dy;
        }

        /* We made it to the edge, it is visible */
        if !((cx >= 0 && cx < width) && (cy >= 0 && cy < height)) {
            return true;
        }
    }

    return false;
}

/* Find the tree's scienic score */
fn scienic(x: isize, y: isize, trees: &Vec<Vec<isize>>) -> isize {
    let height: isize = trees.len() as isize;
    let width : isize = trees[0].len() as isize;

    let mut score = 1;

    /* Check each direction from a tree */
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut dist = 0;
        let (mut cx, mut cy) = (x + dx, y + dy);

        /* Scan from tree to the each to check how far we can see */
        while (cx >= 0 && cx < width) && (cy >= 0 && cy < height) {
            dist += 1; // We can see an additional direction
            if tree_at(cx, cy, trees) >= tree_at(x, y, trees) {
                break; // This tree is taller, we can't see
            }

            cx += dx;
            cy += dy;
        }

        score *= dist; // Score is product of visible dist in each dir
    }

    return score;
}

pub fn part1(trees: Vec<Vec<isize>>) -> isize {
    let height      = trees.len();
    let width       = trees[0].len();
    let mut visible = 0;

    for x in 0..width {
        for y in 0..height {
            if is_visible(x as isize,y as isize,&trees) {
                visible += 1;
            }
        }
    }

    return visible;
}

pub fn part2(trees: Vec<Vec<isize>>) -> isize {
    let height      = trees.len();
    let width       = trees[0].len();
    let mut score   = 0;

    for x in 0..width {
        for y in 0..height {
            score = score.max(scienic(x as isize, y as isize, &trees));
        }
    }

    return score;
}

pub fn parse(input: &str) -> Vec<Vec<isize>> {
    return input
        .lines()
        .map(|l| l.chars()
            .map(|c| (c as isize) - '0' as isize)
            .collect()
        ).collect();
}

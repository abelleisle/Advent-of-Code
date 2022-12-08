#![allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

/* Don't look this is gross */
fn tree_at(x: isize, y: isize, trees: &Vec<&str>) -> isize {
    return (trees[y as usize].chars().nth(x as usize).unwrap() as isize) - '0' as isize;
}

/* Check to see if a tree is visible */
fn is_visible(x: isize, y: isize, trees: &Vec<&str>) -> bool {
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
fn scienic(x: isize, y: isize, trees: &Vec<&str>) -> isize {
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

fn main() {
    let trees : Vec<&str> =
        include_str!("../input.txt")
        .lines()
        .collect();

    let height      = trees.len();
    let width       = trees[0].len();
    let mut visible = 0;
    let mut score   = 0;

    for x in 0..width {
        for y in 0..height {
            if is_visible(x as isize,y as isize,&trees) {
                visible += 1;
            }

            score = score.max(scienic(x as isize, y as isize, &trees));
        }
    }

    println!("P1: {visible}");
    println!("P2: {score}");
}

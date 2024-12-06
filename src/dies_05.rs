use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::bail;

type Node = i32;
type Rule = (Node, Node);
type Rules = Vec<Rule>;
type Update = Vec<Node>;
type Updates = Vec<Update>;

type Graph = HashMap<Node, HashSet<Node>>;

pub struct Data {
    rules: Rules,
    updates: Updates,
}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = fs::read_to_string(path)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let mut rules: Rules = Vec::new();
        let mut updates: Updates = Vec::new();
        for rule in lines.by_ref().take_while(|line| !line.is_empty()) {
            let fields: Vec<Node> = rule
                .split('|')
                .filter_map(|n| n.parse::<Node>().ok())
                .collect();
            match &fields[..] {
                [a, b] => {
                    let rule: Rule = (*a, *b);
                    rules.push(rule);
                }
                _ => bail!("Invalid rule: {rule:?}"),
            }
        }
        for update in lines {
            let update: Update = update
                .split(',')
                .filter_map(|n| n.parse::<Node>().ok())
                .collect();
            if update.is_empty() {
                bail!("Invalid update: {update:?}");
            }
            updates.push(update);
        }
        Ok(Self { rules, updates })
    }

    pub fn solve1(&self) -> anyhow::Result<Node> {
        let (total_mid_valid, _) = solve(&self.updates[..], &self.rules[..])?;
        Ok(total_mid_valid)
    }

    pub fn solve2(&self) -> anyhow::Result<i32> {
        let (_, total_mid_fixed) = solve(&self.updates[..], &self.rules[..])?;
        Ok(total_mid_fixed)
    }
}

fn solve(updates: &[Update], rules: &[Rule]) -> anyhow::Result<(i32, i32)> {
    let g = graph_build(rules);
    let mut total_mid_valid = 0;
    let mut total_mid_fixed = 0;
    for update in updates {
        let n = update.len();
        assert_eq!(1, n % 2, "Odd number of pages in update.");
        if update_is_valid(&update[..], &g) {
            total_mid_valid += update[n / 2];
        } else {
            let g: Graph = graph_filter(&g, &update[..]);
            let update_reordered = graph_topo_sort(&g)?;
            total_mid_fixed += update_reordered[n / 2];
        }
    }
    Ok((total_mid_valid, total_mid_fixed))
}

fn update_is_valid(update: &[Node], rules: &Graph) -> bool {
    let relations =
        update.iter().copied().zip(update.iter().skip(1).copied());
    for (parent, child) in relations {
        if let Some(children) = rules.get(&parent) {
            if !children.contains(&child) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
fn graph_topo_sort(g: &Graph) -> anyhow::Result<Vec<Node>> {
    let mut sorted = Vec::new();
    let mut visited: HashSet<Node> = HashSet::new(); // "permanent mark"
    for parent in g.keys() {
        let mut ancestors = HashSet::new(); // "temporary mark"
        graph_topo_sort_visit(
            g,
            &mut visited,
            &mut ancestors,
            &mut sorted,
            *parent,
        )?;
    }
    Ok(sorted)
}

fn graph_topo_sort_visit(
    g: &Graph,
    visited: &mut HashSet<Node>,
    ancestors: &mut HashSet<Node>,
    sorted: &mut Vec<Node>,
    node: Node,
) -> anyhow::Result<()> {
    if !visited.contains(&node) {
        if ancestors.contains(&node) {
            bail!("cycle");
        }
        ancestors.insert(node);
        if let Some(children) = g.get(&node) {
            for child in children {
                graph_topo_sort_visit(g, visited, ancestors, sorted, *child)?;
            }
        }
        visited.insert(node);
        sorted.push(node);
    }
    Ok(())
}

fn graph_filter(g: &Graph, keep: &[Node]) -> Graph {
    let keep: HashSet<Node> = HashSet::from_iter(keep.iter().copied());
    g.iter()
        .filter(|&(parent, _)| keep.contains(parent))
        .map(|(parent, children)| {
            let children: HashSet<Node> = children
                .iter()
                .filter(|child| keep.contains(child))
                .copied()
                .collect();
            (*parent, children)
        })
        .collect()
}

fn graph_build(rules: &[Rule]) -> Graph {
    let mut g: Graph = HashMap::new();
    for (parent, child) in rules {
        g.entry(*parent)
            .and_modify(|children| {
                children.insert(*child);
            })
            .or_insert(HashSet::from([*child]));
    }
    g
}

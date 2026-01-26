use std::{collections::HashMap, fs};

use nom::{
    IResult, Parser,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, preceded},
};

const INPUT_FILE: &str = "src/aoc/y2017/day7.txt";

#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    weight: u32,
    children: Vec<String>,
}

type Tree = HashMap<String, TreeNode>;

#[allow(dead_code)]
#[derive(Debug)]
enum TreeError {
    ParseError(String),
    NodeNotFound(String),
    NoSolution,
    IoError(std::io::Error),
}

impl From<std::io::Error> for TreeError {
    fn from(err: std::io::Error) -> Self {
        TreeError::IoError(err)
    }
}

type Result<T> = std::result::Result<T, TreeError>;

fn read_input() -> Result<String> {
    Ok(fs::read_to_string(INPUT_FILE)?)
}
pub fn day7_fst() -> String {
    let content = read_input().expect("Failed to read input file");
    let tree = parse_input(&content).expect("Failed to parse input");
    find_root(&tree).expect("No root found")
}

pub fn day7_snd() -> u32 {
    let content = read_input().expect("Failed to read input file");
    let tree = parse_input(&content).expect("Failed to parse input");
    let root = find_root(&tree).expect("No root found");
    find_unbalanced_weight(&tree, &root).expect("No solution found")
}

fn parse_input(content: &str) -> Result<Tree> {
    let (_, nodes) = separated_list1(tag("\n"), parse_line)(content)
        .map_err(|e| TreeError::ParseError(format!("Parse error: {:?}", e)))?;

    let mut tree = HashMap::new();
    for node in nodes {
        tree.insert(node.name.clone(), node);
    }
    Ok(tree)
}

fn parse_line(line: &str) -> IResult<&str, TreeNode> {
    let (rest, name) = alpha1(line)?;
    let (rest, _) = char(' ')(rest)?;
    let (rest, weight) = delimited(char('('), is_not(")"), char(')'))(rest)?;
    let (rest, children) = opt(preceded(tag(" -> "), parse_names))(rest)?;

    let weight = weight.parse::<u32>().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(weight, nom::error::ErrorKind::Digit))
    })?;

    Ok((
        rest,
        TreeNode {
            name: name.to_owned(),
            weight,
            children: children.unwrap_or(vec![]),
        },
    ))
}

fn parse_names(content: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), alpha1)
        .map(|names| {
            names
                .into_iter()
                .map(|name: &str| name.to_owned())
                .collect()
        })
        .parse(content)
}

fn find_root(tree: &Tree) -> Option<String> {
    let mut children_set = std::collections::HashSet::new();

    // Collect all children
    for node in tree.values() {
        for child in &node.children {
            children_set.insert(child.clone());
        }
    }

    // Find node that's not in children set
    for node_name in tree.keys() {
        if !children_set.contains(node_name) {
            return Some(node_name.clone());
        }
    }

    None
}

#[derive(Debug)]
enum DfsResult {
    Weight(u32),   // Normal case: return subtree total weight
    Solution(u32), // Found imbalance: return corrected individual weight
}

fn find_unbalanced_weight(tree: &Tree, root: &str) -> Result<u32> {
    match dfs_traverse(tree, root)? {
        DfsResult::Weight(_) => Err(TreeError::NoSolution),
        DfsResult::Solution(corrected_weight) => Ok(corrected_weight),
    }
}

fn dfs_traverse(tree: &Tree, node_name: &str) -> Result<DfsResult> {
    let node = tree
        .get(node_name)
        .ok_or_else(|| TreeError::NodeNotFound(node_name.to_string()))?;

    // Base case: leaf node
    if node.children.is_empty() {
        return Ok(DfsResult::Weight(node.weight));
    }

    let mut child_weights = Vec::new();

    // Process all children recursively (post-order traversal)
    for child_name in &node.children {
        match dfs_traverse(tree, child_name)? {
            DfsResult::Solution(solution) => {
                // Bubble up solution
                return Ok(DfsResult::Solution(solution));
            }
            DfsResult::Weight(weight) => {
                child_weights.push(weight);
            }
        }
    }

    // Check for imbalance among children at this level
    if let Some(corrected_weight) = detect_imbalance_dfs(tree, node, &child_weights)? {
        return Ok(DfsResult::Solution(corrected_weight));
    }

    // No imbalance found - return total weight of this subtree
    let total_weight = node.weight + child_weights.iter().sum::<u32>();
    Ok(DfsResult::Weight(total_weight))
}

fn detect_imbalance_dfs(
    tree: &Tree,
    parent_node: &TreeNode,
    child_weights: &[u32],
) -> Result<Option<u32>> {
    if child_weights.len() < 2 {
        return Ok(None);
    }

    // Group children by their total weights
    let mut weight_groups: HashMap<u32, Vec<usize>> = HashMap::new();
    for (index, &weight) in child_weights.iter().enumerate() {
        weight_groups.entry(weight).or_default().push(index);
    }

    // If exactly 2 different weights, we have an imbalance
    if weight_groups.len() == 2 {
        let weights: Vec<_> = weight_groups.keys().collect();
        let weight1 = *weights[0];
        let weight2 = *weights[1];

        let group1 = &weight_groups[&weight1];
        let group2 = &weight_groups[&weight2];

        // Find the unique weight (group with exactly 1 child)
        let (unique_weight, unique_index, common_weight) = if group1.len() == 1 {
            (weight1, group1[0], weight2)
        } else if group2.len() == 1 {
            (weight2, group2[0], weight1)
        } else {
            return Ok(None); // No unique weight found
        };

        // Get the unbalanced child's name and individual weight
        let unbalanced_child_name = &parent_node.children[unique_index];
        let unbalanced_child = tree
            .get(unbalanced_child_name)
            .ok_or_else(|| TreeError::NodeNotFound(unbalanced_child_name.clone()))?;

        // Calculate what the unbalanced child's individual weight should be
        let corrected_individual_weight = unbalanced_child.weight - (unique_weight - common_weight);
        return Ok(Some(corrected_individual_weight));
    }

    Ok(None)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example_parsing() {
        let content = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;

        let tree = parse_input(content).expect("Failed to parse example");
        assert_eq!(tree.len(), 13);

        let root = find_root(&tree).expect("Failed to find root");
        assert_eq!(root, "tknk");
    }

    #[test]
    fn example_unbalanced() {
        let content = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;

        let tree = parse_input(content).expect("Failed to parse example");
        let root = find_root(&tree).expect("Failed to find root");
        let result =
            find_unbalanced_weight(&tree, &root).expect("Failed to find unbalanced weight");
        assert_eq!(result, 60);
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day7_fst(), "aapssr".to_string());
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day7_snd(), 1458);
    }
}

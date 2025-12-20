use std::collections::VecDeque;

use swayipc::*;

pub fn parse_output_nodes(tree: &Node) -> Vec<&Node> {
    let mut output_nodes = vec![];
    let mut q = VecDeque::new();

    q.push_back(tree);
    while let Some(node) = q.pop_front() {
        // If we have an output node (and it's not a special/virtual output)
        if (node.node_type == NodeType::Output)
            && !node.nodes.is_empty()
            && node
                .name
                .as_ref()
                .is_none_or(|name| name != "__i3" && name != "__i3_scratch")
        {
            output_nodes.push(node);
        }

        // Look for more outputs in the children
        for child in &node.nodes {
            q.push_back(child);
        }
    }

    output_nodes
}

pub fn find_focused_workspace(output: &Node) -> Option<Node> {
    output
        .clone()
        .find_focused(|n| n.node_type == swayipc::NodeType::Workspace)
}

pub fn get_all_windows(workspace: &Node) -> Vec<Node> {
    let mut nodes = vec![];
    let mut q = VecDeque::new();

    q.push_back(workspace.clone());
    while let Some(node) = q.pop_back() {
        // if we have a window
        if (node.node_type == NodeType::Con || node.node_type == NodeType::FloatingCon)
            && node.nodes.is_empty()
        {
            nodes.push(node.clone());
        }

        // tiled/tabbed/stacked nodes
        for child in &node.nodes {
            let mut c = child.clone();
            // a bit of a hack to keep track of stacked/tabbed layouts:
            // if we're stacked, we set our childrens' layouts to stacked and change the decorator
            // height.
            if node.node_type == NodeType::Con && node.layout == NodeLayout::Stacked {
                c.layout = node.layout;
                // change the decoration height to be the *total height* of all the decorations in
                // the stacked container.
                c.deco_rect.height *= node.nodes.len() as i32;
            }
            q.push_back(c);
        }

        // floating nodes
        for child in &node.floating_nodes {
            q.push_back(child.clone());
        }
    }

    nodes.reverse();
    nodes
}

use std::collections::VecDeque;

pub struct Node {
    idx: usize,
    name: String,
    size: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}
pub struct Tree {
    nodes: Vec<Node>,
}

impl Node {
    pub fn new(idx: usize, name: String, size: usize) -> Self {
        Self {
            idx,
            name,
            size,
            parent: None,
            children: vec![],
        }
    }
}

impl Tree {
    fn add_node(&mut self, name: String, size: usize) -> usize {
        let len = self.nodes.len();
        self.nodes.push(Node::new(len, name, size));
        return len;
    }

    pub fn update_size(&mut self, idx: usize) -> usize {
        if self.nodes[idx].size != 0 {
            return self.nodes[idx].size;
        }
        let mut total = 0;
        let children = self.nodes[idx].children.clone();
        for i in children.iter() {
            total += self.update_size(*i);
        }
        self.nodes[idx].size = total;
        return total;
    }

    fn create_dir_from_commands(&mut self, commands: &Vec<Vec<String>>, start: usize, end: usize, parent: usize) -> Vec<usize> {
        commands.iter().skip(start).take(end - start).map(
            |x| {
                match x[0].as_str() {
                    "dir" => {
                        let idx = self.add_node(x[1].clone(), 0);
                        self.nodes[idx].parent = Some(parent);
                        idx
                    }
                    size => {
                        let idx = self.add_node(x[1].clone(), size.parse::<usize>().unwrap());
                        self.nodes[idx].parent = Some(parent);
                        idx
                    }
                }
            }
        ).collect()
    }

    pub fn create_tree(commands: &Vec<Vec<String>>) -> Tree {
        let mut command_idx = 0;
        let mut root = Tree{ nodes: vec![] };
        let mut path_stack: VecDeque<usize> = VecDeque::new();
        while command_idx < commands.len() {
            if commands[command_idx][1].eq("cd") {
                if !commands[command_idx][2].eq(".."){
                    let mut idx = 0;
                    if path_stack.len() == 0 {
                        idx = root.add_node(commands[command_idx][2].clone(), 0);
                    }
                    else {
                        let last_idx = *(path_stack.back().unwrap());
                        let node_list = &root.nodes[last_idx].children;
                        for i in node_list.iter() {
                            if root.nodes[*i].name.eq(&commands[command_idx][2]) {
                                idx = *i;
                            }
                        }
                    }
                    path_stack.push_back(idx);
                    command_idx += 2;
                    let start = command_idx;
                    while command_idx < commands.len() && !commands[command_idx][0].eq("$") {
                        command_idx += 1;
                    }
                    let end = command_idx;
                    let mut children = root.create_dir_from_commands(commands, start, end, idx);
                    root.nodes[idx].children.append(&mut children);
                }
                else {
                    path_stack.pop_back();
                    command_idx += 1;
                }
                
            }
        }
        root
    }

    pub fn pretty_print(&self) {
        self.print_helper(0, 0);
    }

    fn print_helper(&self, idx: usize, indent: usize) {
        let node = &self.nodes[idx];
        if node.children.len() == 0 {
            println!("{}- {} (file, size={})", "    ".repeat(indent), node.name, node.size);
        }
        else {
            println!("{}- {} (dir) {}", "    ".repeat(indent), node.name, node.size);
            for i in node.children.iter() {
                self.print_helper(*i, indent + 1);
            }
        }
    }

    pub fn part1(&self, idx: usize) -> usize {
        let node = &self.nodes[idx];
        let mut total = 0;
        if node.children.len() == 0 {
            return 0;
        }
        else {
            if node.size <= 100000 {
                total += node.size;
            }
            for i in node.children.iter() {
                total += self.part1(*i);
            }
        }
        total
    }


    pub fn part2(&self, idx: usize) -> usize {
        let space_needed = 30000000 - (70000000 - self.nodes[0].size);
        let node = &self.nodes[idx];
        let mut min = usize::MAX;
        if node.children.len() == 0 {
            return usize::MAX;
        }
        else {
            if node.size >= space_needed && node.size < min {
                min = node.size;
            }
            for i in node.children.iter() {
                let temp = self.part2(*i);
                if temp < min {
                    min = temp;
                }
            }
        }
        min
    }

}

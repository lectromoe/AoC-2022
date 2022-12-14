use std::{cell::RefCell, fs, rc::Rc};

#[derive(Default)]
struct Node {
    size: u32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap() + "\n";

    let root = Rc::new(RefCell::new(Node::default()));
    let mut current = Rc::clone(&root);
    let mut sum = 0;

    input.lines().for_each(|line| {
        let symbols = line.split_whitespace().collect::<Vec<_>>();
        if symbols.is_empty() {
            while current.borrow().parent.as_ref().is_some() {
                go_parent_dir(&mut current, &mut sum);
            }
            return;
        }

        match symbols[0] {
            "$" => {
                if let "cd" = symbols[1] {
                    if let ".." = symbols[2] {
                        go_parent_dir(&mut current, &mut sum);
                    } else {
                        let child = Rc::new(RefCell::new(Node {
                            parent: Some(Rc::clone(&current)),
                            ..Default::default()
                        }));
                        current.as_ref().borrow_mut().children.push(child.clone());
                        current = child;
                    }
                }
            }
            x if x.parse::<u32>().is_ok() => {
                let num = x.parse::<u32>().unwrap();
                current.as_ref().borrow_mut().size += num;
            }
            "dir" => (),
            _ => unreachable!(),
        };
    });
    // root.borrow().print(0);

    println!("{sum}");
}

fn go_parent_dir(current: &mut Rc<RefCell<Node>>, sum: &mut u32) {
    let mut last_pass = true;
    let size = current.as_ref().borrow_mut().size;
    let child_size = current
        .as_ref()
        .borrow_mut()
        .children
        .iter()
        .map(|child| {
            let size = child.as_ref().borrow().size;
            if size == 0 {
                last_pass = false;
            }
            size
        })
        .sum::<u32>();
    let total_size = size + child_size;
    if last_pass && total_size <= 100_000 {
        *sum += size + child_size;
    }
    *current = Rc::clone(current.clone().borrow().parent.as_ref().unwrap());
}

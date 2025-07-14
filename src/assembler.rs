use std::{collections::HashMap, path::Path};

const PROGRAM_START: usize = 512;

pub fn assemble<P>(path: P) -> ()
where
    P: AsRef<Path>,
{
    let file = std::fs::read_to_string("./src/file.txt").expect("Error: file couldnt be found");

    let mut program: Vec<Vec<&str>> = vec![];
    let mut label_map: HashMap<&str, usize> = HashMap::new();

    for line in file.lines() {
        let line_wo_comments: Vec<&str> = line
            .split("//")
            .next()
            .unwrap()
            .split_whitespace()
            .collect();
        match line_wo_comments.as_slice() {
            [] => (),
            list => program.push(list.to_vec()),
        }
    }

    for (i, line) in program.iter().enumerate() {
        match line.as_slice() {
            [one_element] => match one_element.split(":").collect::<Vec<&str>>().as_slice() {
                [label, ""] => {
                    let index = i - label_map.len();
                    label_map.insert(label, index + PROGRAM_START);
                }
                [_, _] => {
                    panic!("Invalid instruction at line: {i}");
                }
                _ => (),
            },
            [e1, e2] => match_two_elements((e1, e2), &label_map),
            [_e1, _e2, _e3] => (),
            _ => (),
        };
    }

    println!("Program: {:?}", program);
    println!("Label map: {:?}", label_map);
}

fn match_two_elements(elements: (&&str, &&str), label_map: &HashMap<&str, usize>) {
    println!("elements {:?}", elements);
    match elements {
        (&"jump", address) => {
            let instr = format!("1{:x}", label_map.get(address).unwrap());
            println!("{:?}", instr);
        }
        _ => (),
    }
}

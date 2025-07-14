use std::{collections::HashMap, fs::File, path::Path};
use std::io::prelude::*;

const PROGRAM_START: usize = 512;


pub fn assemble<P>(path: P) -> ()
where
    P: AsRef<Path>,
{
    let file = std::fs::read_to_string(path).expect("Error: file couldnt be found");

    let mut program: Vec<Vec<&str>> = vec![];
    let mut machine_code: Vec<usize> = vec![];
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

    for (i, line) in program.iter().enumerate(){
        match line.as_slice() {
            [label] => match label.split(":").collect::<Vec<&str>>().as_slice(){
                [label, ""] => {
                    let index = i - label_map.len();
                    label_map.insert(label, index + PROGRAM_START);
                }
                _ => (),
            }
            _ => (),
        }
    }

    for (_i, line) in program.iter().enumerate() {
        match line.as_slice() {
            [e1] => {
                machine_code.push(match_one_element(e1));
            }
            [e1, e2] => {
                machine_code.push(match_two_elements((e1, e2), &label_map) as usize)
            },
            [_e1, _e2, _e3] => (),
            _ => (),
        };
    }

    println!("Program: {:?}", program);
    println!("Label map: {:?}", label_map);
    println!("machine_code: {:?}", machine_code);

    write_to_hex();
}

fn match_one_element(element: &str) -> usize{
    match element {
        "clear" => { return 0x00E0; }
        "ret"   => { return 0x00EE; }
        _ => {return 0;}
    }
}

fn match_two_elements(elements: (&str, &str), label_map: &HashMap<&str, usize>) -> u16 {
    //println!("elements {:?}", elements);
    match elements {
        ("jump", address) => {
            println!("instr: {:?}", get_jump_instr(address, label_map));
            return get_jump_instr(address, label_map);
        }
        ("call", address) => {
            return get_call_instr(address, label_map);
        }
        _ => return 0,
    }
}


fn get_call_instr(address: &str, label_map: &HashMap<&str, usize>) -> u16{
    if label_map.contains_key(address){
        let instr = format!("2{:02x}", label_map.get(address).unwrap());
        //let test: u16 = instr.parse().unwrap();
        return u16::from_str_radix(&instr[..], 16).unwrap();
    }
    else{
        let addr: usize = address.parse().unwrap();
        let string = format!("2{:x}",addr);
        //let parsed: u16 = string.parse().unwrap();
        return u16::from_str_radix(&string[..], 16).unwrap();
    }
    
}

fn get_jump_instr(address: &str, label_map: &HashMap<&str, usize>) -> u16{
    if label_map.contains_key(address) {
        let instr = format!("1{:02x}", label_map.get(address).unwrap());
        //let test: u16 = instr.parse().unwrap();
        return u16::from_str_radix(&instr[..], 16).unwrap();
    }
    else{
        let addr: usize = address.parse().unwrap();
        let string = format!("1{:x}",addr); 
        //let instr = format!("1{:x}",addr).parse().unwrap();
        return u16::from_str_radix(&string[..], 16).unwrap();
    }
}

fn write_to_hex(){
    let mut file = File::create("output/output.bin").expect("couldnt create file");
    let _ = file.write_all(&vec![128, 128, 224]);
}

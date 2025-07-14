use std::{collections::HashMap, fs::File, path::Path};
use std::io::prelude::*;

const PROGRAM_START: usize = 512;


pub fn assemble<P>(path: P) -> ()
where
    P: AsRef<Path>,
{
    let file = std::fs::read_to_string(path).expect("Error: file couldnt be found");

    let mut program: Vec<Vec<&str>> = vec![];
    let mut machine_code: Vec<u8> = vec![];
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
                match match_one_element(e1){
                    Some(bytes) => {
                        machine_code.push(bytes.0);
                        machine_code.push(bytes.1);
                    }
                    _ => ()
                }
                //machine_code.push(match_one_element(e1).0);
                //machine_code.push(match_one_element(e1).1);
            }
            [e1, e2] => {
                match match_two_elements((e1, e2), &label_map) {
                    Some(bytes) => {
                        machine_code.push(bytes.0);
                        machine_code.push(bytes.1);
                    }
                    _ => ()
                }
                // machine_code.push(match_two_elements((e1, e2), &label_map).0);
                // machine_code.push(match_two_elements((e1, e2), &label_map).1);
            },
            [_e1, _e2, _e3] => (),
            _ => (),
        };
    }

    println!("Program: {:?}", program);
    println!("Label map: {:?}", label_map);
    println!("machine_code: {:?}", machine_code);

    write_to_hex(&machine_code);
}

fn match_one_element(element: &str) -> Option<(u8, u8)>{
    match element {
        "clear" => { return Some((0x00, 0xE0)); }
        "ret"   => { return Some((0x00, 0xEE)); }
        _ => {return None;}
    }
}

fn match_two_elements(elements: (&str, &str), label_map: &HashMap<&str, usize>) -> Option<(u8,u8)> {
    //println!("elements {:?}", elements);
    match elements {
        ("jump", address) => {
            println!("instr: {:?}", get_jump_instr(address, label_map));
            return Some(get_jump_instr(address, label_map));
        }
        ("call", address) => {
            return Some(get_call_instr(address, label_map));
        }
        _ => return None,
    }
}


fn get_call_instr(address: &str, label_map: &HashMap<&str, usize>) -> (u8, u8){
    if label_map.contains_key(address){
        let instr = format!("2{:02x}", label_map.get(address).unwrap());
        let bytes = get_bytes(instr);
        return (bytes.0, bytes.1);
    }
    else{
        let addr: usize = address.parse().unwrap();
        let string = format!("2{:x}",addr);
        let bytes = get_bytes(string);
        return (bytes.0, bytes.1);
    }
    
}

fn get_bytes(instruction: String) -> (u8, u8){
    let bytes = instruction.split_at(2);
    return (u8::from_str_radix(&bytes.0[..], 16).unwrap(), u8::from_str_radix(&bytes.1[..], 16).unwrap())
}

fn get_jump_instr(address: &str, label_map: &HashMap<&str, usize>) -> (u8, u8){
    if label_map.contains_key(address) {
        let instr = format!("1{:02x}", label_map.get(address).unwrap());
        let bytes = instr.split_at(2);
        //println!("higher byte {:?}, lower byte {:?}", higherbyte.0, higherbyte.1);
        return (u8::from_str_radix(&bytes.0[..], 16).unwrap(),u8::from_str_radix(&bytes.1[..], 16).unwrap());
    }
    else{
        let addr: usize = address.parse().unwrap();
        let string = format!("1{:x}",addr); 
        let bytes = get_bytes(string);
        //let instr = format!("1{:x}",addr).parse().unwrap();
        return (bytes.0, bytes.1);
    }
}

fn write_to_hex(code: &Vec<u8>){
    let mut file = File::create("output/output.bin").expect("couldnt create file");
    let _ = file.write_all(code);
}

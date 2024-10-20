use std::fs::read_to_string;

use crate::Sacados;
pub enum InstanceType {
    Jooken, Pisinger,Standard
}
//pub const FILE : &str = "C:/Users/paulc/Desktop/knapsack/largecoeff/knapPI_1_50_1000000/knapPI_1_50_1000000_1.csv";
pub const FILE : &str = "../../knapsack/largecoeff/knapPI_2_10000_10000000/knapPI_2_10000_10000000_49.csv";
pub fn read_instance (file_path : String, format : InstanceType) -> Sacados {
    match format {
        InstanceType::Standard=> read_standard(file_path),
        InstanceType::Jooken => read_jooken(file_path),
        InstanceType::Pisinger => read_pisinger(file_path),
    }
}
fn read_standard(file_path : String) -> Sacados {
    let mut sac = Sacados::empty();
    let filestring = read_to_string(file_path).unwrap();
    let mut file = filestring.lines();
    let mut first_line = file.next().unwrap().split_ascii_whitespace();
    let n = first_line.next().unwrap().parse().unwrap();
    let w = first_line.next().unwrap().parse().unwrap();
    sac.set_instance_cap(n);
    sac.set_max_capacity(w);
    for line in file {
        let split : Vec<&str> = line.split(',').collect();
        let profit = split[1].parse().unwrap();
        let weight = split[2].parse().unwrap();
        sac.add_item(profit, weight);
    }
    sac
}
fn read_jooken(file_path : String) -> Sacados {
    let mut sac = Sacados::empty();
    let filestring = read_to_string(file_path).unwrap();
    let mut file = filestring.lines();
    let mut first_line = file.next().unwrap().split_ascii_whitespace();
    let n = first_line.next().unwrap().parse().unwrap();
    sac.set_instance_cap(n);
    for (u, line) in file.into_iter().enumerate() {
        if u >= n {
            let w = line.parse().unwrap();
            sac.set_max_capacity(w);
            break;
        }
        let split : Vec<&str> = line.split(',').collect();
        let profit = split[1].parse().unwrap();
        let weight = split[2].parse().unwrap();
        sac.add_item(profit, weight);
    }
    sac
}
fn read_pisinger(file_path : String) -> Sacados {
    let mut sac = Sacados::empty();
    let filestring = read_to_string(file_path).unwrap();
    let mut file = filestring.lines();
    let first_line = file.nth(1).unwrap();
    let n = first_line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
    let first_line = file.nth(0).unwrap();
    let c = first_line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
    let first_line = file.nth(0).unwrap();
    let z : u64 = first_line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
    sac.set_instance_cap(n);
    sac.set_max_capacity(c);
    sac.set_expected(z);
    file.next().unwrap();
    println!("{} {}", n, c);
    for (u, line) in file.into_iter().enumerate() {
        if u >= n {
            let w = line.parse().unwrap();
            sac.set_max_capacity(w);
            break;
        }
        let split : Vec<&str> = line.split(',').collect();
        let profit = split[1].parse().unwrap();
        let weight = split[2].parse().unwrap();
        sac.add_item(profit, weight);
    }
    sac
}
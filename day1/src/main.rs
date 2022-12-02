use std::env;
use std::fs;


fn sum(numbers: &[&str]) -> u32 {
    let mut total: u32 = 0;
    for s in numbers {
        if let Ok(num) = s.parse::<u32>() {
            total += num;
        }
    }
    return total
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Processing file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut groups: Vec<u32> = lines
        .split_inclusive(|l| *l == "")
        .map(|x| sum(x)).collect();
    groups.sort();
    groups.reverse();

    //let max = groups.iter().max().expect("There should be a max here");

    let total3: u32 = groups.iter().take(3).sum();

    println!("{}", total3);

}

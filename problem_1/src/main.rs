use std::fs;


fn find_pair_that_sums(sum: i64, array: &[i64]) -> Option<(i64, i64)> {

    let len = array.len();
    let mut i = 0;
    while i < len {
        let s = array[i];
        let sl = &array[i+1..len];
        for ss in sl {
            if s + ss == sum{
                println!("{}", s);
                println!("{}", ss);
                return Some((s, *ss))
            }
        }
        i += 1
    }
    None
}

fn part1(numbers: &Vec<i64>){
    let len = numbers.len();
    find_pair_that_sums(2020, &numbers[0..len]);
}

fn part2(numbers: &Vec<i64>){

    let len = numbers.len();
    let mut i = 0;
    while i < len {
        let s = numbers[i];
        let x = find_pair_that_sums(2020 - s, &numbers[i..len]);
        match x {
            Some(j) => {
                println!("{}", s);
                return()
            },
            None => {
                i += 1;
                continue;
            }
        };
    }
}

fn main() {

    let contents = fs::read_to_string("input.txt").unwrap();

    let mut numbers: Vec<i64> = Vec::new();
    
    for num in contents.lines() {
        let n = num.parse::<i64>().unwrap();
        numbers.push(n);
    }
    // part1(&numbers)
    part2(&numbers)

}

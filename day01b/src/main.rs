pub fn main() {
    let input: Vec<usize> = include_str!("../input.txt")
    .lines()
    .map(|i| i.parse::<usize>().unwrap())
    .collect();

    let mut solution = 0;
    let mut previous_sum = input[0] + input[1] + input[2];
    for x in 1..input.len() - 2 {
        let current_sum = input[x] + input[x + 1] + input[x + 2];
        if current_sum > previous_sum {
            solution = solution + 1;
        }
        previous_sum = current_sum;
    }

    println!(
        "{}",
        solution
    );
}
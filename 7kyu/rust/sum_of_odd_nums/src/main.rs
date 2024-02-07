fn main() {
    println!("Hello, world!");
}

fn add_odds(starting_number: i64, iterations: i64) -> i64 {
    // recursive function to add the next n odd numbers
    if iterations < 0 {
        return 0;
    }
    return starting_number + add_odds(starting_number + 2, iterations - 1);
}

fn row_sum_odd_numbers(n: i64) -> i64 {
    // first_number = row_count2 - (row_count - 1)
    if n == 1 {
        return 1;
    }
    let leading_num = n.pow(2) - (n - 1);
    println!("leading num: {}", leading_num);

    let sum = add_odds(leading_num, n - 1);
    println!("sum: {}", sum);

    return sum;
}

//              1
//           3     5
//        7     9    11
//    13    15    17    19
// 21    23    25    27    29

#[test]
fn returns_expected1() {
    assert_eq!(row_sum_odd_numbers(1), 1);
}
#[test]
fn returns_expected2() {
    assert_eq!(row_sum_odd_numbers(2), 8);
}
#[test]
fn returns_expected3() {
    assert_eq!(row_sum_odd_numbers(42), 74088);
}

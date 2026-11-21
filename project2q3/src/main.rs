fn main() {
    let input = vec!["yi", "nine", "josh", "ba", "test", "ten"];
    go(&input);
}

const CHINESE: [&str; 11] = [
    "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",
];

const ENGLISH: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

//Return the position if the word in the parameter represents a valid number in the list of chinese words
fn lookup_chinese(word: &str) -> Option<usize> {
    CHINESE.iter().position(|&candidate| candidate == word)
}

//Return the position if the word in the parameter represents a valid number in the list of english words
fn lookup_english(word: &str) -> Option<usize> {
    ENGLISH.iter().position(|&candidate| candidate == word)
}

//Return the position of any word. If the word does not exist then return -1
fn translate(word: &str) -> i32 {
    match lookup_chinese(word) {
        Some(i) => i as i32,
        None => match lookup_english(word) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}

//Create a list, check if the list is not empty,
//translate the head and add it to he list if it is valid,
//and finally append the tail sublist through recursion
fn translate_and_build(nums: &[&str]) -> Vec<i32> {
    let mut new_list: Vec<i32> = Vec::new();

    match nums.iter().as_slice(){
        [] => {},
        [head, tail @ ..] => {
            if translate(head) > -1 {
                new_list.push(translate(head));
            }
            new_list.append(&mut translate_and_build(tail));
        }
    }

    new_list
}

//Simple loop to print the integer numbers, separated by spaces
fn print_translation(numbers: &Vec<i32>) {
    print!("Translation: {} ", numbers[0]);
    for i in 1..numbers.len(){
        print!(" {} ", numbers[i]);
    }
    println!();
}

//Simple loop to print the integer numbers, separated by +'s, with the sum printed last
fn print_addition(numbers: &Vec<i32>) {
    print!("Addition:  {} ", numbers[0]);
    for i in 1..numbers.len(){
        print!("+ {} ", numbers[i]);
    }

    let sum: i32 = numbers.iter().sum();
    println!("= {}", sum);
}

//Simple loop to print the integer numbers, separated by *'s, with the product printed last
fn print_multiplication(numbers: &Vec<i32>) {
    print!("Multiplication:  {} ", numbers[0]);
    for i in 1..numbers.len(){
        print!("* {} ", numbers[i]);
    }

    let product: i32 = numbers.iter().product();
    println!("= {}", product);
}

//Starter function that sends the list to be translated
//and built into a list of i32. Then prints the list
//unchanged, then prints the sum, and then prints the
//product
fn go(input: &[&str]) {
    let numbers: Vec<i32> = translate_and_build(input);
    print_translation(&numbers);
    print_addition(&numbers);
    print_multiplication(&numbers);
}
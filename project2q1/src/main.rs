fn main() {
    let list = vec![12, 77, 42, 893, 100, 101];
    println!("{}", hitchhiker(&list));
    
    let list2 = everyother(list);
    for elem in list2 {
        print!("{} ", elem);
    }
    println!();

    let num = 100;
    goldbach(num);
}

fn hitchhiker(list: &Vec<i32>) -> bool {
    list.contains(&42)
}

fn everyother(mut list: Vec<i32>) -> Vec<i32> {
    let mut new_list: Vec<i32> = vec![];

    match list.as_slice() {
        [] => {}
        [_single] => {}
        [_head,_tail @ ..] => {
            new_list.push(*list.get(1).unwrap());
            new_list.append(&mut everyother(list.split_off(2)));
        }
    }

    new_list
}

fn goldbach(num: i32) {
    if num % 2 == 0 {
        let addend1 = 0;
        let addend2 = num;

        are_two_addends_prime(addend1, addend2);
    } else {
        println!("Error, number is not even")
    }
}

fn are_two_addends_prime(addend1: i32, addend2: i32) {
    if is_prime(addend1) && is_prime(addend2) {
        println!("{} + {} is {}", addend1, addend2, addend1 + addend2);
    } else if addend1 < addend2 {
        are_two_addends_prime(addend1 + 1, addend2 - 1);
    } else {
        println!("Error could not find primes");
    }
}

fn is_prime(num: i32) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        n if prime_helper(n, 2) => true,
        _ => false
    }
}

fn prime_helper(num: i32, i: i32) -> bool {
    match num {
        n if n == i => true,
        n if n % i == 0 => false,
        _ => prime_helper(num, i + 1),
    }
}
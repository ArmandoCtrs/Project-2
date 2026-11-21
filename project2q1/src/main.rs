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

//This function uses the existing contains function on
//the list in the parameter in order to if the
//desired value of 42 exists in the list by returning
//true if it does and false if it doesn't.
fn hitchhiker(list: &Vec<i32>) -> bool {
    list.contains(&42)
}

//This function builds a working list from the list in
//the parameter by using pattern matching. The
//conditions to match are an empty list, a list
//with a single item, and a list with multiple
//items. The empty list and the single item condition
//do nothing andends the function. The multi-item
//condition pushes the head of the tail of the list
//to the working list and the appends to the working
//list the results of the recursive call to the
//function with the tail of the tail as a parameter.
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

//this function checks if the parameter num
//is even. If it is even then it continues
//by searching for the goldbach numbers using
//the are_two_addends_prime prime with the
//original value of num and the number zero
//as parameters.
fn goldbach(num: i32) {
    if num % 2 == 0 {
        let addend1 = 0;
        let addend2 = num;

        are_two_addends_prime(addend1, addend2);
    } else {
        println!("Error, number is not even")
    }
}

//This function expects to get two addends as
//parameters: the original number sent to the
//goldabch function and the number zero. Then
//the function checks if both numbers are prime
//and prints thme if so, indicating the end of
//the search for goldbach addends. However, if
//any of the two values is not prime, then the
//value of num is decremented by one and the
//value of zero is incremented by one. These
//addends (named addend1 and addend2) are then
//used as paramaters to rescursively call the
//function until both are prime. If somehow
//the values become num/2, then an error
//message is printed
fn are_two_addends_prime(addend1: i32, addend2: i32) {
    if is_prime(addend1) && is_prime(addend2) {
        println!("{} + {} is {}", addend1, addend2, addend1 + addend2);
    } else if addend1 < addend2 {
        are_two_addends_prime(addend1 + 1, addend2 - 1);
    } else {
        println!("Error could not find primes");
    }
}

//This value checks if the parameter is a prime
//number by using pattern matching and returns
//a corresponding bool value as a result. The
//smallest cases are covered by default (0 or 1
//are not prime while 2 and 3 are prime). The
//last condition is any number n if the results
//from the prime_helper function are true, which
//then returns a value of true. The last
//condition is any other extraneous value which
//returns false.
fn is_prime(num: i32) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        n if prime_helper(n, 2) => true,
        _ => false
    }
}

//This function checks to see if a number is 
//divisible by any number other than itself
//by checking num's divisibility by i.
//It uses pattern matching by comparing
//three conditions against the value of num. The first
//condition is any value n if n is equal to i, which
//then returns true (this indicates the number is
//divisible by itself, the base case). The other
//condition is any value n if n is wholly divisible
//by i, which returns false (not prime as the number
//num is divisible by a number other than 1 or itself).
//The last conidition is another number which 
//recursively the function with the incremented
//value of num, so as to continuing searching
//if any number between 2 and num can wholly divide
// the number num.
fn prime_helper(num: i32, i: i32) -> bool {
    match num {
        n if n == i => true,
        n if n % i == 0 => false,
        _ => prime_helper(num, i + 1),
    }
}
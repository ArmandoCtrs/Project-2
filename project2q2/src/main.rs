fn main() {
    let result = binary_addition(vec![1, 1, 0, 1], vec![1, 1]);
    println!("Binary Addition result: {:?}", result);

    let result2 = binary_subtraction(vec![1, 1, 0, 1], vec![1, 1]);
    println!("Binary Subtraction result: {:?}", result2);
}

fn int_to_bool(bits: Vec<i32>) -> Vec<bool> {
    let mut new_list: Vec<bool> = vec![];

    match bits.as_slice() {
        [] => Vec::new(),
        [head, tail @ ..] => {
            new_list.push(match head {
                0 => false,
                1 => true,
                _ => panic!("binary bits must be 0 or 1"),
            });
            new_list.append(&mut int_to_bool(tail.to_vec()));
            new_list
        }
    }
}

fn bool_to_int(bits: Vec<bool>) -> Vec<i32> {
    let mut new_list: Vec<i32> = vec![];

    match bits.as_slice() {
        [] => Vec::new(),
        [head, tail @ ..] => {
            new_list.push(match head {
                false => 0,
                true => 1,
            });
            new_list.append(&mut bool_to_int(tail.to_vec()));
            new_list
        }
    }
}


fn do_binary_addition(bits1: Vec<bool>, bits2: Vec<bool>, carry: bool) -> Vec<bool> {
    let mut new_list = Vec::new();

    match (bits1.as_slice(), bits2.as_slice()) {
        ([], []) => {
            new_list.extend(finish_binary_add(Vec::new(), carry));
        }

        (remaining, []) => {
            new_list.extend(finish_binary_add(remaining.to_vec(), carry));
        }

        ([], remaining) => {
            new_list.extend(finish_binary_add(remaining.to_vec(), carry));
        }

        ([head1, tail1 @ ..], [head2, tail2 @ ..]) => {
            let sum = *head1 ^ *head2 ^ carry;

            let new_carry =
                (*head1 && *head2) ||
                (*head1 && carry) ||
                (*head2 && carry);

            new_list.push(sum);

            new_list.extend(do_binary_addition(
                tail1.to_vec(),
                tail2.to_vec(),
                new_carry,
            ));
        }
    }

    new_list
}


fn finish_binary_add(bits: Vec<bool>, carry: bool) -> Vec<bool> {
    let mut new_list: Vec<bool> = vec![];

    match bits.as_slice() {
        [] => {
            if carry {
                new_list.push(true);
            }
        },
        [head, tail @ ..] => {
            match (head, carry) {
                (true, true) => {
                    new_list.push(false);
                    new_list.append(&mut finish_binary_add(tail.to_vec(), true));
                },
                _ => {
                    new_list.push(*head || carry);
                    new_list.append(&mut finish_binary_add(tail.to_vec(), false));
                }
            }
        }
    }

    new_list
}

fn binary_addition(bits1: Vec<i32>, bits2: Vec<i32>) -> Vec<i32> {
    let mut bits1_rev = bits1.clone();
    let mut bits2_rev = bits2.clone();

    bits1_rev.reverse();
    bits2_rev.reverse();

    let bits1_bool = int_to_bool(bits1_rev);
    let bits2_bool = int_to_bool(bits2_rev);

    let mut bool_added: Vec<bool> = do_binary_addition(bits1_bool, bits2_bool, false);
    bool_added.reverse();
    
    bool_to_int(bool_added)
}

fn invert_bits(bits: Vec<bool>) -> Vec<bool> {
    let mut new_list = vec![];

    match bits.as_slice() {
        [] => {},
        [head, tail @ ..] => {
            new_list.push(!head);
            new_list.append(&mut invert_bits(tail.to_vec()));
        }
    }

    new_list
}

fn binary_subtraction(bits1: Vec<i32>, bits2: Vec<i32>) -> Vec<i32> {
    let mut bits1_rev = bits1.clone();
    let mut bits2_rev = bits2.clone();

    bits1_rev.reverse();
    bits2_rev.reverse();

    let mut bits1_bool = int_to_bool(bits1_rev);
    let mut bits2_bool = int_to_bool(bits2_rev);

    if bits1_bool.len() < bits2_bool.len() {
        bits1_bool.resize(bits2_bool.len(), false);
    } else {
        bits2_bool.resize(bits1_bool.len(), false);
    }

    let bits2_bool_inverted = invert_bits(bits2_bool.clone());
    let mut bool_added: Vec<bool> = do_binary_addition(bits1_bool.clone(), bits2_bool_inverted, true);
    bool_added.truncate(bits1_bool.len());
    bool_added.reverse();
    
    bool_to_int(bool_added)
}
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Please enter random numbers, one line at a time");
    
    let mut vec: Vec<i32> = Vec::new();
    
    loop {
        let mut num = String::new();
        
        io::stdin().read_line(&mut num)
            .expect("WHAT");
            
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
            
        println!("You entered: {}", num);
        vec.push(num);
    }
    
    println!("We have a list of {} numbers", vec.len());
    
    vec = merge_sort(vec);
    
    for sorted_num in vec {
        println!("{}", sorted_num);
    }
}

// our recursive merge sort function
fn merge_sort(mut left_vec: Vec<i32>) -> Vec<i32> {
    // Grab the length of the incoming vector
    let vec_length = left_vec.len();
    // If the vector's length is greater than 1
    let mut right_vec = left_vec.split_off(vec_length / 2);
    if left_vec.len() > 1 {
        left_vec = merge_sort(left_vec);
    }
    
    if right_vec.len() > 1 {
        right_vec = merge_sort(right_vec);
    }
    
    // The new vector that will be returned
    let mut returned_vec: Vec<i32> = Vec::new();
    
    // our left and right cursors
    let mut left_cur = 0;
    let mut right_cur = 0;
    
    // Lengths of our vectors
    let left_len = left_vec.len();
    let right_len = right_vec.len();
    
    while left_cur < left_len || right_cur < right_len {
        if right_cur >= right_len {
            // Mash the left
            returned_vec.push(left_vec[left_cur]);
            left_cur += 1;
        } else if left_cur >= left_len {
            // Mash the right
            returned_vec.push(right_vec[right_cur]);
            right_cur += 1;
        } else {
            // Perform the comparison sort
            match left_vec[left_cur].cmp(&right_vec[right_cur]) {
                Ordering::Less => {
                    returned_vec.push(left_vec[left_cur]);
                    left_cur += 1;
                }
                Ordering::Greater => {
                    returned_vec.push(right_vec[right_cur]);
                    right_cur += 1;
                }
                Ordering::Equal => {
                    returned_vec.push(left_vec[left_cur]);
                    left_cur += 1;
                } 
            }
        }
    }

    return returned_vec;
}

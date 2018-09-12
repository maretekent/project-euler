// A prime number is a whole number greater than 1 
// whose only factors are 1 and itself. A factor is a whole numbers that 
// can be divided evenly into another number. The first few 
// prime numbers are 2, 3, 5, 7, 11, 13, 17, 19, 23 and 29. 
// Numbers that have more than two factors are called composite numbers. 
// The number 1 is neither prime nor composite. 

// Euler Problem: 
// ===============
// The number, 197, is called a circular prime because 
// all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31,
//  37, 71, 73, 79, and 97.
// How many circular primes are there below one million?
#[warn(dead_code)]

// fn is_prime(nums: &i32) -> bool {
//     let mut i: i32 = 3;
//     while &i < nums {
//         if nums % &i == 0 {
//             return false
//         }
//         i = i + 2
//     }
//     return true;
// }

// pub fn primes_up_to(limit: u64) -> Vec<u64> {
//     let mut vec: Vec<_> = (2...limit).collect();

//     for p in 2...limit {
//         vec.retain(|&x| x <= p || x % p != 0);
//     }
//     vec
// }

// fn main() {
//     let x = 17;
//     println!("{}",is_prime(&x));
// }

// fn main(){
//     let mut array = vec![1,2,3,4];
//     // let mut array = vec![2,7,9,3,5];
//     generate(array.len(), &mut array);
// }

// fn generate(n : usize, a : &mut Vec<usize>) {
//     if n == 1 {
//         println!("{:?}", a);
//     }
//     else {
//         for i in  0 .. n - 1 {
//             generate(n - 1, a);

//             if n % 2 == 0 {
//                 a.swap(i, n - 1);
//             }
//             else {
//                 a.swap(0, n - 1);
//             }
//         }
//         generate(n - 1, a);
//     }
// }

extern crate itertools; // 0.7.8

use std::iter::FromIterator;
use itertools::free::join;
use std::fmt;

pub struct JoinMyVec {
    myvec : Vec<usize>,
}

impl fmt::Display for JoinMyVec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = fmt.write_str(&join(&self.myvec[..], &""))?;
        Ok(s)
    }
}

fn main() {

    // let join_this = JoinMyVec {
    //     myvec : vec![2, 3, 4, 5],
    // };
    // println!("{}", join_this);
    
    // let x = i.to_string();
    // for y in x.chars(){
    //   let z = (y.to_string()).parse::<i32>().unwrap(); 
    // }
    
    // let mut array = vec![1,2,3,4];
    let mut array = vec![1,9,7];

    // let v = vec!['1', '2', '3', '4'];
    // let s = String::from_iter(&v);
    // vs
    // let s: String = v.into_iter().collect();
    // println!("{}", s);
    
    generate(array.len(), &mut array);
}

fn generate(n : usize, a : &mut Vec<usize>) {
    if n == 1 {
        let join_this = JoinMyVec {
            myvec : a.to_vec(),
        };
        println!("{}", join_this);
    
        // println!("{:?}", a);
    }
    else {
        for i in  0 .. n - 1 {
            generate(n - 1, a);

            if n % 2 == 0 {
                a.swap(i, n - 1);
            }
            else {
                a.swap(0, n - 1);
            }
        }
        generate(n - 1, a);
    }
}
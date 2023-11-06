// 6.1 mimicking pointers and references
fn basic(){
    static B: [u8; 5] = [1,2,3,4,5];
    static C: [u8; 5] = [10,11,12,13,14];

    let a = 42;
    let b = &B;
    let c = &C; 
    // here noth b and c point to the first element of the array B and C respectively

    println!("a: {}, b: {:p}, c: {:p}", a, b, c); // {:P} asks the compiler to format the variable as a pointer and print its memory adddress
    // println!("a: {}, b: {}, c: {}", a, b, c);

}

use std::mem::size_of;  
fn comparing_references_and_box(){
    static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
    static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

    let a : usize = 42;
    let b : &[u8;10] = &B; // reference to an array of 10 bytes
    let c : Box<[u8]> = Box::new(C);    // box byte slice. when we place values inside the box, the ownership moves to owner of the box

    println!("a (an unsigned integer):");
    println!(" location: {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>());
    println!(" value: {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!(" location: {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8; 10]>());
    println!(" points to: {:p}", b);
    println!();

    println!("c (a 'box' for C):");
    println!(" location: {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
    println!(" points to: {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!(" location: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!(" location: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {:?}", C);

}

fn raw_pointers(){
    // pointers without inherent rust safety features
    // for eg, raw pointers can be NULL
    // Normal ryst pointers compile down to raw-pointers, which means it is possible to access the performance of rae pointers without explicitly using them

    let a : i64 = 432;
    let a_ptr = &a as *const i64; // can also write *mut i64, which will be for mutable raw pointers

    println!("a: {} ({:p})", a, a_ptr);

    // raw pointers point to the starting byet fo the type T and always know the width of the type T

    
}

fn main() {
    // basic();
    comparing_references_and_box();
}
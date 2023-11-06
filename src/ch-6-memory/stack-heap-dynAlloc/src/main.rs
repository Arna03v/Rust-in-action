// stack needs to know size of ots variables at compile time, if in doubt use types that implement 'Sized'
fn stack(){
    // entries in the stack acalled stack frames. These are created as function calls are made
    // stack pointer
    
    // Stack frames contain a function’s state during the call. When a function is called
    // within a function, the older function’s values are effectively frozen in time.

    //  Stacl frame contains space for function args, pointer to the priginal call site & local variables

    // stack is fast as all function variables are side by side in memory, which speeds up access
}

// working with functions that noyl accept STring or str

// fn is_strong(password: String) -> bool {
// password.len() > 5
// }

// is_strong can only accept String. That means that the following code won’t work:

// let pw = "justok";
// let is_strong = is_strong(pw);

// But generic code can help. In cases where read-only access is required, use functions
// with the type signature fn x<T: AsRef<str>> (a: T) rather than fn x(a: String).
// The fairly unwieldy type signature reads “as function x takes an argument password
// of type T, where T implements AsRef<str>.” Implementors of AsRef<str> behave
// as a reference to str even when these are not.

// Here is the code snippet again for the previous listing, accepting any type T that
// implements AsRef<str>. It now has the new signature in place:

// fn is_strong<T: AsRef<str>>(password: T) -> bool {
// password.as_ref().len() > 5
// }

// When read-write access to the argument is required, normally you can make use of
// AsRef<T>'s sibling trait AsMut<T>

fn heap(){
    // area of program for types whose size is not known at compile time
    // That is, either some types grow/shrink like String or Vec<T>. OR the types which are unable to tell the compiler 
    // how much size to allocate even though its size doesnt vary

    // Read the virtual memory section after this to complete the understanding of what heap actually is

    // Variables on the heap must be accessed by a pointer
    let a: i32 = 40; // on the stack
    let b: Box<i32> = Box::new(60); // on the heap  
    // let result = a + b;
    let result = a + *b; // de-referncing b to be used

    fn allocating_and_deallocating();

}

use std::mem::drop; // brings manual drop to local scope
fn allocating_and_deallocating(){
    let a = Box::new(1); // allocates a variable on the heap
    let b = Box::new(1);
    let c = Box::new(1);

    // Something that has been boxed lives on the heap, with a pointer to it on the
    // stack. This is demonstrated in the first column of figure 6.5, where the number
    // 0x100 at address 0xfff points to the value 1 at address 0x100. However, no
    // actual box of bytes encloses a value, nor is the value hidden or concealed in
    // some way

    let res1 = *a + *b + *c; // = 3
    drop(a);

    // The lifetime of the variable
    // a has ended at this point.
    // Accessing this memory
    // address is now invalid. Its data
    // will still be there on the stack,
    // but it's to accessimpossible
    // it within safe Rust. Its corresponding space in the heap is freed for the next heap variable to be used

    let d = Box::new(1);
    let res2 = *b + *c + *d; // = 3



}

// a function to multiple elemnets of vector * 2

fn dynamic_allocation_of_memory(){
    
}
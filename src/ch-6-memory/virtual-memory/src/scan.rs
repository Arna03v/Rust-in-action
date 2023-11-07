// This section's example would have us build the core of a tool that is capable of inspecting andmodifying values of a running program

//  Page—A fixed-size block of words of real memory. Typically 4 KB in size for
// 64-bit operating systems.
//  Word—Any type that is size of a pointer. This corresponds to the width of the
// CPU’s registers. In Rust, usize and isize are word-length types.
//  Page fault—An error raised by the CPU when a valid memory address is
// requested that is not currently in physical RAM. This signals to the OS that at
// least one page must be swapped back into memory.
//  Swapping—Migrating a page of memory stored temporarily on disk from main
// memory upon request.
//  Virtual memory—The program’s view of its memory. All data accessible to a
// program is provided in its address space by the OS.
//  Real memory—The operating system’s view of the physical memory available
// on the system. In many technical texts, real memory is defined independently
// from physical memory, which becomes much more of an electrical engineer-
// ing term.
//  Page table—The data structure maintained by the OS to manage translating
// from virtual to real memory.

//  Segment—A block within virtual memory. Virtual memory is divided into blocks
// to minimize the space required to translate between virtual and physical
// addresses.
//  Segmentation fault—An error raised by the CPU when an illegal memory address
// is requested.
//  MMU—A component of the CPU that manages memory address translation.
// Maintains a cache of recently translated addresses (called the TLB), which
// stands for the translation lookaside buffer, although that terminology has fallen
// from fashion.

// this module focuses on having a process scan its own memory byte by byte

static GLOBAL: i32 = 1000;             // <1>

fn noop() -> *const i32 {
    let noop_local = 12345;            // <2>
    &noop_local as *const i32          // <3>
}


fn main(){
    let mut n_nonzero = 0;
    for i in 1..100000{ // if we start from 0 program will crash as it attempting to derefernce a NULL pointer
        let ptr = i as *const u8; // a raw pointer of type u8 to inspect raw memory addresses
        let byte_at_addr = unsafe { *ptr }; // reading the value being pointed to

        if byte_at_addr != 0 {
            n_nonzero += 1;
        } // move forward till we hit a 0

    } // this loop crashes, giving a segmentation fault. As we are trying to access memory that is not ours


    printnln!("number of non-zero bytes is : {}", n_nonzero);

    // so instead lets try to print addresses of data str in our code
    let local_str = "a";               // <4>
    let local_int = 123;               // <4>
    let boxed_str = Box::new('b');     // <4>
    let boxed_int = Box::new(789);     // <4>
    let fn_int = noop();               // <4>

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);

}

// Some memory addresses are illegal. The OS will shut your program down if it attempts
// to access memory that is out of bounds.

// Memory addresses are not arbitrary. Although values seem to be spread quite far
// apart within the address space, values are clustered together within pockets

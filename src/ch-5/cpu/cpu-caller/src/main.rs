// allows us to call function
// validates that functions are also data


// some additional opcodes are needed to build functions
// CALL opcode ( 0x2nnn, where nnn is a memory address)
// RETURN opcode (0x00EE) set the pogram counter to the memory address of the previous CALL opcode

// we have the STACK , mmemory to store these addresses. Each CALL sets an addr in the stack by incrementing the stack pointer and writing nnn to the location
// each RETURN removes the top address from the stack by decrmenting the stack pointer

strcut CPU{
    registers : [u8; 16],
    program_counter : usize,
    memory : [u8; 4096],
    stack : [u16; 16], // max height of stakc is 16
    stack_pointer : usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
  
        op_byte1 << 8 | op_byte2
    } // same as in cpu-mulitplier

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let nnn = opcode & 0x0FFF;
            // let kk  = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (  0,   0,   0,   0) => { return; }, // no-op
                (  0,   0, 0xE, 0xE) => self.ret(), // return
                (0x2,   _,   _,   _) => self.call(nnn), /// call
                (0x8,   _,   _, 0x4) => self.add_xy(x, y), // addition
                _                    => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
  
        let (val, overflow) = arg1.overflowing_add(arg2); // returns overflow = true if addition overflows

        self.registers[x as usize] = val;
  
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main(){
    // say we want to perform addition twice and then return
    // we break up the opcodes into bytes and store it in memory

    // let mut memory : [u8; 4096] = [0; 4096];
    // let mem = &mut memory;
    
    // let add_twice = [
    //     0x80, 0x14,
    //     0x80, 0x14,
    //     0x00, 0xEE,
    // ]; // storing the opcodes as a slices

    // mem[0x100..0x106].copy_from_slice(&add_twice); // storing these operation in those memory locations
    // println!("Storing opcodes in memory is a success\n");
    // println!("{:?}\n", &mem[0x100..0x106]); // prints [128, 20, 128, 20, 0, 238]

    // performing 5 + (10 * 2) + (10 * 2) 

    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    // another way to add opcodes to the memory other than the commented method
    mem[0x000] = 0x21; mem[0x001] = 0x00;    
    mem[0x002] = 0x21; mem[0x003] = 0x00;    
    mem[0x004] = 0x00; mem[0x005] = 0x00;    

    mem[0x100] = 0x80; mem[0x101] = 0x14;    
    mem[0x102] = 0x80; mem[0x103] = 0x14;    
    mem[0x104] = 0x00; mem[0x105] = 0xEE; 

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);



}




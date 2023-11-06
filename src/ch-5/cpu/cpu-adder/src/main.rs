struct CPU{
    current_operation : u16,
    registers : [u8; 2] // 2 registers each of size u8
}

// we need to do the following 
// 1. initialize a CPU
// 2. load the values in the registers
// 3. Load the addition opcode into current_operation
// 4. perform the operation

impl CPU{
    fn read_opcode(&self) -> u16{
        self.current_operation // reads and returns the opcode
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8; 
        let x = ((opcode & 0x0F00) >> 8) as u8; 
        let y = ((opcode & 0x00F0) >> 4) as u8; 
        let d = ((opcode & 0x000F) >> 0) as u8; 

        match (c,x,y,d) {
            (0x8, _, _, 0x4) => self.add_xy(x,y),
            _ => todo!("opcode {:04x}", opcode),
        }

    }

    fn add_xy(&mut self, x : u8, y : u8) {
        self.registers[x as usize] += self.registers[y as usize]
    }

}

fn main(){
    let mut cpu = CPU {
        current_operation : 0, // a no-op doing nothing for now
        registers : [0; 2]
    };

    cpu.current_operation = 0x8014; // 8 - involving 2 regs, 0 - to reg0, 1 - reg1, 4 - addition
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("Addition is succesful, the value of the sum is : {}", cpu.registers[0]);

}
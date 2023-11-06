// can perform mulitple operation unlike cpu-adder


struct CPU {
    registers: [u8; 16], // for calculations
    position_in_memory: usize, // special purpose register. program counter. indicates which instr to execute next
    memory: [u8; 0x1000], // system's memory, for convineince. 0x1000 = 4096
}

impl CPU{
    fn read_opcode(&self) -> u16{
        let PC = self.position_in_memory;
        let op_byte1 = self.memory[PC] as u16;
        let op_byte2 = self.memory[PC+1] as u16;

        op_byte1 << 8 | op_byte2 // left 8 are opbyte1 and the right 8 are opbyte2
    }

    fn run(&mut self){
        loop{
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0)     => { return; },        
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _                => todo!("opcode {:04x}", opcode),
            }
        }
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
    let mut cpu = CPU{
        registers: [0; 16],
        position_in_memory: 0,
        memory: [0; 0x1000],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;               
    cpu.registers[3] = 10;  

    let mut mem = cpu.memory;

    mem[0] = 0x80; mem[1] = 0x14;        
    mem[2] = 0x80; mem[3] = 0x24;        
    mem[4] = 0x80; mem[5] = 0x34; 

    cpu.run();

    assert_eq!(cpu.registers[0], 35);


}



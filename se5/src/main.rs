type pu16 = usize;

struct CPU {
  memory: [u8; 65536],
  stack_size: u16,
  pointer_instruction: pu16,
  pointer_stack: pu16,
  registers: [u8; 16],
}

impl CPU {
  fn stack_busy(&self) -> u16 {
    (self.memory.len() - self.pointer_stack) as u16
  }
  fn stack_available(&self) -> u16 {
    self.stack_size - self.stack_busy()
  }

  fn stack_push(&mut self, data: u8) -> () {
    if self.stack_available() <= 0 {
      panic!("stack overflow")
    }
    self.memory[self.pointer_stack] = data;
    self.pointer_stack -= 1;
  }
  fn stack_pop(&mut self) -> u8 {
    if self.stack_busy() <= 0 {
      panic!("segmentation fault")
    }
    self.pointer_stack += 1;
    self.memory[self.pointer_stack]
  }

  fn read_opcode(&mut self) -> u8 {
    let opcode = self.memory[self.pointer_instruction];
    self.pointer_instruction += 1;
    opcode
  }

  fn read_arg(&mut self) -> u8 {
    self.read_opcode()
  }

  fn read_arg2(&mut self) -> (u8, u8) {
    (self.read_arg(), self.read_arg())
  }

  fn load_memory(&mut self, pos: pu16, memory: &[u8]) -> () {
    let len = memory.len();
    self.memory[pos..(pos + len)].copy_from_slice(memory)
  }

  fn set_register(&mut self, pos: pu16, data: u8) -> () {
    self.registers[pos] = data
  }

  fn run(&mut self) {
    loop {
      let opcode = self.read_opcode();
      match opcode {
        END => {
          println!("CALL END PROGRAM");
          return;
        }
        MOV => {
          let (to, from) = self.read_arg2();
          let from_value = self.registers[from as usize];
          self.set_register(to as usize, from_value)
        }
        ADD => {
          let from = self.read_arg();
          let from_value = self.registers[from as usize];
          let to_value = self.registers[RA as usize];
          self.set_register(RA as usize, to_value + from_value)
        }
        PUSH => {
          let from = self.read_arg();
          let from_value = self.registers[from as usize];
          self.stack_push(from_value);
        }
        POP => {
          let from = self.read_arg();
          let value = self.stack_pop();
          self.set_register(from as usize, value)
        }
        CALL => {
          let (l, r) = self.read_arg2();
          let pointer: pu16 = (((l as usize) << 8) | r as usize);
          println!(
            "{}, {}, {}",
            self.pointer_instruction,
            ((self.pointer_instruction) & 0xFF) as u8,
            ((self.pointer_instruction >> 8) & 0xFF) as u8
          );
          self.stack_push((self.pointer_instruction & 0xFF) as u8);
          self.stack_push(((self.pointer_instruction >> 8) & 0xFF) as u8);
          self.pointer_instruction = pointer;
        }
        RET => {
          let l = self.stack_pop() as usize;
          let r = self.stack_pop() as usize;
          let pointer = (l << 8) | r;
          self.pointer_instruction = pointer;
        }
        PRINT => {
          let from = self.read_arg();
          let value = self.registers[from as usize];
          println!("CALL COMMAND PRINT REGISTER #{} = {}", from, value)
        }
        _ => {
          panic!("unknown command")
        }
      }
    }
  }
}

const R1: u8 = 0;
const R2: u8 = 1;
const R3: u8 = 2;
const R4: u8 = 3;
const R5: u8 = 4;
const R6: u8 = 5;
const R7: u8 = 6;
const R8: u8 = 7;
const R9: u8 = 8;

const RA: u8 = 9;
const RB: u8 = 10;
const RC: u8 = 11;
const RD: u8 = 12;
const RE: u8 = 13;
const RF: u8 = 14;
const RG: u8 = 15;

const END: u8 = 0;
const PRINT: u8 = 1;

const MOV: u8 = 2;
const ADD: u8 = 3;

const CALL: u8 = 5;
const RET: u8 = 6;

const PUSH: u8 = 10;
const POP: u8 = 11;

fn main() {
  let mut cpu = CPU {
    stack_size: 512,
    pointer_instruction: 0,
    pointer_stack: 65535,
    memory: [0; 65536],
    registers: [0; 16],
  };

  let function_sum_vec = vec![
    PRINT, RC, PUSH, RA, ADD, RB, ADD, RC, MOV, RG, RA, POP, RA, RET,
  ];
  let function_sum: &[u8] = function_sum_vec.as_slice();

  let program_vec: Vec<u8> = vec![
    MOV, RA, R1, ADD, R2, MOV, R5, RA, PRINT, R5, MOV, RA, R5, MOV, RB, R8,
    MOV, RC, R9, PRINT, RA, PRINT, RB, PRINT, RC, CALL, 0x07, 0xd0, PRINT, RG,
    END
  ];
  let program: &[u8] = program_vec.as_slice();

  cpu.load_memory(0, program);
  cpu.load_memory(2000, function_sum);
  cpu.set_register(R1 as usize, 7);
  cpu.set_register(R2 as usize, 88);
  cpu.set_register(R9 as usize, 100);
  cpu.set_register(R8 as usize, 14);
  cpu.run();
}

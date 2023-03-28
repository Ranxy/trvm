use std::io::Read;
use std::{fs::File, io::BufReader};

enum OpCode {
    Cst,
    Add,
    Mul,
    Var,
    Pop,
    Swap,
    Call,
    Ret,
    IfZero,
    Goto,
    End,
}

impl From<i32> for OpCode {
    fn from(value: i32) -> Self {
        match value {
            0 => OpCode::Cst,
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Var,
            4 => OpCode::Pop,
            5 => OpCode::Swap,
            6 => OpCode::Call,
            7 => OpCode::Ret,
            8 => OpCode::IfZero,
            9 => OpCode::Goto,
            10 => OpCode::End,
            n => panic!("Wrong OpCode {}", n),
        }
    }
}

pub struct Vm {
    code: Vec<i32>,
    stack: [i32; 400],
    sp: i32,
    pc: i32,
}

impl Vm {
    pub fn new(code: Vec<i32>, pc: i32) -> Self {
        Vm {
            code,
            stack: [0; 400],
            sp: 0,
            pc,
        }
    }

    pub fn new_from_file(file_path: &str) -> std::io::Result<Self> {
        let code = read_file_as_i32_vec(file_path)?;
        Ok(Vm {
            code,
            stack: [0; 400],
            sp: 0,
            pc: 0,
        })
    }

    fn get_code(&self, idx: i32) -> i32 {
        unsafe {
            let v = self.code.get_unchecked(idx as usize);
            *v
        }
    }

    fn push_stack(&mut self, x: i32) {
        self.stack[self.sp as usize] = x;
        self.sp += 1
    }

    fn pop_stack(&mut self) -> i32 {
        self.sp = self.sp - 1;
        self.stack[self.sp as usize]
    }

    fn insert_data_to_pos(&mut self, pos: usize, data: &[i32]) {
        let need_copy = &self.stack[pos..self.sp as usize].to_vec();
        let copy_pos = pos + data.len();

        if !need_copy.is_empty() {
            let mut i = need_copy.len() - 1;
            while i > 0 {
                self.stack[copy_pos + i] = need_copy[i];
                i -= 1;
            }
            self.stack[copy_pos] = need_copy[0];
        }

        for (idx, value) in data.iter().enumerate() {
            self.stack[pos + idx] = *value;
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut stop = false;

        while !stop {
            let op_code = self.get_code(self.pc);
            //println!(">>>Op: {}, PC: {}, SP: {}",op_code,self.pc,self.sp);

            match OpCode::from(op_code) {
                OpCode::Cst => {
                    let i = self.get_code(self.pc + 1);
                    self.push_stack(i);
                    self.pc += 2;
                }
                OpCode::Add => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a + b);
                    self.pc += 1;
                }
                OpCode::Mul => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a * b);
                    self.pc += 1;
                }
                OpCode::Var => {
                    let i = self.get_code(self.pc + 1);
                    let vardata = self.stack[(self.sp - i - 1) as usize];
                    self.push_stack(vardata);
                    self.pc += 2;
                }
                OpCode::Pop => {
                    self.pop_stack();
                    self.pc += 1;
                }
                OpCode::Swap => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a);
                    self.push_stack(b);
                    self.pc += 1;
                }
                OpCode::Call => {
                    let offset = self.get_code(self.pc + 1);
                    let arity = self.get_code(self.pc + 2);
                    let next_pc = self.pc + offset;
                    let data = vec![self.pc + 3];
                    self.insert_data_to_pos((self.sp - arity) as usize, data.as_slice());
                    self.sp += 1;
                    self.pc = next_pc;
                }
                OpCode::Ret => {
                    let arity = self.get_code(self.pc + 1);
                    let res = self.pop_stack();
                    self.sp -= arity;
                    let next_pc = self.pop_stack();
                    self.push_stack(res);
                    self.pc = next_pc;
                }
                OpCode::IfZero => {
                    let offset = self.get_code(self.pc + 1);
                    let to = self.pc + offset;
                    let cond = self.pop_stack();
                    if cond == 0 {
                        self.pc = to
                    } else {
                        self.pc += 2
                    }
                }
                OpCode::Goto => {
                    let offset = self.get_code(self.pc + 1);
                    self.pc += offset;
                }
                OpCode::End => stop = true,
            }
        }

        self.pop_stack()
    }
}

fn read_file_as_i32_vec(file_name: &str) -> std::io::Result<Vec<i32>> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let mut vec = Vec::new();
    for i in 0..bytes.len() / 4 {
        let j = i * 4;
        let n = ((bytes[j] as u32) << 24)
            | ((bytes[j + 1] as u32) << 16)
            | ((bytes[j + 2] as u32) << 8)
            | (bytes[j + 3] as u32);
        vec.push(n as i32);
    }
    Ok(vec)
}

#[cfg(test)]
mod test {
    use super::read_file_as_i32_vec;
    use super::Vm;

    #[test]
    fn read_file() {
        let res = read_file_as_i32_vec("./testdata/fib27");
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn run_file() {
        let res = Vm::new_from_file("./testdata/fib27");

        let mut vm = res.unwrap();

        let res = vm.run();
        println!("RES: {}", res);
        assert_eq!(res, 196418)
    }
}

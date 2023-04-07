use std::io::Write;

use crate::vm;

pub struct Repl<'a> {
    vm: &'a mut vm::Vm,

    show_status: bool,
    show_stack: bool,
}

impl<'a> Repl<'a> {
    pub fn new(vm: &'a mut vm::Vm) -> Repl {
        Repl {
            vm,
            show_status: false,
            show_stack: false,
        }
    }

    fn line_print() {
        print!(">");
        std::io::stdout().flush().unwrap();
    }

    fn read_line(&self) -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        buf
    }

    fn process_input(&mut self, input: String) -> bool {
        let input_str = input.trim();
        match input_str {
            "n" => self.vm.exec_op(),
            "c" => {
                self.vm.run();
                true
            }
            "#show_status" => {
                println!("ShowStatus: {}", !self.show_status);
                self.show_status = !self.show_status;
                false
            }
            "#show_stack" => {
                println!("ShowStack: {}", !self.show_stack);
                self.show_stack = !self.show_stack;
                false
            }
            "#show_code" | "l" | "list" => {
                println!("{}", self.vm.show_code_with_data());
                false
            }
            "dis" => {
                self.vm.print_code_all();
                false
            }

            _ => false,
        }
    }

    pub fn start(&mut self) {
        loop {
            Self::line_print();
            let input = self.read_line();
            let stop = self.process_input(input);
            if stop {
                let res = self.vm.get_result();
                println!("Result: {}", res);
                return;
            }
            if self.show_status {
                println!("{}", self.vm.show_exec_status());
            }
            if self.show_stack {
                println!("{}", self.vm.show_stack())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_match() {
        let si = String::from("si");
        match si.as_str() {
            "si" => {
                println!(")__ si")
            }
            f => {
                println!("FF: {}", f)
            }
        }
    }

    #[test]
    fn repl_run() {
        let mut vm = crate::vm::Vm::new_from_file("./testdata/fib27").unwrap();

        let mut repl = super::Repl::new(&mut vm);

        repl.start()
    }
}

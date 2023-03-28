use std::env;

mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("Need exec code file name args");
        return
    }

    let file_name = &args[1];

    let mut vm = vm::Vm::new_from_file(file_name).unwrap();
    let res = vm.run();

    println!("RES: {}", res)
}

use clap::Parser;

mod repl;
mod vm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File name of the exec code
    #[arg(short, long)]
    file_name: String,

    /// sleep ms after exec one op
    #[arg(short = 'm', long, default_value_t = 0)]
    sleep_ms: u64,
    /// TODO: exec code single step
    #[arg(short = 's', long)]
    single_step: bool,
}

fn main() {
    let args = Args::parse();

    let mut vm = vm::Vm::new_from_file(&args.file_name).unwrap();
    vm.set_sleep_ms(args.sleep_ms);

    if args.single_step {
        let mut repl = repl::Repl::new(&mut vm);
        repl.start();
    } else {
        vm.run();
        let res = vm.get_result();
        println!("RES: {}", res)
    }
}

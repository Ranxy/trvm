use clap::Parser;

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

    println!("file_name: {}", args.file_name);
    println!("sleep_ms: {}", args.sleep_ms);
    println!("single_step: {}", args.single_step);

    let mut vm = vm::Vm::new_from_file(&args.file_name).unwrap();
    vm.set_sleep_ms(args.sleep_ms);

    let res = vm.run();

    println!("RES: {}", res)
}

use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(Debug, StructOpt)]
#[structopt(name = "sicpgen")]
struct SicpGen {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Start(Options),
    Test(Options),
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    exercise: String,

    #[structopt(parse(from_os_str), default_value=".")]
    root: std::path::PathBuf,
}

#[paw::main]
fn main(args: SicpGen) -> Result<(), ExitFailure> {
    let cmd = &args.cmd;
    match cmd {
        Command::Start(opts) => sicpgen::generate(&opts.root, &opts.exercise)?,
        Command::Test(opts) => sicpgen::test(&opts.root, &opts.exercise)?,
    };

    Ok(())
}

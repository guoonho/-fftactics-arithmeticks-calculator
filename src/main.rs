use structopt::StructOpt;
mod arithmeticks;

#[derive(StructOpt)]
struct Cli {
    state: String
}

fn main() {
    let _args = Cli::from_args();

    arithmeticks::validate(&_args.state);
    arithmeticks::calculate(&_args.state);
}

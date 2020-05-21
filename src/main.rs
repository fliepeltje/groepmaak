use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Create sized groups from a list of names")]
struct Opt {
    #[structopt(parse(from_os_str), help = "File with all names to place into groups separated by breakpoints")]
    names_file: PathBuf,

    #[structopt(help = "Size of groups to made")]
    group_size: u8,

    #[structopt(parse(from_os_str), help = "Output file for groups, stdout if not specified", short = "out", long = "output-file")]
    output: Option<PathBuf>
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}

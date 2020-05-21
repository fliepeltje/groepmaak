use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Create sized groups from a list of names")]
struct Opt {
    #[structopt(
        parse(from_os_str),
        help = "Plaintext file with all names to place into groups separated by new lines"
    )]
    names_file: PathBuf,

    #[structopt(help = "Size of groups to made")]
    group_size: u8,

    #[structopt(
        parse(from_os_str),
        help = "Output file for groups, stdout if not specified",
        long = "out"
    )]
    output: Option<PathBuf>,
}

fn create_name_vector(path: &PathBuf) -> Vec<String> {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    let mut names: Vec<String> = file_content.lines().map(|s| s.to_owned()).collect();
    names.shuffle(&mut thread_rng());
    names
}

fn create_groups(names: Vec<String>, group_size: &u8) -> Vec<Vec<String>> {
    let groups: Vec<Vec<String>> = names
        .chunks(group_size.to_owned().into())
        .map(|v| v.to_vec())
        .collect();
    groups
}

fn format_groups(groups: Vec<Vec<String>>) -> String {
    let groups: Vec<String> = groups
        .iter()
        .enumerate()
        .map(|(i, g)| format_single_group(g.to_vec(), i))
        .collect();
    let string = groups.join("\n\n");
    string
}

fn format_single_group(group: Vec<String>, group_number: usize) -> String {
    let mut body = vec![format!("Group {}:", group_number + 1)];
    body.extend(group);
    let group_string = body.join("\n");
    group_string
}

fn main() {
    let opt = Opt::from_args();
    let ordered_list = create_name_vector(&opt.names_file);
    let shuffled_list = create_groups(ordered_list, &opt.group_size);
    let group_output = format_groups(shuffled_list);
    match opt.output {
        Some(path) => {
            fs::write(&path, &group_output).expect("Could not write to file");
            println!("Succesfully wrote to file.");
            println!("{}", group_output);
        }
        None => println!("{}", group_output),
    }
}

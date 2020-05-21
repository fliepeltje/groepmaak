## Description
A minimal command line tool that takes in a list of names and groups them randomly according to a specified group size.

## Installation
The easiest method is to use cargo. You can install directly from the repo: 
`cargo install --git https://github.com/fliepeltje/groepmaak`

## Usage
There are 2 ways to use this tool. The first is to only print the groups to `stdout`. To do this run `groepmaak <input_file> <group_size>` where the input file is a plaintext file where each name is on a new line and the group size is an integer. 
The other way is to also write the created groups to a file, to do this you must specify the `--out` flag as `--out <output_file>`.
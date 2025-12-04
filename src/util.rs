
use std::fs::read_to_string;
use std::path::Path;
use std::env;

pub fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn get_input_path() -> std::path::PathBuf {
    let exe = env::current_exe().unwrap();
    let exe_name = exe.file_name().unwrap();
    let exe_dir = exe.parent().unwrap();
    return exe_dir.parent().unwrap().parent().unwrap().join("inputs").join(exe_name);
}

pub fn cartestian_product<I, J>(iter1: I, iter2: J) -> impl Iterator<Item=(I::Item, J::Item)>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Copy,
    J::Item: Copy,
{
    iter1.flat_map(move |i| iter2.clone().map(move |j| (i, j)))
}

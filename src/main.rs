extern crate rand;

use std::env;
use std::io;
use std::io::{stdin};
use std::io::prelude::*;
use std::fs::{self, DirEntry, File};
use std::path::Path;

use rand::{thread_rng, Rng};

fn main() {
    // Are you save?
    println!("If you have save file? y/n");
    let mut ans = String::new();
    stdin().read_line(&mut ans).expect("Not correct");
    ans.retain(|x| x != '\n');

    let mut result: Vec<String> = Vec::new();

    // No save file
    if ans == "n" {
        // Input file directory
        println!("Input root directory of mp3");
        let mut root = String::new();
        stdin().read_line(&mut root).expect("Not correct");
        root.retain(|x| x != '\n');

        let dir = Path::new(&root);

        result = find_mp3(dir).unwrap();
        thread_rng().shuffle(&mut result);
        write_mp3_list("save_0.txt", &result);
        println!("Write complete");
        println!("{:?}", result);
    } else if ans == "y" {
        // Save file
        let name = Path::new("save_0.txt");
        let mut f = File::open(name).expect("File not found");
        let mut temp = String::new();

        f.read_to_string(&mut temp)
            .expect("Can't open this file");

        result = temp.split('\n')
            .map(|x| x.to_string()).collect();
    }

    let length = result.len();
    let mut result2: Vec<String> = Vec::new();
    let mut i: usize = 0;

    let mut ans3 = String::new();
    println!("If you have temporary saved file? y/n");
    stdin().read_line(&mut ans3).expect("Not correct answer");
    ans3.retain(|x| x != '\n');

    if ans3 == "y" {
        let temp_file = Path::new("temp_1.txt");
        let log_file = Path::new("log.txt");
        let mut f = File::open(temp_file).expect("File not found");
        let mut g = File::open(log_file).expect("File not found");
        let mut temp = String::new();
        let mut temp2 = String::new();

        f.read_to_string(&mut temp)
            .expect("Can't open this file");

        g.read_to_string(&mut temp2)
            .expect("Can't open this file");

        result2 = temp.split('\n')
            .map(|x| x.to_string()).collect();

        temp2.retain(|x| x != '\n');

        i = temp2.parse().unwrap();
    }

    while i < length / 2 - 1 {
        let x = &result[2*i];
        let y = &result[2*i+1];
        println!("{}  vs  {}", x, y);
        let mut ans = String::new();
        stdin().read_line(&mut ans).expect("Not correct");
        ans.retain(|x| x != '\n');

        if ans == "1" {
            result2.push(x.to_string());
            i += 1;
        } else if ans == "2" {
            result2.push(y.to_string());
            i += 1;
        } else if ans == "3" {
            result2.push(x.to_string());
            result2.push(y.to_string());
            i += 1;
        } else if ans == "s" {
            write_mp3_list("temp_1.txt", &result2);
            let temp_write = format!("{}", i);
            write_mp3_list("log.txt", &vec![temp_write]);
            i += 1;
        } else if ans == "4" {
            i += 1;
        } else {
            continue;
        }
    }

    write_mp3_list("save_1.txt", &result2);
}

fn find_mp3(dir: &Path) -> io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let mut temp_path = path.to_string_lossy().to_string();
                let temp_dir = dir.to_str().unwrap().len();
                for _i in 0 .. temp_dir {
                    temp_path.remove(0);
                }
                temp_path.remove(0); //Remove slash

                if temp_path.contains("mp3") {
                    result.push(temp_path);
                } else {
                    continue
                }
            } else if path.is_dir() {
                result.extend(find_mp3(&path).unwrap());
            } else {
                continue
            }
        }
    }
    Ok(result)
}

fn write_mp3_list(name: &str, list: &Vec<String>) -> io::Result<()> {
    let target = list.join("\n");
    let temp = target.as_bytes();
    let mut f = File::create(Path::new(name))?;
    f.write_all(&temp)?;

    f.sync_all()?;
    Ok(())
}
#[macro_use] mod macros;

use std::{env, fs};
use std::path::Path;
use std::ffi::OsStr;
use std::io::prelude::*;

fn path_splitext(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn path_split(filename: &str) -> Option<&str> {
    Path::new(filename)
        .file_name()
        .and_then(OsStr::to_str)
}

fn reverse_file(
    buf: &Vec<u8>,
    n: usize,
    path_str: &String,
) {
    let mut reversed: Vec<u8> = vec![];
    reversed.extend(buf[..n].iter().rev());
    let mut file = fs::File::create(path_str).unwrap();
    file.write_all(&reversed).ok();
}

fn dir_handler(
    watch_list: &Vec<&str>,
    watch_header_jpeg: &Vec<u8>,
    watch_header_png: &[u8; 4],
    watch_header_png_2: &[u8; 4],
    watch_header_webp: &[u8; 4],
    watch_header_webp_2: &[u8; 4],
    enc_flag: bool,
    s_dir_str: String,
) {
    let paths = fs::read_dir(s_dir_str).unwrap();
    for path in paths {
        let path_str = path.unwrap().path().into_os_string().into_string().unwrap();
        let file_name = match path_split(&*path_str) {
            None => String::from(""),
            Some(res) => {
                String::from(res)
            },
        };
        if file_name.len() > 0 && file_name.chars().nth(0).unwrap() == '.' {
            // 跳过隐藏文件
            continue;
        }
        let md = fs::metadata(&path_str).unwrap();
        if md.is_dir() {
            dir_handler(
                watch_list,  
                watch_header_jpeg, 
                watch_header_png,
                watch_header_png_2,
                watch_header_webp,
                watch_header_webp_2,
                enc_flag, 
                path_str
            );
            continue;
        }
        let ext = match path_splitext(&*path_str) {
            None => String::from(""),
            Some(res) => {
                String::from(res)
            },
        };
        if watch_list.contains(&&*ext) {
            // 文件扩展名存在于监视列表中
            let mut f = fs::File::open(&path_str).expect("Something went wrong reading the file");
            // buffer 20M
            let mut buf: Vec<u8> = vec![0; 20971520];

            let n:usize = f.read(&mut buf[..]).unwrap();
            if (n > 16) && header_checker(
                &buf[..16].try_into().expect("Slice with incorrect length"), 
                watch_header_jpeg,
                watch_header_png,
                watch_header_png_2,
                watch_header_webp,
                watch_header_webp_2,
            ) {
                // 是正文件
                if enc_flag {
                    // 编码
                    reverse_file(&buf, n, &path_str)
                }
            } else {
                if !enc_flag {
                    // 解码
                    reverse_file(&buf, n, &path_str)
                }
            }
        }
    }
}

fn header_checker(
    header: &[u8; 16], 
    watch_header_jpeg: &Vec<u8>,
    watch_header_png: &[u8; 4],
    watch_header_png_2: &[u8; 4],
    watch_header_webp: &[u8; 4],
    watch_header_webp_2: &[u8; 4],
) -> bool{
    if [255, 216, 255].eq(&header[..3]) && header[3] >= 208 {
        // a-zA-Z_组成的字符集，或者第一位0，剩下在1236之间切换的设置选项
        if (
            (65 <= header[6] && header[6] <= 90) || (95 <= header[6] && header[6] <= 122)
            && (65 <= header[7] && header[7] <= 90) || (95 <= header[7] && header[7] <= 122)
            && (65 <= header[8] && header[8] <= 90) || (95 <= header[8] && header[8] <= 122)
            && (65 <= header[9] && header[9] <= 90) || (95 <= header[9] && header[9] <= 122)
        ) || (header[6] == 0 
            && watch_header_jpeg.contains(&header[7])
            && watch_header_jpeg.contains(&header[8])
            && watch_header_jpeg.contains(&header[9])
        ){
            // 合法的正向jpg文件
            return true
        } 
    } else if watch_header_png.eq(&header[..4]) {
        if watch_header_png_2.eq(&header[12..]) {
            // 合法的png文件
            return true
        }
    } else if watch_header_webp.eq(&header[..4]) {
        if watch_header_webp_2.eq(&header[8..12]) {
            // 合法的webp文件
            return true
        }
    }
    false
}

fn reverse_single_file(
    path_str: &String,
    watch_list: &Vec<&str>,
) {
    // buffer 20M
    let mut f = fs::File::open(&path_str).expect("Something went wrong reading the file");
    let mut buf: Vec<u8> = vec![0; 20971520];
    let n:usize = f.read(&mut buf[..]).unwrap();
    let ext = match path_splitext(&*path_str) {
        None => String::from(""),
        Some(res) => {
            String::from(res)
        },
    };
    if watch_list.contains(&&*ext) {
        reverse_file(&buf, n, &path_str)
    }   
}

fn main() {
    let watch_list: Vec<&str> = vec!["png","jpg","jpeg","webp"];
    let mut s_dir_str = String::from(".\\");

    let args: Vec<String> = env::args().collect();
    let enc_str: String = String::from("--enc");
    let dec_str: String = String::from("--dec");
    let help_str: String = String::from("-h");
    let mut enc_flag = false;
    if args.len() > 1 {
        if enc_str.eq(&args[1]) {
            enc_flag = true;
        } else if dec_str.eq(&args[1]) {
            enc_flag = false;
        } else if help_str.eq(&args[1]) {
            println!("Help: todo.");
            return ();
        } else if Path::new(&args[1]).is_file() {
            return reverse_single_file(&args[1], &watch_list);
        } else {
            let md = fs::metadata(&args[1]).unwrap();
            if md.is_dir() {
                // 想解码直接双击运行就行了，所以拖入文件夹大概率是加密
                s_dir_str = args[1].clone();
                enc_flag = true;
            }
        }
    }

    let watch_header_jpeg: Vec<u8> = vec![1,2,3,6];
    let watch_header_png: [u8; 4] = [137, 80, 78, 71];
    let watch_header_png_2: [u8; 4] = [73, 72, 68, 82];
    let watch_header_webp: [u8; 4] = [82, 73, 70, 70];
    let watch_header_webp_2: [u8; 4] = [87, 69, 66, 80];

    dir_handler(
        &watch_list, 
        &watch_header_jpeg, 
        &watch_header_png,
        &watch_header_png_2,
        &watch_header_webp,
        &watch_header_webp_2,
        enc_flag, 
        s_dir_str
    );
}


































































































































//
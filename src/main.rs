use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::env;

struct Picture {
    format: String,
    size: String,
    colors: String,
    value: String,
}

struct Point {
    red: u32,
    green: u32,
    blue: u32,
}

fn read_content(filename: &str) -> Picture {
    let mut picture = Picture { 
        format: String::new(), 
        size: String::new(), 
        value: String::new(), 
        colors : String::new() 
    };
    for (i,line) in read_to_string(filename).unwrap().lines().enumerate() {
        match i {
            0 => picture.format.push_str(line),
            1 => picture.size.push_str(line),
            2 => picture.colors.push_str(line),
            _ => picture.value.push_str(&format!("{}\n", line)),
        }
    }

    return picture
}

fn green_filter(point :&mut Point, content :&mut String) {
        if point.green <= 155 {
            point.green += 50;
        } else {
            point.green = 255;
        }
        content.push_str(&format!("{} {} {}\n", point.red, point.green, point.blue));
}

fn red_filter(point :&mut Point, content :&mut String) {
        if point.red <= 155 {
            point.red += 50;
        } else {
            point.red = 255;
        }
        content.push_str(&format!("{} {} {}\n", point.red, point.green, point.blue));
}

fn white_to_red(point :&mut Point, content :&mut String) {
    if point.red > 200 && point.green > 200 && point.blue > 200 {
        point.green = 0;
        point.blue = 0;
    }
    content.push_str(&format!("{} {} {}\n", point.red, point.green, point.blue));
}
   

fn violet_filter(point :&mut Point, content :&mut String) {
        if point.red <= 155 {
            point.red += 50;
        } else {
            point.red = 255;
        }

        if point.blue <= 155 {
            point.blue += 50;
        } else {
            point.blue = 255;
        }
        content.push_str(&format!("{} {} {}\n", point.red, point.green, point.blue));
}

fn thread_fn(picture :MutexGuard<Picture>, filter :fn(&mut Point, &mut String), targetfile :&str) {
    let mut content = format!("{}\n{}\n{}\n", &picture.format, &picture.size, &picture.colors);
    let numbers :Vec<u32>= picture.value.split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let mut i = 0;
    
    while i < numbers.len() {
        let mut point = Point { red:numbers[i], green:numbers[i+1], blue:numbers[i+2]};
        filter(&mut point, &mut content);
        i += 3;
    }

    let path = Path::new(targetfile);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create file {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() {
    let args :Vec<String> = env::args().collect();

    let now = Instant::now();
    let pic = Arc::new(Mutex::new(read_content(&args[1])));

    let mut handles = Vec::with_capacity(4);
    let functions = vec![(red_filter as fn(&mut Point, &mut String), "red.ppm"), 
        (green_filter as fn(&mut Point, &mut String), "green.ppm"), 
        (violet_filter as fn(&mut Point, &mut String), "violet.ppm"), 
        (white_to_red as fn(&mut Point, &mut String), "whiteToRed.ppm")];

    for function in functions {
        let pic = Arc::clone(&pic);
        let handle = thread::spawn(move || {
            let picture = pic.lock().unwrap();
            thread_fn(picture, function.0, function.1);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = now.elapsed();
    println!("Runtime = {:?}", elapsed_time);

}

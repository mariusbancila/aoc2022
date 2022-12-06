use crate::utils;
use std::path::Path;
use std::collections::VecDeque;

pub fn execute() {
    println!("=== puzzle 6 ===");

    assert_eq!(7, get_packet_position(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(5, get_packet_position(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(6, get_packet_position(String::from("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(10, get_packet_position(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(11, get_packet_position(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));

    let pos_packet = get_packet_position(get_text_from_file("./data/input06.txt"));
    println!("pos packet={}", pos_packet);

    assert_eq!(19, get_message_position(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(23, get_message_position(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(23, get_message_position(String::from("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(29, get_message_position(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(26, get_message_position(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));    

    let pos_message = get_message_position(get_text_from_file("./data/input06.txt"));
    println!("pos message={}", pos_message);

    println!();
}

fn get_text_from_file<P>(filename: P) -> String
where P : AsRef<Path> {
    let text = utils::read_file_string(filename);
    
    text.unwrap()
}

fn get_packet_position(text : String) -> i32 {
    get_position(text, 4)
}

fn get_message_position(text : String) -> i32 {
    get_position(text, 14)
}

fn get_position(text : String, length : usize) -> i32 {
    let mut packet : VecDeque<char> = VecDeque::new();

    let chars = text.chars();
    let mut pos = 0;

    for c in chars {
        if packet.iter().any(|x|*x==c) {
            loop {
                let removed = packet.pop_front().unwrap();
                if removed == c {
                    break;
                }
            }
        }

        packet.push_back(c);        
        pos += 1;

        if packet.len() == length {
            break;
        }
    }
    
    pos
}
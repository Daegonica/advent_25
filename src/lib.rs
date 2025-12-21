use file_reader::*;
use dlog::{*, enums::OutputTarget};
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Advent {
    log: Logger,
    puzzle: Vec<String>
}

impl Advent {

    pub fn new(day: &str) -> Advent {
        let log = Logger::init("advent", None, OutputTarget::Terminal).unwrap();
        let mut fr = FileReader::new("Terminal");

        let puzzle_day = format!("puzzles\\day_{}.txt", day);
        let puzzle = convert_puzzle_info(&fr.file_type(&puzzle_day).unwrap());

        Advent { log, puzzle }
    }

    pub fn puzzle_input(&mut self) {

        self.log.info(format!("{:?}", &self.puzzle.len()));

    }

    // Day 1 Part 1/2+2/2
    pub fn crack_safe(&mut self) {
        // [A-Za-z]+ looks for any letter lower/upper. (\d+) finds any digit of any amount. (\w+) will find L3 and not just L fyi
        let rot_dir_regex = Regex::new(r"([A-Za-z]+)(\d+)").unwrap();
        let mut word = "";
        let mut num_int = 0;
        let mut dial = 50;
        let mut zero_count = 0;

        // Take in puzzle info and figure out if you must turn left or right
        for rotation in &self.puzzle {

            if let Some(caps) = rot_dir_regex.captures(rotation) {
                if let Some(word_str) = caps.get(1) {
                    word = word_str.as_str();
                }
                if let Some(num_str) = caps.get(2) {
                    num_int = num_str.as_str().parse().unwrap();
                }
            }

            let mut current = dial as i32;
            match word {
                "R" => {
                    // Add/Subtract by 1 from 0-num_int to see when dial hits 0.
                    for _ in 0..num_int {
                        // .rem_euclid(100) keeps the int wrapped in 0-99
                        current = (current + 1).rem_euclid(100);
                        if current == 0 {
                            zero_count += 1;
                        }
                    }
                },
                "L" =>  {
                    for _ in 0..num_int {
                        current = (current - 1).rem_euclid(100);
                        if current == 0 {
                            zero_count += 1;
                        }
                    }
                },
                &_ => todo!(),
            }
            dial = current;
            self.log.info(format!("Dir: {} Amout: {} Dial: {} Hit Zero: {}", word, num_int, dial, zero_count));

        }
    }

    // Day 2 Part 1/2+2/2
    pub fn find_invalid(&mut self) {
        
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        for i in &self.puzzle {
            let mut total_invalid = 0;
            let mut full_invalid = 0;

            for caps in re.captures_iter(i) {
                let start: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                let finish: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
        
                for n in start..=finish {
                    if equal_halfs(n) {
                        total_invalid += n;
                    }

                    if has_repeated_sequence(n) {
                        full_invalid += n;
                    }
                }
            }
            self.log.info(format!("Total: {}. Full: {}", total_invalid, full_invalid));
        }
    }
    // Day 2 part 1/2+2/2 threaded
    pub fn thread_invalid(&mut self) {

        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        for info in &self.puzzle {
            for caps in re.captures_iter(info) {
                let start: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                let finish: i64 = caps.get(2).unwrap().as_str().parse().unwrap();

                ranges.push((start, finish));
            }
        }

        let total_invalid = Arc::new(Mutex::new(0i64));
        let full_invalid = Arc::new(Mutex::new(0i64));

        let mut handles = vec![];
        for (start, finish) in ranges {
            let total_invalid = Arc::clone(&total_invalid);
            let full_invalid = Arc::clone(&full_invalid);

            let handle = thread::spawn(move || {
                let mut local_total = 0;
                let mut local_full = 0;

                for n in start..=finish {
                    if equal_halfs(n) {
                        local_total += n;
                    }

                    if has_repeated_sequence(n) {
                        local_full += n;
                    }
                }

                *total_invalid.lock().unwrap() += local_total;
                *full_invalid.lock().unwrap() += local_full;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        self.log.info(
            format!(
                "Total: {}. Full: {}", 
                *total_invalid.lock().unwrap(), 
                *full_invalid.lock().unwrap()
            )
        );
    }

    // Day 3 part 1/2
    pub fn highest_volts(&mut self) {
        let mut all_banks: Vec<u32> = Vec::new();    
        for bank in &self.puzzle {
            let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let len = digits.len();

            let mut max1 = (usize::MAX, 0);
            let mut max2 = (usize::MAX, 0);

            // Get the two highest digits and their index
            for (i, &n) in digits.iter().enumerate() {
                if n > max1.1 {
                    max2 = max1;
                    max1 = (i, n);
                } else if n > max2.1 {
                    max2 = (i, n);
                }
            }

            let joined = if max1.0 == len-1 {
                format!("{}{}", max2.1, max1.1)
            } else {
                let second = digits[(max1.0 + 1)..].iter().max().copied().unwrap_or(0);
                format!("{}{}", max1.1, second)
            };
            all_banks.push(joined.parse().unwrap());
            
        }

        let total: u32 = all_banks.iter().sum();
        self.log.info(format!("Total Volts: {}", total));
    }

    // Day 3 Part 2/2
    pub fn highest_volts_advanced(&mut self) {
        let mut all_banks: Vec<String> = Vec::new();

        for bank in &self.puzzle {

            let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let print = highest(&digits, 12);
            self.log.info(format!("{print}"));
            all_banks.push(highest(&digits, 12));
            
        }

        let total: u64 = all_banks.iter().map(|n| n.parse::<u64>().unwrap()).sum();
        self.log.info(format!("{}", total));
    }

    // Day 4 Part 1/2
    pub fn carpet_rolls(&mut self) {

        let grid: Vec<Vec<char>> = self.puzzle.iter().map(|line| line.chars().collect()).collect();

        let adjacent = [(-1, -1), (-1, 0), (-1, 1),
                        (0, -1),           (0, 1),
                        (1, -1),  (1, 0),  (1, 1),];

        let mut touch_carpet = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '@' {
                    let mut count = 0;
                    for (dr, dc) in adjacent.iter() {
                        let nr = row as isize + dr;
                        let nc = col as isize + dc;
                        if nr >= 0 && nr < grid.len() as isize && nc >=0 && nc < grid[row].len() as isize {
                            if grid[nr as usize][nc as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        touch_carpet += 1;
                    }
                }
            }
        }

        self.log.info(format!("Take out {} carpets.", touch_carpet));
    }

    // Day 4 Part 2/2
    pub fn remove_carpets(&mut self) {

        let mut grid: Vec<Vec<char>> = self.puzzle.iter().map(|line| line.chars().collect()).collect();

        let adjacent = [(-1, -1), (-1, 0), (-1, 1),
                        (0, -1),           (0, 1),
                        (1, -1),  (1, 0),  (1, 1),];

        let mut removed_carpet = 0;
        let mut removed = false;
        let mut run = true;


        while run {
            removed = false;
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col] == '@' {
                        let mut count = 0;
                        for (dr, dc) in adjacent.iter() {
                            let nr = row as isize + dr;
                            let nc = col as isize + dc;
                            if nr >= 0 && nr < grid.len() as isize && nc >=0 && nc < grid[row].len() as isize {
                                if grid[nr as usize][nc as usize] == '@' {
                                    count += 1;
                                }
                            }
                        }
                        if count < 4 {
                            grid[row][col] = '.';
                            removed_carpet += 1;
                            removed = true;
                        }
                    }
                }
            }

            if removed == false {
                run = false;
            }
        }

        self.log.info(format!("Removed: {}", removed_carpet));

    }
}


pub fn highest(digits: &[u32], count: usize) -> String {
    let mut stack: Vec<u32> = Vec::with_capacity(count);
    let mut to_remove = digits.len().saturating_sub(count);

    for &digit in digits {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }
    stack.truncate(count);
    stack.iter().map(|d| d.to_string()).collect()
}

pub fn convert_puzzle_info(puzzle_info: &FileContent) -> Vec<String> {
    puzzle_info.as_lines().unwrap().to_vec()
}

pub fn has_repeated_sequence<T: ToString>(input: T) -> bool {

    let s = input.to_string();
    let len = s.len();
    for l in 1..=len/2 {
        if len % l != 0 {
            continue;
        }
        let part = &s[0..l];
        let mut matched = true;
        for i in (l..len).step_by(l) {
            if &s[i..i+l] != part {
                matched = false;
                break;
            }
        }
        if matched {
            return true;
        }
    }
    false
}

pub fn equal_halfs<T: ToString>(input: T) -> bool {
    let s = input.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    let first = &s[0..mid];
    let second = &s[mid..];

    first == second
}
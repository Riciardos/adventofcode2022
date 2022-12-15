use std::thread;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() {
    let join_1 = thread::spawn(|| day1::day1());
    let join_2 = thread::spawn(|| day2::day2());
    let join_3 = thread::spawn(|| day3::main());
    let join_4 = thread::spawn(|| day4::main());
    let join_5 = thread::spawn(|| day5::main());
    let join_6 = thread::spawn(|| day6::main());
    let join_7 = thread::spawn(|| day7::main());
    let join_8 = thread::spawn(|| day8::main());
    let join_9 = thread::spawn(|| day9::main());
    let join_10 = thread::spawn(|| day10::main());
    let join_11 = thread::spawn(|| day11::main());
    let join_12 = thread::spawn(|| day12::main());

    join_1.join().expect("thread 1 panicked");
    join_2.join().expect("thread 2 panicked");
    join_3.join().expect("thread 3 panicked");
    join_4.join().expect("thread 4 panicked");
    join_5.join().expect("thread 5 panicked");
    join_6.join().expect("thread 6 panicked");
    join_7.join().expect("thread 7 panicked");
    join_8.join().expect("thread 8 panicked");
    join_9.join().expect("thread 9 panicked");
    join_10.join().expect("thread 10 panicked");
    join_11.join().expect("thread 11 panicked");
    join_12.join().expect("thread 12 panicked");
}

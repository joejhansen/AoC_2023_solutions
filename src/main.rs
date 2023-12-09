use std::time::SystemTime;
// mod day_1;
// mod day_4;
// mod day_5;
// mod day_6;
mod day_7;
fn main() {
    // fn day_1() {
    //     let values = day_1::values();
    //     let start_time_1 = SystemTime::now();
    //     let d1p1_result = day_1::p1(values);
    //     let end_time_1 = SystemTime::now();
    //     println!("Timer 1 ended");
    //     let duration = end_time_1.duration_since(start_time_1).unwrap().as_millis();
    //     println!("Part 1 answer: {d1p1_result}\n{duration} milliseconds have elapsed");
    //     println!("Timer 2 started");
    //     let start_time_2 = SystemTime::now();
    //     let d1p2_result = day_1::p2(values);
    //     let end_time_2 = SystemTime::now();
    //     println!("Timer 2 ended");
    //     let duration = end_time_2.duration_since(start_time_2).unwrap().as_millis();
    //     println!("Part 2 answer: {d1p2_result}\n{duration} milliseconds have elapsed");
    // }
    // fn day_2() {}
    // fn day_3() {}
    // fn day_4() {
    //     let values = day_4::values();
    //     println!("Timer 1 started");
    //     let start_time_1 = SystemTime::now();
    //     let d4p1_result = day_4::p1(values);
    //     let end_time_1 = SystemTime::now();
    //     println!("Timer 1 ended");
    //     let duration = end_time_1.duration_since(start_time_1).unwrap().as_millis();
    //     println!("Part 1 answer: {d4p1_result}\n{duration} milliseconds have elapsed");
    //     println!("Timer 2 started");
    //     let start_time_2 = SystemTime::now();
    //     let d4p2_result = day_4::p2(values);
    //     let end_time_2 = SystemTime::now();
    //     println!("Timer 2 ended");
    //     let duration = end_time_2.duration_since(start_time_2).unwrap().as_millis();
    //     println!("Part 2 answer: {d4p2_result}\n{duration} milliseconds have elapsed");
    // }
    // fn day_5(){
    //     let test_values = day_5::values();
    //     let mut test_result = day_5::p2(&test_values);
    //     test_result.sort();
    //     let lowest_location = test_result[0];
    //     println!("Lowest = {lowest_location}");
    // }
    // fn day_6(){
    //     let values = day_6::values();
    //     // let result = day_6::p1(values);
    //     let result = day_6::p2(values);
    //     println!("{result}");
    // }
    fn day_7() {
        let values = day_7::values();
        let test_values = day_7::test_values();
        let start_time_1 = SystemTime::now();
        let p1_result = day_7::p1::p1(values);
        let end_time_1 = SystemTime::now();
        let duration = end_time_1.duration_since(start_time_1).unwrap().as_millis();
        println!("Part 1 Result: {p1_result}\nOperation finished in {duration} milliseconds");
    }
    day_7()
}

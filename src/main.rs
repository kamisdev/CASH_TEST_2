extern crate rand;
extern crate math;

use rand::prelude::*;
use std::io;
use math::round;
use std::process;
use std::convert::From;

// function for creating random number
fn create_random_value() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(1.00..10.00);
    y
}

// function for comparing input price and generated random price
fn compare(price_origin: f64, price_input: f64) -> bool {
    price_input >= price_origin
}

fn input_payment(price_origin: f64) -> (f64, usize) {

    let mut price_input: f64;
    let mut tried_cnt: usize = 0;

    loop {
        println!("Please input value for payment. Or please press 'q' to quit.");
        let mut str_input = String::new();

        // input string
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");

        // check if input 'q'
        let pp = str_input.as_bytes();
        if pp[0] == b'q' {
            println!("Do you really want to exit? y/n");

            // confirm quit
            str_input = String::from("");
            io::stdin()
                .read_line(&mut str_input)
                .expect("Cannot read line");
            
            let pp = str_input.as_bytes();
            if pp[0] == b'y' {
                process::exit(1);
            } else {
                continue;
            }
        }
        
        price_input = match str_input.trim().parse() {
            Ok(pay) => {
                if pay <= 0.0 || pay >= 1000.0 {
                    println!("Invalid input! Must be between 1.00 to 999.99");
                    continue;
                }

                // check format 000.00
                let check_pay = round::ceil(pay, 2);
                if check_pay * 100.0 != pay * 100.0 {
                    println!("Invalid input format! Must be format like : 000.00");
                    continue;
                }
                tried_cnt += 1;
                pay
            },
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        
        if compare(price_origin, price_input) {
            break;
        }
        
        println!("Price is not enough! Tried count : {tried_cnt}");

    }

    (price_input, tried_cnt)
}

fn calculate_coin_amount(price_change: f64) -> String {
    let coin_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];

    let mut price_change = price_change;

    let mut cur_index = 0;

    let mut result = String::new();

    while price_change > 0.0 {

        if price_change < coin_array[cur_index] {
            if cur_index == coin_array.len() - 1 {
                break;
            }
            cur_index += 1;
            continue;
        }

        let rest_price = (price_change*100.0) as usize;
        let coin_unit = (coin_array[cur_index]*100.0) as usize;

        let coin_cnt = rest_price / coin_unit;
        result.insert_str(result.len(), format!("{} of {} coin\n", coin_cnt, coin_array[cur_index]).as_str());
        let rest = rest_price % coin_unit;

        price_change = (rest as f64) / 100.0;
    }

    return result;
}

fn main() {
    let price_origin = create_random_value();
    let price_origin = round::floor(price_origin, 2);

    let (price_input, tried_cnt) = input_payment(price_origin);

    println!("{price_input}, {price_origin}, You succeed in {tried_cnt} times.");

    let price_change = price_input - price_origin;
    println!("Change is {:.2}", price_change);

    let coins_result = calculate_coin_amount(price_change);
    println!("{coins_result}");
}
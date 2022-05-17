// imports
use rand::Rng;

#[derive(Debug)]
enum WhaleGender {
    Male,
    Female,
}

// Functions
fn random_whales(amount: u8) -> Vec<WhaleGender> {
    // uninitialised Whales
    let mut whales: Vec<WhaleGender> = vec![];
    // let whales2 = [Option::<WhaleGender>::None; 2];

    for _ in 0..amount {
        let rand_num: f64 = rand::thread_rng().gen();

        match rand_num {
            x if x < 0.5 => whales.push(WhaleGender::Male),
            _ => whales.push(WhaleGender::Female),
        }
    }

    // for whale in &mut whales {
    //     // random float on [0;1[
    //     let rand_num: f64 = rand::thread_rng().gen();

    //     match rand_num {
    //         x if x < 0.5 => *whale = Some(WhaleGender::Male),
    //         _ => *whale = Some(WhaleGender::Female),
    //     }
    // }
    whales

    // Random numbers
    // let num: f64 = rand::thread_rng().gen::<f64>();
    // let num = rand::thread_rng().gen::<f64>();
    // let num: f64 = rand::thread_rng().gen();
}

// Main
#[allow(unused)]
pub fn run() {
    // 2 Whales(male or female)

    let mut rand_whales: Vec<WhaleGender>;
    let mut both_male_count: u64 = 0;
    let mut different_count: u64 = 0;
    for _ in 1..1000000 {
        rand_whales = random_whales(2);
        match &rand_whales[0..2] {
            [WhaleGender::Female, WhaleGender::Female] => (),
            [WhaleGender::Male, WhaleGender::Male] => both_male_count += 1,
            [WhaleGender::Male, WhaleGender::Female] => different_count += 1,
            [WhaleGender::Female, WhaleGender::Male] => different_count += 1,
            _ => todo!(),
        }
    }
    println!("both_male_count: {}", both_male_count);
    println!("different_count: {}", different_count);
    let sum = both_male_count + different_count;
    println!("sum: {}", sum);
    println!("both_male_rate: {}", both_male_count as f64 / sum as f64);
}

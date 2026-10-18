use rand::Rng;
use std::io;

fn main() {
    println!("добро пожаловать в игру Камень-Ножницы-Бумага!");

    loop {
        println!("Введи свой выбор (stone, scissors, paper):");

        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Ошибка чтения");
        
        let player_input = player_input.trim(); 

        
        if player_input == "exit" {
            println!("Спасибо за игру! До встречи!");
            break; 
        }

        
        let player_num = choice_to_number(player_input);
        if player_num == -1 {
            println!("Ошибка: неверное слово. Используй stone, scissors или paper.\n");
            continue; 
        }

        
        let computer_num = computer_choice();

        
        println!("Ты выбрал: {}", number_to_choice(player_num));
        println!("Компьютер выбрал: {}", number_to_choice(computer_num));
        
        
        println!("{}", result(player_num, computer_num));
        println!("-----------------------------");
    }
}


fn choice_to_number(choice: &str) -> i32 {
    match choice {
        "stone" => 0,
        "scissors" => 1,
        "paper" => 2,
        _=> -1,
    }
}

fn number_to_choice(num: i32) -> &'static str {
    match num {
        0 => "stone",
        1 => "scissors",
        2 => "paper",
        _=> "err",
    }
}

fn result(player: i32, computer: i32) -> &'static str {
    let result = (player - computer + 3) % 3;

    match result {
        0 => "Ничья! Вы оба выбрали одинаковое.",
        1 => "Компьютер победил!",
        2 => "Ты победил!",
        _ => "err",
    }
}

fn computer_choice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..3)
}

use rand::Rng;
use std::io;

fn main() {
    println!("добро пожаловать в игру Камень-Ножницы-Бумага!");

    loop {
        println!("Введи свой выбор (stone, scissors, paper, lizard, spock):");

        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Ошибка чтения");
        
        let player_input = player_input.trim(); 

        
        if player_input == "exit" {
            println!("Спасибо за игру! До встречи!");
            break; 
        }

        
        let player_num = choice_to_number(player_input);
        if player_num == -1 {
            println!("Ошибка: неверное слово. Используй stone, scissors, paper, lizard, spock.");
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
        "lizard" => 1,
        "spock" => 2,
        "scissors" => 3,
        "paper" => 4,
        _=> -1,
    }
}

fn number_to_choice(num: i32) -> &'static str {
    match num {
        0 => "stone",
        1 => "lizard",
        2 => "spock",
        3 => "scissors",
        4 => "paper",
        _=> "err",
    }
}

fn result(player: i32, computer: i32) -> &'static str {
    
    if player == computer {
        return "Ничья! Вы оба выбрали одинаковое.";
    }
 
    

    let player_wins = 
        (player == 0 && (computer == 3 || computer == 1)) || 
        (player == 1 && (computer == 4 || computer == 2)) || 
        (player == 2 && (computer == 0 || computer == 3)) || 
        (player == 3 && (computer == 4 || computer == 1)) || 
        (player == 4 && (computer == 0 || computer == 2));   

    if player_wins {
        "Ты победил!"
    } else {
        "Компьютер победил!"
    }
}

fn computer_choice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..5)
}



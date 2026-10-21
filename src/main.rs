/*
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
*/
use rand::Rng;
use std::io;

const ITEMS: &[&str] = &["stone", "lizard", "spock", "scissors", "paper"];

// Таблица побед: 
// По вертикали (строки) — выбор игрока.
// По горизонтали (столбцы) — выбор компьютера.
const MATRIX: [[i32; 5]; 5] = [
    [  0,  1, -1,  1, -1 ], // Игрок: Камень
    [ -1,  0,  1, -1,  1 ], // Игрок: Ящерица
    [  1, -1,  0,  1, -1 ], // Игрок: Спок
    [ -1,  1, -1,  0,  1 ], // Игрок: Ножницы
    [  1, -1,  1, -1,  0 ], // Игрок: Бумага
];

fn main() {
    println!("добро пожаловать в игру!");
    
    loop {
        println!("Введи свой выбор ({:?}):", ITEMS);

        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Ошибка чтения");
        let player_input = player_input.trim().to_lowercase();

        if player_input == "exit" {
            println!("Спасибо за игру! До встречи!");
            break; 
        }

        // Ищем номер предмета игрока
        let player_idx = match ITEMS.iter().position(|&x| x == player_input) {
            Some(index) => index,
            None => {
                println!("Ошибка: неверное слово.\n");
                continue; 
            }
        };

        // Выбор компьютера
        let mut rng = rand::thread_rng();
        let computer_idx = rng.gen_range(0..ITEMS.len());

        println!("Ты выбрал: {}", ITEMS[player_idx]);
        println!("Компьютер выбрал: {}", ITEMS[computer_idx]);
        
        // Получаем результат из матрицы (это займет 1 микросекунду)
        let result_code = MATRIX[player_idx][computer_idx];
        
        let result_text = match result_code {
            1 => "Ты победил!",
            0 => "Ничья! Вы оба выбрали одинаковое.",
            -1 => "Компьютер победил!",
            _ => "Ошибка",
        };

        println!("{}\n-----------------------------", result_text);
    }
}



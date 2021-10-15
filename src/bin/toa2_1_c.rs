/*
C. Проверьте правильность ситуации

Ограничение времени 	1 секунда
Ограничение памяти 	64Mb

Ввод 	стандартный ввод или input.txt
Вывод 	стандартный вывод или output.txt

Напишите программу, которая по изображению поля для игры в «Крестики-нолики» определит, 
могла ли такая ситуация возникнуть в результате игры с соблюдением всех правил.

Напомним, что игра в «Крестики-нолики» ведется на поле 3*3. 
Два игрока ходят по очереди. 
Первый ставит крестик, а второй – нолик. 
Ставить крестик и нолик разрешается в любую еще не занятую клетку поля. 
Когда один из игроков поставит три своих знака в одной горизонтали, 
вертикали или диагонали, или когда все клетки поля окажутся заняты, игра заканчивается.

Формат ввода
Вводится три строки по три числа в каждой, описывающих игровое поле. 
Число 0 обозначает пустую клетку, 1 – крестик, 2 – нолик. 
Числа в строке разделяются пробелами.

Формат вывода
Требуется вывести слово YES, 
если указанная ситуация могла возникнуть в ходе игры, 
и NO в противном случае.

Пример 1
Ввод
1 1 1
1 1 1
1 1 1
Вывод
NO

Пример 2
Ввод
2 1 1
1 1 2
2 2 1
Вывод
YES

Пример 3
Ввод
1 1 1
2 0 2
0 0 0
Вывод
YES

Пример 4
Ввод
0 0 0
0 1 0
0 0 0
Вывод
YES

*/

fn check_win(field: [[u8; 3]; 3], player: u8) -> bool {
    for i in 0..3 {
        let mut success = true;
        for j in 0..3 {
            if field[i][j] != player {
                success = false;
                break;
            }
        }
        if success {
            return true;
        }
    }

    for i in 0..3 {
        let mut success = true;
        for j in 0..3 {
            if field[j][i] != player {
                success = false;
                break;
            }
        }
        if success {
            return true;
        }
    }

    {
        let mut success = true;
        for j in 0..3 {
            if field[j][j] != player {
                success = false;
                break;
            }
        }
        if success {
            return true;
        }
    }

    {
        let mut success = true;
        for j in 0..3 {
            if field[j][2 - j] != player {
                success = false;
                break;
            }
        }
        if success {
            return true;
        }
    }

    false
}

fn is_correct(field: [[u8; 3]; 3]) -> bool {
    let mut count_x: u8 = 0;
    let mut count_o: u8 = 0;
    for i in 0..3 {
        for j in 0..3 {
            if field[i][j] == 1 {
                count_x += 1;
            } else if field[i][j] == 2 {
                count_o += 1;
            }
        }
    }

    if count_o > count_x {
        return false;
    }
    if count_o + 1 < count_x {
        return false;
    }

    if count_x == count_o {
        if check_win(field, 1) {
            return false;
        }
    } else {
        if check_win(field, 2) {
            return false;
        }
    }

    true
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    
}

fn main() {
    let mut field = [[0u8; 3]; 3];
    for i in 0..3 {
        let line: Vec<u8> = get_input().trim().split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
        field[i][0] = line[0];
        field[i][1] = line[1];
        field[i][2] = line[2];
    }
    if is_correct(field) {
        println!("YES");
    } else {
        println!("NO");
    }
}

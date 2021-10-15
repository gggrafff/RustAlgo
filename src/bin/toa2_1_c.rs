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
    
}
/*
https://acm.timus.ru/problem.aspx?space=1&num=1197
1197. Один в поле воин
Ограничение времени: 1.0 секунды
Ограничение памяти: 64 МБ

Условие этой задачи очень простое: вам всего лишь надо определить,
сколько клеток находится под боем шахматного коня, одиноко стоящего на шахматной доске.
На всякий случай напомним, что конь ходит буквой «Г» — на две клетки по горизонтали или вертикали
в любом направлении, и потом на одну клетку в направлении, перпендикулярном первоначальному.

Исходные данные
В первой строке находится единственное число N, 1 ≤ N ≤ 64 — количество тестов.
В каждой из последующих N строк содержится очередной тест:
два символа (маленькая латинская буква от 'a' до 'h' и цифра от 1 до 8) — стандартное шахматное
обозначение клетки, на которой стоит конь.
При этом буква обозначает вертикаль, а цифра — горизонталь.

Результат
Выведите N строк: в каждой из них должно находиться единственное число — количество клеток
шахматной доски, находящихся под боем коня.

Пример
исходные данные
3
a1
d4
g6
результат
2
8
6
*/

// https://zen.yandex.ru/media/english_upper_intermediate/kak-poangliiski-budet-ferz-kon-ladia-5a5624d2ad0f2218e3f2754c
/*
    int vert[8][8] = {
    {2,3,4,4,4,4,3,2,},
    {3,4,6,6,6,6,4,3,},
    {4,6,8,8,8,8,6,4,},
    {4,6,8,8,8,8,6,4,},
    {4,6,8,8,8,8,6,4,},
    {4,6,8,8,8,8,6,4,},
    {3,4,6,6,6,6,4,3,},
    {2,3,4,4,4,4,3,2,}};
*/
fn calc_cells(knight_cell: String) -> u8 {
    let x = knight_cell.chars().nth(0).unwrap() as u8 - 'a' as u8;
    let y = knight_cell.chars().nth(1).unwrap() as u8 - '1' as u8;

    let size_board = 8;
    let max_index = size_board - 1;

    if x % max_index == 0 && y % max_index == 0 {
        return 2;
    }

    let mut result = 8;
    if x < 2 || x > (max_index - 2) {
        result -= 2;
    }
    if y < 2 || y > (max_index - 2) {
        result -= 2;
    }
    if x == 0 || x == max_index {
        if y == 1 || y == (max_index - 1) {
            result -= 1;
        } else {
            result -= 2;
        }
    }
    if y == 0 || y == max_index {
        if x == 1 || x == (max_index - 1) {
            result -= 1;
        } else {
            result -= 2;
        }
    }

    return result;
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let n = get_input().trim().parse::<u8>().unwrap();
    for _i in 0..n {
        let knight_cell = get_input();
        let result = calc_cells(knight_cell);
        println!("{}", result);
    }
}

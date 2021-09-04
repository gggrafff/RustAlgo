/*
https://contest.yandex.ru/contest/28724/problems/A/

Решить в целых числах уравнение ( ax + b ) : ( cx + d ) = 0

Формат ввода
Вводятся 4 числа: a, b, c и d; c и d не равны нулю одновременно.

Формат вывода
Необходимо вывести все целочисленные решения, если их число конечно,
“NO” (без кавычек), если целочисленных решений нет, и “INF” (без кавычек), если их бесконечно много.
 */

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let a = get_input().trim().parse::<i64>().unwrap();
    let b = get_input().trim().parse::<i64>().unwrap();
    let c = get_input().trim().parse::<i64>().unwrap();
    let d = get_input().trim().parse::<i64>().unwrap();

    if a == 0 && b == 0 {
        println!("INF");
        return;
    }
    if a == 0 {
        println!("NO");
        return;
    }

    let candidate = -b / a;
    if a * candidate != -b {
        println!("NO");
        return;
    }
    if c * candidate == -d {
        println!("NO");
        return;
    }
    println!("{}", candidate);
    return;
}
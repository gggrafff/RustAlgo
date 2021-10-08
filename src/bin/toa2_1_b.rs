/*
B. Параллелограмм
Ограничение времени 1 секунда
Ограничение памяти 64Mb
Ввод стандартный ввод или input.txt
Вывод стандартный вывод или output.txt
На уроке геометрии семиклассники Вася и Петя узнали, что такое параллелограмм.
На перемене после урока они стали играть в игру:
Петя называл координаты четырех точек в произвольном порядке,
а Вася должен был ответить, являются ли эти точки вершинами параллелограмма.
Вася, если честно, не очень понял тему про параллелограммы,
и ему требуется программа, умеющая правильно отвечать на Петины вопросы.
Напомним, что параллелограммом называется четырехугольник,
противоположные стороны которого равны и параллельны.
Формат ввода
В первой строке входного файла записано
целое число N (1 ≤ N ≤ 10) - количество заданных Петей вопросов.
Каждая из N последующих строк содержит
описание четырех точек - четыре пары целых чисел X и Y (−100 ≤ X ≤ 100, −100 ≤ Y ≤ 100),
обозначающих координаты точки.
Гарантируется, что четыре точки, о которых идет речь в одном вопросе,
не лежат на одной прямой.
Формат вывода
Для каждого из вопросов выведите "YES",
если четыре заданные точки могут образовать параллелограмм,
и "NO" в противном случае.
Ответ на каждый из запросов должен быть в отдельной строке без кавычек.
Пример
Ввод
3
1 1 4 2 3 0 2 3
1 1 5 2 2 3 3 0
0 0 5 1 6 3 1 2
Вывод
YES
NO
YES
*/

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Interval<'a, 'b> {
    first: &'a Point,
    second: &'b Point,
}

fn distance(first: &Point, second: &Point) -> i64 {
    (first.x - second.x).pow(2) + (first.y - second.y).pow(2)
}

fn sort_points(points: &mut [Point; 4]) {
    if distance(&points[0], &points[2]) < distance(&points[0], &points[1]) {
        points.swap(1, 2);
    }
    if distance(&points[0], &points[2]) < distance(&points[0], &points[3]) {
        points.swap(2, 3);
    }
}

fn is_parallels(interval1: Interval, interval2: Interval) -> bool {
    let side1 = Point {x: interval1.second.x - interval1.first.x, 
                       y: interval1.second.y - interval1.first.y};
    let side2 = Point {x: interval2.second.x - interval2.first.x, 
                       y: interval2.second.y - interval2.first.y};
    if side1.y * side2.x != side1.x * side2.y {
        return false;
    }
    true
}

fn is_parallelogram(points: &mut [Point; 4]) -> bool {
    sort_points(points);

    if distance(&points[0], &points[1]) != distance(&points[2], &points[3]) {
        return false;
    }
    if !is_parallels(Interval {first: &points[0], second: &points[1]}, 
                     Interval {first: &points[2], second: &points[3]}){
        return false;
    }

    if distance(&points[0], &points[3]) != distance(&points[1], &points[2]) {
        return false;
    }
    if !is_parallels(Interval {first: &points[0], second: &points[3]}, 
                     Interval {first: &points[1], second: &points[2]}){
        return false;
    }

    true
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn numbers_to_figure(mut numbers: Vec<i64>) -> [Point; 4] {
    // assert_eq!(numbers.len(), 8);
    numbers.resize(8, 0);
    let mut figure = [Point { x: 0, y: 0 }; 4];
    for j in 0..4 {
        figure[j].x = numbers[j * 2];
        figure[j].y = numbers[j * 2 + 1];
    }
    figure
}

fn parse_input(input: String) -> [Point; 4] {
    let numbers: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    numbers_to_figure(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    #[test]
    fn test_parse_input_for_random_strings() {
        let mut rng = rand::thread_rng();
        for _i in 0..1000 {
            let mut numbers = Vec::<i64>::new();
            for _j in 0..8 {
                numbers.push(rng.gen_range(-100..100));
            }
            let random_input: String = numbers
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            assert!(parse_input(random_input)
                .iter()
                .zip(numbers_to_figure(numbers).iter())
                .all(|(a, b)| a == b))
        }
    }

    #[test]
    fn test_is_parallelogram_no_panic() {
        let mut rng = rand::thread_rng();
        for _i in 0..1000 {
            let mut numbers = Vec::<i64>::new();
            for _j in 0..8 {
                numbers.push(rng.gen_range(-100..100));
            }
            is_parallelogram(&mut numbers_to_figure(numbers));
        }
    }
}

fn main() {
    let n = get_input().trim().parse::<i64>().unwrap();
    for _i in 0..n {
        let mut points = parse_input(get_input());

        if is_parallelogram(&mut points) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

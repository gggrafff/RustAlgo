/*
https://acm.timus.ru/problem.aspx?space=1&num=1022

1022. Генеалогическое дерево
Ограничение времени: 1.0 секунды
Ограничение памяти: 64 МБ

Вступление
Система родственных отношений у марсиан достаточно запутана.
Собственно говоря, марсиане почкуются когда им угодно и как им угодно,
собираясь для этого разными группами, так что у марсианина может быть и один родитель,
и несколько десятков, а сотней детей сложно кого-нибудь удивить.
Марсиане привыкли к этому, и такой жизненный уклад кажется им естественным.

А вот в Планетарном Совете запутанная генеалогическая система создает серьёзные неудобства.
Там заседают достойнейшие из марсиан, и поэтому, чтобы никого не обидеть,
во всех обсуждениях слово принято предоставлять по очереди, так, чтобы сначала высказывались
представители старших поколений, потом те, что помладше, и лишь затем уже самые юные
и бездетные марсиане. Однако соблюдение такого порядка на деле представляет собой совсем не
простую задачу. Не всегда марсианин знает всех своих родителей,
что уж тут говорить про бабушек и дедушек! Но когда по ошибке сначала высказывается праправнук,
а потом только молодо выглядящий прапрадед — это настоящий скандал.

Задача
Ваша цель — написать программу, которая определила бы раз и навсегда такой порядок выступлений
в Планетарном Совете, который гарантировал бы, что каждый член совета получает возможность
высказаться раньше любого из своих потомков.

Исходные данные
В первой строке входных данных к этой задаче находится единственное число N,
1 ≤ N ≤ 100 — количество членов Марсианского Планетарного Совета.
По многовековой традиции все члены Совета нумеруются целыми числами от 1 до N.
Далее следуют ровно N строк, причем i-тая строка содержит список детей
члена Совета с порядковым номером i.
Список детей представляет собой последовательность порядковых номеров детей,
разделенных пробелами и следующих в произвольном порядке.
Список детей может быть пустым. Список детей (даже если он пуст) оканчивается нулем.

Результат
Выход должен содержать последовательность номеров выступающих, разделенных пробелами.
Если несколько последовательностей удовлетворяют условиям задачи, то можно вывести любую из них.
Гарантируется, что хотя бы одна такая последовательность существует.

Пример
исходные данные
5
0
4 5 1 0
1 0
5 3 0
3 0

результат
2 4 5 3 1
*/

fn main() {


}
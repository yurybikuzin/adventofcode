
# [https://adventofcode.com/2023/day/1](https://adventofcode.com/2023/day/1)

## Task Description

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```
In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together produces `142`.

Consider your entire [calibration document](https://u2h.ru/aoc2023_day1_input.txt). **What is the sum of all of the calibration values?**

## Solution

### Simple (naive) approach ([make it work](https://wiki.c2.com/?MakeItWorkMakeItRightMakeItFast))

Let's write solution in ["one line"](https://github.com/yurybikuzin/adventofcode/commit/bba3c4a99c98b4d961109f6c7a886e6689503395#diff-293ddc6db1ff30583647b4ecba71f30e3a0fb5b5b06ebe68e8a9d0bb84039040)

And supply it with a [test](https://github.com/yurybikuzin/adventofcode/blob/bba3c4a99c98b4d961109f6c7a886e6689503395/src/rust/aoc2023_day1/src/main.rs#L10-L25)

[The main function](https://github.com/yurybikuzin/adventofcode/blob/bba3c4a99c98b4d961109f6c7a886e6689503395/src/rust/aoc2023_day1/src/main.rs#L3-L8) will be simple since [PUZZLE_INPUT](https://github.com/yurybikuzin/adventofcode/blob/bba3c4a99c98b4d961109f6c7a886e6689503395/src/rust/aoc2023_day1/src/main.rs#L27-L1026) is a `const`

This works and solves the problem

But is it suitable for real life? 

In real life, the data may be larger than in our simple example, and may arrive in chunks (for example, via Internet)

So for real life solution, we might need something less resource consuming 

### Real life approach ([make it right](https://wiki.c2.com/?MakeItWorkMakeItRightMakeItFast))

New [solution](https://github.com/yurybikuzin/adventofcode/blob/v0.2.1/src/rust/aoc2023_day1/src/summator.rs)
with [test](https://github.com/yurybikuzin/adventofcode/blob/v0.2.1/src/rust/aoc2023_day1/src/lib.rs#L21-L30)

The [use in main function](https://github.com/yurybikuzin/adventofcode/blob/v0.2.1/src/rust/aoc2023_day1/src/main.rs#L11-L23) is a little more complicated because we download [the calibration document](https://u2h.ru/aoc2023_day1_input.txt) from Internet and calculate the sum of the calibration values on the fly, without having to access the entire document at the same time


# AoC-Rust-Utils

The main goal is to create automatically the AdventOfCode rust file for a given year and date.
Please don't use this with LLMs to reach the <a>https://adventofcode.com/ leaderboard.

## Installing AoC-Rust-Utils

```sh
git clone https://github.com/username2000w/AoC-Rust-Utils/ && cd AoC-Rust-Utils
```

## Getting Started

My tool follow the [**automation guidelines**](https://old.reddit.com/r/adventofcode/wiki/faqs/automation) on the [/r/adventofcode](https://www.reddit.com/r/adventofcode/) community

You have to write your adventofcode session key in the `session_key.txt` file in the root directory

Then you can run the program with (or without) parameters.

### Parameters

- `--year` (or `-y`) : the year you want the AoC input, the default value is the current UTC date's day.
- `--day` (or `-d`) : the day you want the AoC input, the default value is the current UTC date's year.
- `--path` (or `-p`) : the path you want to create the day's directory. /!\ THIS IS NOT IMPLEMENTED YET. /!\
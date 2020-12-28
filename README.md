# relaty

This small cli app helps you rank and rate stuff based on how items compare to each other.

## Usage

### Help

`relaty --help` shows you usage information.
You may use `relaty <CMD> --help` to see information on the usage of different commands.

### new

`relaty new <OUTPUT> [ITEM]...` creates an empty list file

### from

`relaty from <INPUT> <OUTPUT>` creates a list file from an existing text file. Each line will get an own entry.

### add

`relaty add <FILE> [ITEM]...` add items to a list file

#### Options

- `-o <OUTPUT>`: Use a separate output file

### remove

`relaty remove <FILE> <FILTER>` remove entries from a file. `<FILTER>` is a regular expression ([Syntax](https://docs.rs/regex/1.4.2/regex/#syntax))

### reset

`relaty reset <FILE> <FILTER>` reset entries in a file. `<FILTER>` is a regular expression ([Syntax](https://docs.rs/regex/1.4.2/regex/#syntax))

#### Options

- `-o <OUTPUT>`: Use a separate output file.

### print

`relaty print <INPUT> [OUTPUT]` prints a list file to screen or to a file showing wins, votes and win percentage.

#### Options

- `-f <FILTER>`: Filter by regular expression ([Syntax](https://docs.rs/regex/1.4.2/regex/#syntax))

### stats

`relaty stats <INPUT>` shows some stats about a list file.

### vote

`relate vote <FILE> [ROUNDS]` vote ROUNDS (default is 10) times between two items.

#### Options

- `-i`: Show additional information (wins, votes, percentage)
- `-o <OUTPUT>`: Use a separate output file
- `-s <STRATEGY>`: Use a different strategy. Default is `random`.

#### Strategies

- `random`: Choose both entries at random.
- `onemin`: Choose one entry with the minimal amount of votes. The other is random. Will fail if there are no matching pairs.
- `equal`: Choose two entries with equal percentage.
- `minequal`: Choose between `onemin` and `equal` randomly (Chance is 50/50).
- `nearest`: Similar to `equal`. Chooses the entries nearest to each other.
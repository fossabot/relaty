# relaty

This small cli app helps you rank and rate stuff based on how items compare to each other.

## Usage

### Help

`relaty --help` shows you usage information.
You may use `relaty <CMD> --help` to see information on the usage of different commands.

### new

`relaty new -o <FILE>` creates an empty storage file

### from

`relaty from -i <FILE> -o <FILE>` creates a storage file from an existing text file. Each line will get an own entry.

### print

`relaty print -i <FILE> [-o <FILE>]` prints a storage file to screen or to a file showing wins, votes and win percentage.
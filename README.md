# About

Simple utility to check and update Tresorit’s [roaming filters](https://support.tresorit.com/hc/en-us/articles/217103697-Exclude-specific-file-types-from-sync-advanced-).

## Running

```
• trfilter -h
trfilter 0.1.0
Balakrishnan Chandrasekaran <balakrishnan.c@gmail.com>
Utility for editing Tresorit's roaming filter

USAGE:
    trfilter [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <filter>    Absolute/relative path of a roaming filter [default: .tresorit/Filters/roaming.filter]

SUBCOMMANDS:
    check      Check rules in the roaming filter file
    help       Prints this message or the help of the given subcommand(s)
    show       Show rules specified in the roaming filter
    suggest    Suggest rules for adding to the roaming filter
    upgrade    Initialize or upgrade roaming filter with suggestions
```

## Caveats

_It is my first attempt to write more than a simple one-file program in rust. Constructive feedback is always welcome!_

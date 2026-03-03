# CrashLink

## Usage
Download the executable for your system from the [latest release](https://github.com/Thegamefire/CrashLink/releases/latest)
Run the executable with the necessary arguments detailed below.
```
CrashLink

USAGE:
    CrashLink list-processes
    CrashLink run [FLAGS] [OPTIONS] <target-process> <ap-host> <ap-slot>

FLAGS:
    -h, --help                Prints help information
    -V, --version             Prints version information
    -n, --use-process-name    Matches processes based on name

OPTIONS:
    -p, --ap-pass <ap-pass>   Password for the archipelago server [default: ]

ARGS:
    <target-process>
    <ap-host>
    <ap-slot>

SUBCOMMANDS:
    help              Prints this message or the help of the given subcommand(s)
    list-processes    Lists running processes and exits
    run               Connect to Archipelago and monitor a process
```

### How do I now what the target process is
run 
```
CrashLink list-processes
``` 
and look through the list. 
Usually you can find your game in the `Exe` field, in this case you shouldn't use the `use-process-name` option.

When this isn't possible you'll need to use the process name.
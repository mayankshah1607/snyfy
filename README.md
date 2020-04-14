# snyfy
A simple CLI tool built with Rust to perform port enumeration on remote systems.

## Installation

### Linux and Mac
```bash
$ curl snyfy.mayankshah.dev | sh
```

### Windows
Coming soon...

### Install from source

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system
2. Clone this repo
   ```bash
   $ git clone 
   ```
3. Change to the root directory of the project
   ```bash
   $ cd snyfy
   ```
4. Install using `make`
   ```bash
   $ make install
   ```

## Usage

To use this tool, simply run
```bash
$ snyfy [OPTIONS] <IP>
```

### Args
| Arg    | Description     |
| ------ | -------------   |
| `<IP>` | Sets the IPv4 Address of the target machine |

### Flags
| Flag | Description |
|------|-------------|
| `-h`, `--help` | Prints help and usage information |
| `-V`, `--version` | Prints version information |

### Options
| Option | Description |
| -------| ----------- |
| `-u`, `--max` | Sets a lower bound on the range of ports to scan. Default is 65535 |
| `-l`, `--max` | Sets a lower bound on the range of ports to scan. Default is 0 |
| `-n`, `--num` | Sets the number of threads to use. Default is 1000 |

### Examples
```bash
# Scan ports from 0 to 65535 using 1000 threads
$ snyfy 192.168.1.1 

# Scan ports from range 0 to 65535 using 1500 threads
$ snyfy -n 1500 192.168.0.1

# Scan ports from range 10000-20000 using 10000 threads
$ snyfy -n 1500 -l 10000 -u 20000 192.168.1.1
```
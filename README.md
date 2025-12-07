# Filter

A powerful command-line tool for filtering and processing command output with a
clean, composable syntax.

## Overview

Filter allows you to pipe command output through a series of transformations
using a simple `.` separator. All commands are executed through your default 
shell (determined by the `$SHELL` environment variable - bash, zsh, fish, etc.) 
with the `-i` flag, so all your aliases and shell functions work seamlessly.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/filter`.

## Usage

```bash
filter <command> -- <operation1> . <operation2> . <operation3>
```

- Everything before `--` is the command to execute
- Everything after `--` is a series of operations separated by `.`
- Operations are applied left-to-right

## Commands

### `filter <pattern>`

Keep only lines that contain the pattern. Supports multiple conditions:
- `filter "pattern1 || pattern2"` - Keep lines matching ANY pattern (OR)
- `filter "pattern1 & pattern2"` - Keep lines matching ALL patterns (AND)

**Examples:**

```bash
# Show only error lines from logs
filter cat app.log -- filter error

# Find files containing "TODO"
filter grep -r "TODO" . -- filter src

# Show running processes for user "john"
filter ps aux -- filter john

# Show lines with either "error" or "warning"
filter cat app.log -- filter "error || warning"

# Show lines with both "error" and "network"
filter cat app.log -- filter "error & network"

# Complex OR condition - lines with "404" or "500" or "503"
filter cat access.log -- filter "404 || 500 || 503"
```

### `exclude <pattern>`

Remove lines that contain the pattern. Supports multiple conditions:
- `exclude "pattern1 || pattern2"` - Exclude lines matching ANY pattern (OR)
- `exclude "pattern1 & pattern2"` - Exclude lines matching ALL patterns (AND)

**Examples:**

```bash
# Remove comment lines
filter cat config.txt -- exclude "#"

# Show all processes except Chrome
filter ps aux -- exclude chrome

# Filter out info messages from logs
filter cat app.log -- exclude info

# Exclude lines with either "debug" or "trace"
filter cat app.log -- exclude "debug || trace"

# Exclude lines that have both "test" and "skip"
filter cat output.txt -- exclude "test & skip"
```

### `slice <range>`

Select specific lines by index (1-indexed, inclusive). Negative indices count from the end.

**Syntax:**
- `slice N` - Get line N only
- `slice N:M` - Get lines N through M (inclusive)
- `slice N:` - Get lines from N to the end
- `slice -N` - Get the Nth line from the end (-1 is last line)
- `slice -N:-M` - Get lines from Nth to Mth from the end (inclusive)
- `slice -N:` - Get last N lines to the end

**Examples:**

```bash
# Get the first line (header)
filter ls -la -- slice 1

# Get lines 5 through 10
filter cat file.txt -- slice 5:10

# Skip the first 3 lines
filter cat file.txt -- slice 4:

# Get just the 10th line
filter cat file.txt -- slice 10

# Get the last line
filter cat file.txt -- slice -1

# Get the last 5 lines
filter cat file.txt -- slice -5:-1

# Get everything except the first line
filter ls -la -- slice 2:
```

### `choose <columns>`

Select specific columns from whitespace-separated output (1-indexed, inclusive). Negative indices count from the end.

**Syntax:**
- `choose N` - Get column N only
- `choose N M P` - Get columns N, M, and P
- `choose N:M` - Get columns N through M (inclusive)
- `choose N:` - Get columns from N to the end
- `choose -N` - Get the Nth column from the end (-1 is last column)
- `choose -N:-M` - Get columns from Nth to Mth from the end (inclusive)

**Examples:**

```bash
# Get just usernames from ps
filter ps aux -- choose 1

# Get PID and command name
filter ps aux -- choose 2 11

# Get columns 3 through 5
filter ls -la -- choose 3:5

# Get everything from column 4 onwards
filter ls -la -- choose 4:

# Show only the permissions and filename
filter ls -la -- choose 1 9

# Get the last column (command name from ps)
filter ps aux -- choose -1

# Get the last 3 columns
filter ls -la -- choose -3:-1

# Get first and last column
filter ps aux -- choose 1 -1
```

### `unique`

Remove duplicate lines while preserving the order of first occurrence.

**Examples:**

```bash
# Get unique lines from a file
filter cat names.txt -- unique

# Get unique error messages
filter cat app.log -- filter error . unique

# Get unique process owners
filter ps aux -- choose 1 . unique

# Find unique file extensions
filter find . -type f -- choose 1 . unique
```

## Chaining Commands

The real power comes from chaining operations together:

```bash
# Get unique error messages, then show the first 10
filter cat app.log -- filter error . unique . slice 1:10

# Get the 2nd column, remove duplicates, and filter for "python"
filter ps aux -- choose 2 . unique . filter python

# Get files, exclude hidden files, show first 20
filter ls -la -- exclude "." . slice 1:20

# Show running processes, get PID and name, filter for node, show first 5
filter ps aux -- choose 2 11 . filter node . slice 1:5

# Get unique users running processes, exclude root
filter ps aux -- choose 1 . unique . exclude root

# Complex pipeline: errors from last 100 lines, unique, show first 5
filter tail -n 100 app.log -- filter error . unique . slice 1:5

# Show lines with errors or warnings, but not debug messages
filter cat app.log -- filter "error || warning" . exclude debug

# Find critical errors (lines with both "error" and "critical")
filter cat app.log -- filter "error & critical" . slice 1:20
```

## Real-World Examples

### Log Analysis

```bash
# Find unique error types in the last hour of logs
filter tail -n 1000 /var/log/app.log -- filter ERROR . choose 3: . unique

# Count different types of HTTP status codes
filter cat access.log -- choose 9 . unique

# Get all 500 errors, show timestamp and message
filter cat access.log -- filter "500" . choose 4 7:

# Show errors or warnings from the last 100 lines
filter tail -n 100 app.log -- filter "error || warning" . unique

# Find database errors (lines with both "error" and "database")
filter cat app.log -- filter "error & database" . slice 1:50
```

### Process Management

```bash
# Find all Python processes with their memory usage
filter ps aux -- filter python . choose 1 4 11

# Get top 5 memory-consuming processes
filter ps aux --sort=-%mem -- slice 2:6 . choose 11 4

# List all unique users running processes
filter ps aux -- choose 1 . unique
```

### File System

```bash
# Find all Rust files in src directory
filter find src -type f -- filter .rs

# Show large files (over 1GB) sorted by size
filter du -h /var -- filter G . slice 1:10

# Get unique file extensions from current directory
filter find . -type f -- choose 1 . unique
```

### Git Operations

```bash
# Get unique authors from git log
filter git log --oneline -- choose 2: . unique

# Show commits from last week by author
filter git log --since="1 week ago" --oneline -- choose 2: . unique

# Find all files changed in last 10 commits
filter git log --name-only --oneline -10 -- filter "/" . unique
```

## Features

- **Shell Alias Support**: Runs commands through your default shell (`$SHELL`) with `-i` flag, so all your aliases and functions work (supports bash, zsh, fish, etc.)
- **1-Indexed**: Line and column numbers start at 1 (not 0) for intuitive usage
- **Inclusive Ranges**: `1:5` includes both 1 and 5
- **Composable**: Chain any commands together for complex transformations
- **Whitespace Handling**: `choose` intelligently handles irregular spacing

## Testing

Run the test suite:

```bash
cargo test
```

## Why Filter?

Traditional Unix tools like `grep`, `awk`, `sed`, and `cut` are powerful but can be hard to remember and chain together. Filter provides a simpler, more intuitive syntax:

**Traditional:**
```bash
ps aux | grep python | awk '{print $2, $11}' | head -5
```

**With Filter:**
```bash
filter ps aux -- filter python . choose 2 11 . slice 1:5
```

## License

MIT

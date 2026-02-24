fn filter_lines(input: String, args: &[&str]) -> String {
    // Keep lines that contain the pattern(s)
    // Supports: filter pattern1 & pattern2 (AND), filter pattern1 || pattern2 (OR)
    if args.is_empty() {
        return input;
    }
    
    let args_str = args.join(" ");
    
    // Check if we have OR conditions
    if args_str.contains("||") {
        let patterns: Vec<&str> = args_str.split("||").map(|s| s.trim()).collect();
        let result = input
            .lines()
            .filter(|line| patterns.iter().any(|pattern| line.contains(pattern)))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    } else if args_str.contains("&") {
        // Check if we have AND conditions
        let patterns: Vec<&str> = args_str.split("&").map(|s| s.trim()).collect();
        let result = input
            .lines()
            .filter(|line| patterns.iter().all(|pattern| line.contains(pattern)))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    } else {
        // Single pattern
        let pattern = args_str.trim();
        let result = input
            .lines()
            .filter(|line| line.contains(pattern))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    }
}

fn exclude_lines(input: String, args: &[&str]) -> String {
    // Remove lines that contain the pattern(s)
    // Supports: exclude pattern1 & pattern2 (AND - exclude if has both), exclude pattern1 || pattern2 (OR - exclude if has either)
    if args.is_empty() {
        return input;
    }
    
    let args_str = args.join(" ");
    
    // Check if we have OR conditions (exclude if line matches ANY pattern)
    if args_str.contains("||") {
        let patterns: Vec<&str> = args_str.split("||").map(|s| s.trim()).collect();
        let result = input
            .lines()
            .filter(|line| !patterns.iter().any(|pattern| line.contains(pattern)))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    } else if args_str.contains("&") {
        // Check if we have AND conditions (exclude if line matches ALL patterns)
        let patterns: Vec<&str> = args_str.split("&").map(|s| s.trim()).collect();
        let result = input
            .lines()
            .filter(|line| !patterns.iter().all(|pattern| line.contains(pattern)))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    } else {
        // Single pattern
        let pattern = args_str.trim();
        let result = input
            .lines()
            .filter(|line| !line.contains(pattern))
            .collect::<Vec<_>>()
            .join("\n");
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    }
}

fn slice_lines(input: String, args: &[&str]) -> String {
    // Take a range of lines (1-indexed, inclusive: "slice 1:5" takes lines 1-5, "slice 2", or "slice 5:")
    // Negative indices count from the end: "slice -1" is last line, "slice -5:-1" is last 5 lines
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    
    if let Some(first_arg) = args.get(0) {
        let result = if first_arg.contains(':') {
            // Parse range notation "start:end" or "start:"
            let parts: Vec<&str> = first_arg.split(':').collect();
            
            // Parse start index (can be negative)
            let start = if let Some(s) = parts.get(0).filter(|s| !s.is_empty()) {
                if let Ok(n) = s.parse::<isize>() {
                    if n < 0 {
                        // Negative index: count from end
                        len.saturating_sub(n.unsigned_abs())
                    } else {
                        // Positive index: convert from 1-indexed to 0-indexed
                        (n as usize).saturating_sub(1)
                    }
                } else {
                    0
                }
            } else {
                0
            };
            
            // Parse end index (can be negative)
            let end = if let Some(s) = parts.get(1).filter(|s| !s.is_empty()) {
                if let Ok(n) = s.parse::<isize>() {
                    if n < 0 {
                        // Negative index: count from end
                        len.saturating_sub(n.unsigned_abs())
                    } else {
                        // Positive index: keep as-is for inclusive range
                        n as usize
                    }
                } else {
                    len
                }
            } else {
                len
            };
            
            lines
                .into_iter()
                .skip(start)
                .take(end.saturating_sub(start))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            // Single number - take just that line (1-indexed or negative)
            if let Ok(n) = first_arg.parse::<isize>() {
                let start = if n < 0 {
                    // Negative index: count from end
                    len.saturating_sub(n.unsigned_abs())
                } else {
                    // Positive index: convert from 1-indexed to 0-indexed
                    (n as usize).saturating_sub(1)
                };
                
                lines
                    .into_iter()
                    .skip(start)
                    .take(1)
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                String::new()
            }
        };
        
        if !result.is_empty() {
            format!("{}\n", result)
        } else {
            result
        }
    } else {
        input
    }
}

fn choose_columns(input: String, args: &[&str]) -> String {
    // Select specific columns (1-indexed, inclusive: "choose 1 3" to get columns 1 and 3, "choose 2:5" for columns 2-5, or "choose 2:" for column 2 onwards)
    // Negative indices count from the end: "choose -1" is last column, "choose -3:-1" is last 3 columns
    
    let result = input
        .lines()
        .map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let col_len = columns.len();
            
            // Check if we have a range
            let has_range = args.iter().any(|arg| arg.contains(':'));
            
            if has_range {
                // Parse range notation
                for arg in args {
                    if arg.contains(':') {
                        let parts: Vec<&str> = arg.split(':').collect();
                        
                        // Parse start index (can be negative)
                        let start = if let Some(s) = parts.get(0).filter(|s| !s.is_empty()) {
                            if let Ok(n) = s.parse::<isize>() {
                                if n < 0 {
                                    col_len.saturating_sub(n.unsigned_abs())
                                } else {
                                    (n as usize).saturating_sub(1)
                                }
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        
                        // Parse end index (can be negative)
                        let end = if let Some(s) = parts.get(1).filter(|s| !s.is_empty()) {
                            if let Ok(n) = s.parse::<isize>() {
                                if n < 0 {
                                    col_len.saturating_sub(n.unsigned_abs())
                                } else {
                                    n as usize
                                }
                            } else {
                                col_len
                            }
                        } else {
                            col_len
                        };
                        
                        return columns
                            .into_iter()
                            .skip(start)
                            .take(end.saturating_sub(start))
                            .collect::<Vec<_>>()
                            .join(" ");
                    }
                }
                String::new()
            } else {
                // Parse individual column indices (can be negative)
                let column_indices: Vec<usize> = args
                    .iter()
                    .filter_map(|arg| {
                        if let Ok(n) = arg.parse::<isize>() {
                            if n < 0 {
                                Some(col_len.saturating_sub(n.unsigned_abs()))
                            } else {
                                Some((n as usize).saturating_sub(1))
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                
                column_indices
                    .iter()
                    .filter_map(|&i| columns.get(i).copied())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    if !result.is_empty() {
        format!("{}\n", result)
    } else {
        result
    }
}

fn unique_lines(input: String, _args: &[&str]) -> String {
    // Output only unique lines (preserves order of first occurrence)
    let mut seen = std::collections::HashSet::new();
    let result = input
        .lines()
        .filter(|line| seen.insert(*line))
        .collect::<Vec<_>>()
        .join("\n");
    
    if !result.is_empty() {
        format!("{}\n", result)
    } else {
        result
    }
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<String>>();

    let split = args.iter().position(|x| x == "--");

    let (command, rest) = if let Some(index) = split {
        let rest = args.split_off(index);
        let command = args.drain(..);

        (
            command.collect::<Vec<_>>(),
            rest.into_iter().skip(1).collect::<Vec<_>>(),
        )
    } else {
        return;
    };

    let command_str = command.join(" ");
    
    // Get the user's default shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    
    let output = std::process::Command::new(&shell)
        .arg("-i")
        .arg("-c")
        .arg(&command_str)
        .output()
        .expect("failed to execute process");

    // Convert output to string
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse the rest commands
    let rest_str = rest.join(" ");
    let commands: Vec<&str> = rest_str.split(".").map(|s| s.trim()).collect();

    for cmd in commands {
        if cmd.is_empty() {
            continue;
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command_name = parts[0];
        let args = &parts[1..];

        match command_name {
            "filter" => {
                result = filter_lines(result, args);
            }
            "exclude" => {
                result = exclude_lines(result, args);
            }
            "slice" => {
                result = slice_lines(result, args);
            }
            "choose" => {
                result = choose_columns(result, args);
            }
            "unique" => {
                result = unique_lines(result, args);
            }
            _ => {
                eprintln!("Unknown command: {}", command_name);
            }
        }
    }

    // Print the final result
    print!("{}", result);
    eprint!("{}", stderr);

    std::process::exit(output.status.code().unwrap_or(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_lines_basic() {
        let input = "line one\nline two\nline three\n".to_string();
        let result = filter_lines(input, &["two"]);
        assert_eq!(result, "line two\n");
    }

    #[test]
    fn test_filter_lines_multiple_matches() {
        let input = "error: failed\ninfo: success\nerror: timeout\n".to_string();
        let result = filter_lines(input, &["error"]);
        assert_eq!(result, "error: failed\nerror: timeout\n");
    }

    #[test]
    fn test_filter_lines_no_matches() {
        let input = "line one\nline two\n".to_string();
        let result = filter_lines(input, &["three"]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_filter_lines_or_condition() {
        let input = "error: failed\ninfo: success\nwarning: deprecated\nerror: timeout\n".to_string();
        let result = filter_lines(input, &["error", "||", "warning"]);
        assert_eq!(result, "error: failed\nwarning: deprecated\nerror: timeout\n");
    }

    #[test]
    fn test_filter_lines_and_condition() {
        let input = "error: network failed\ninfo: success\nerror: timeout\nwarning: network slow\n".to_string();
        let result = filter_lines(input, &["error", "&", "network"]);
        assert_eq!(result, "error: network failed\n");
    }

    #[test]
    fn test_filter_lines_or_no_matches() {
        let input = "line one\nline two\n".to_string();
        let result = filter_lines(input, &["three", "||", "four"]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_filter_lines_and_partial_match() {
        let input = "has foo\nhas bar\nhas foo and bar\n".to_string();
        let result = filter_lines(input, &["foo", "&", "bar"]);
        assert_eq!(result, "has foo and bar\n");
    }

    #[test]
    fn test_exclude_lines_basic() {
        let input = "line one\nline two\nline three\n".to_string();
        let result = exclude_lines(input, &["two"]);
        assert_eq!(result, "line one\nline three\n");
    }

    #[test]
    fn test_exclude_lines_multiple_matches() {
        let input = "error: failed\ninfo: success\nerror: timeout\n".to_string();
        let result = exclude_lines(input, &["error"]);
        assert_eq!(result, "info: success\n");
    }

    #[test]
    fn test_exclude_lines_or_condition() {
        let input = "error: failed\ninfo: success\nwarning: deprecated\nerror: timeout\n".to_string();
        let result = exclude_lines(input, &["error", "||", "warning"]);
        assert_eq!(result, "info: success\n");
    }

    #[test]
    fn test_exclude_lines_and_condition() {
        let input = "error: network failed\ninfo: success\nerror: timeout\nwarning: network slow\n".to_string();
        let result = exclude_lines(input, &["error", "&", "network"]);
        assert_eq!(result, "info: success\nerror: timeout\nwarning: network slow\n");
    }

    #[test]
    fn test_slice_single_line() {
        let input = "line 1\nline 2\nline 3\nline 4\n".to_string();
        let result = slice_lines(input, &["2"]);
        assert_eq!(result, "line 2\n");
    }

    #[test]
    fn test_slice_range() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n".to_string();
        let result = slice_lines(input, &["2:4"]);
        assert_eq!(result, "line 2\nline 3\nline 4\n");
    }

    #[test]
    fn test_slice_from_to_end() {
        let input = "line 1\nline 2\nline 3\nline 4\n".to_string();
        let result = slice_lines(input, &["3:"]);
        assert_eq!(result, "line 3\nline 4\n");
    }

    #[test]
    fn test_slice_first_line() {
        let input = "line 1\nline 2\nline 3\n".to_string();
        let result = slice_lines(input, &["1"]);
        assert_eq!(result, "line 1\n");
    }

    #[test]
    fn test_slice_negative_index() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n".to_string();
        let result = slice_lines(input, &["-1"]);
        assert_eq!(result, "line 5\n");
    }

    #[test]
    fn test_slice_negative_range() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n".to_string();
        let result = slice_lines(input, &["-3:-1"]);
        assert_eq!(result, "line 3\nline 4\nline 5\n");
    }

    #[test]
    fn test_slice_negative_start() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n".to_string();
        let result = slice_lines(input, &["-2:"]);
        assert_eq!(result, "line 4\nline 5\n");
    }

    #[test]
    fn test_choose_single_column() {
        let input = "col1 col2 col3\nrow1a row1b row1c\nrow2a row2b row2c\n".to_string();
        let result = choose_columns(input, &["2"]);
        assert_eq!(result, "col2\nrow1b\nrow2b\n");
    }

    #[test]
    fn test_choose_multiple_columns() {
        let input = "col1 col2 col3 col4\nrow1a row1b row1c row1d\n".to_string();
        let result = choose_columns(input, &["1", "3"]);
        assert_eq!(result, "col1 col3\nrow1a row1c\n");
    }

    #[test]
    fn test_choose_range() {
        let input = "col1 col2 col3 col4 col5\nrow1a row1b row1c row1d row1e\n".to_string();
        let result = choose_columns(input, &["2:4"]);
        assert_eq!(result, "col2 col3 col4\nrow1b row1c row1d\n");
    }

    #[test]
    fn test_choose_from_to_end() {
        let input = "col1 col2 col3 col4\nrow1a row1b row1c row1d\n".to_string();
        let result = choose_columns(input, &["3:"]);
        assert_eq!(result, "col3 col4\nrow1c row1d\n");
    }

    #[test]
    fn test_choose_with_irregular_spacing() {
        let input = "col1  col2   col3\nrow1a    row1b row1c\n".to_string();
        let result = choose_columns(input, &["2"]);
        assert_eq!(result, "col2\nrow1b\n");
    }

    #[test]
    fn test_choose_negative_index() {
        let input = "col1 col2 col3 col4 col5\nrow1a row1b row1c row1d row1e\n".to_string();
        let result = choose_columns(input, &["-1"]);
        assert_eq!(result, "col5\nrow1e\n");
    }

    #[test]
    fn test_choose_negative_range() {
        let input = "col1 col2 col3 col4 col5\nrow1a row1b row1c row1d row1e\n".to_string();
        let result = choose_columns(input, &["-3:-1"]);
        assert_eq!(result, "col3 col4 col5\nrow1c row1d row1e\n");
    }

    #[test]
    fn test_choose_negative_multiple() {
        let input = "col1 col2 col3 col4 col5\nrow1a row1b row1c row1d row1e\n".to_string();
        let result = choose_columns(input, &["-2", "-4"]);
        assert_eq!(result, "col4 col2\nrow1d row1b\n");
    }

    #[test]
    fn test_choose_mixed_positive_negative() {
        let input = "col1 col2 col3 col4 col5\nrow1a row1b row1c row1d row1e\n".to_string();
        let result = choose_columns(input, &["1", "-1"]);
        assert_eq!(result, "col1 col5\nrow1a row1e\n");
    }

    #[test]
    fn test_unique_basic() {
        let input = "line 1\nline 2\nline 1\nline 3\nline 2\n".to_string();
        let result = unique_lines(input, &[]);
        assert_eq!(result, "line 1\nline 2\nline 3\n");
    }

    #[test]
    fn test_unique_all_unique() {
        let input = "line 1\nline 2\nline 3\n".to_string();
        let result = unique_lines(input, &[]);
        assert_eq!(result, "line 1\nline 2\nline 3\n");
    }

    #[test]
    fn test_unique_all_duplicates() {
        let input = "same\nsame\nsame\nsame\n".to_string();
        let result = unique_lines(input, &[]);
        assert_eq!(result, "same\n");
    }

    #[test]
    fn test_unique_preserves_order() {
        let input = "apple\nbanana\napple\ncherry\nbanana\napple\n".to_string();
        let result = unique_lines(input, &[]);
        assert_eq!(result, "apple\nbanana\ncherry\n");
    }

    #[test]
    fn test_unique_empty_lines() {
        let input = "line 1\n\nline 2\n\nline 1\n".to_string();
        let result = unique_lines(input, &[]);
        assert_eq!(result, "line 1\n\nline 2\n");
    }

    #[test]
    fn test_pipeline_filter_then_slice() {
        let input = "error line 1\ninfo line 2\nerror line 3\ninfo line 4\nerror line 5\n".to_string();
        let filtered = filter_lines(input, &["error"]);
        let result = slice_lines(filtered, &["1:2"]);
        assert_eq!(result, "error line 1\nerror line 3\n");
    }

    #[test]
    fn test_pipeline_choose_then_filter() {
        let input = "col1 error data1\ncol2 info data2\ncol3 error data3\n".to_string();
        let chosen = choose_columns(input, &["2"]);
        let result = filter_lines(chosen, &["error"]);
        assert_eq!(result, "error\nerror\n");
    }

    #[test]
    fn test_pipeline_unique_then_slice() {
        let input = "apple\nbanana\napple\ncherry\nbanana\ndateapple\n".to_string();
        let unique = unique_lines(input, &[]);
        let result = slice_lines(unique, &["2:3"]);
        assert_eq!(result, "banana\ncherry\n");
    }
}

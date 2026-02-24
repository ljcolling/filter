pub fn filter_lines(input: String, args: &[&str]) -> String {
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

pub fn exclude_lines(input: String, args: &[&str]) -> String {
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

pub fn slice_lines(input: String, args: &[&str]) -> String {
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

pub fn choose_columns(input: String, args: &[&str]) -> String {
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

pub fn unique_lines(input: String, _args: &[&str]) -> String {
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

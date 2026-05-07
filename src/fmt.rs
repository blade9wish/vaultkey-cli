//! Output formatting module for vaultkey-cli.
//! Supports table, JSON, and plain text output formats.

use crate::error::VaultKeyError;
use serde::Serialize;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Plain,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self, VaultKeyError> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            "plain" => Ok(OutputFormat::Plain),
            other => Err(VaultKeyError::Config(format!(
                "Unknown output format: '{}'. Valid options: table, json, plain",
                other
            ))),
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Table
    }
}

/// Render a list of key-value pairs as a formatted table.
pub fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::from("(no entries)\n");
    }
    let col_count = headers.len();
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < col_count {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }
    let mut out = String::new();
    let separator: String = widths.iter().map(|w| "-".repeat(w + 2)).collect::<Vec<_>>().join("+");
    let _ = writeln!(out, "+{}+", separator);
    let header_row: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!(" {:width$} ", h, width = widths[i]))
        .collect::<Vec<_>>()
        .join("|");
    let _ = writeln!(out, "|{}|", header_row);
    let _ = writeln!(out, "+{}+", separator);
    for row in rows {
        let cells: String = (0..col_count)
            .map(|i| {
                let cell = row.get(i).map(String::as_str).unwrap_or("");
                format!(" {:width$} ", cell, width = widths[i])
            })
            .collect::<Vec<_>>()
            .join("|");
        let _ = writeln!(out, "|{}|", cells);
    }
    let _ = writeln!(out, "+{}+", separator);
    out
}

/// Render a serializable value as JSON.
pub fn render_json<T: Serialize>(value: &T) -> Result<String, VaultKeyError> {
    serde_json::to_string_pretty(value)
        .map_err(|e| VaultKeyError::Config(format!("JSON serialization error: {}", e)))
}

/// Render key-value pairs as plain text (one per line).
pub fn render_plain(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

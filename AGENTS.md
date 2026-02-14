# AGENTS.md

## Mandatory Instruction

If you generate code or modify files, always run the GitButler update branches MCP tool first.
If an operation is not possible with the GitButler MCP tools available in this environment, use the GitButler CLI (`but`) before falling back to `git`.
Use `git` only as a last resort when neither GitButler MCP nor `but` can perform the required action.

This file provides guidance to AI coding agents working in this repository.

## Project Overview

CalWeek Finder is a Rust CLI tool that converts between dates and "calweeks" (a 4-digit format: 2 digits for year + 2 digits for week number). Example: "2348" = week 48 of 2023.

## Core Functionality

The application has three modes:

1. **CLI with arguments**: Direct conversion via command line arguments
2. **Interactive TUI**: Menu-driven interface when no arguments provided
3. **Current calweek**: Returns today's calweek when passed "today"

## Architecture

- `main.rs`: Entry point with argument parsing logic
- `calweek.rs`: Core calweek conversion functions using ISO week numbering
- `dates.rs`: Date parsing with support for multiple formats (%Y-%m-%d, %d/%m/%Y, %m/%d/%Y, %d-%m-%Y, %d.%m.%Y)
- `ui.rs`: Interactive terminal interface using dialoguer

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run` (interactive mode) or `cargo run <argument>`
- **Test**: `cargo test`
- **Release build**: `cargo build --release`

## Dependencies

- `chrono`: Date/time handling and ISO week calculations
- `dialoguer`: Interactive CLI prompts and menus

## Input Handling Logic

- No arguments → Interactive TUI mode
- "today" → Current calweek
- 4-digit string → Calweek to date range conversion
- Other strings → Date parsing attempt across supported formats

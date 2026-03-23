# Architecture

CalWeek Finder follows a **3-layer architecture** where each layer has a single, well-defined responsibility. No layer skips another.

```text
┌─────────────────────────────────────────────┐
│  Presentation  │  main.rs · ui.rs           │
├─────────────────────────────────────────────┤
│  Application   │  app.rs                    │
├─────────────────────────────────────────────┤
│  Domain        │  calweek.rs · dates.rs     │
└─────────────────────────────────────────────┘
```

---

## Layers

### Domain (`calweek.rs`, `dates.rs`)

Pure logic with no I/O or formatting dependencies.

- `calweek.rs` — conversions between dates and calweeks (ISO week numbering)
- `dates.rs` — date string parsing across multiple formats

Rules:

- No `Display` implementations — domain structs are plain data
- No dependency on clap, dialoguer, or any I/O crate
- Functions are pure: same input always produces same output

### Application (`app.rs`)

Orchestration layer. The single entry point into the domain for all frontends.

- Receives raw string inputs
- Classifies and routes to the correct domain function
- Returns a unified `AppOutput` enum consumed by the presentation layer

Both `main.rs` (CLI) and `ui.rs` (TUI) call this layer. Neither calls the domain directly.

```rust
pub enum AppOutput {
    Calweek(CalWeekResult),
    WeekRange(WeekRangeResult),
    WeekDay(CalWeekDayResult),
}
```

Public API:

- `process_input(input: &str)` — auto-detects format: "today", YYWW, YYWWD, or date
- `process_date(date_str: &str)` — explicit date → calweek conversion
- `process_week(week_str: &str)` — 4-digit week range or 5-digit specific day
- `process_today()` — returns current calweek

### Presentation (`main.rs`, `ui.rs`)

Handles all I/O: argument parsing, user prompts, and output formatting.

- `main.rs` — clap argument parsing, JSON/plain output, exit codes
- `ui.rs` — interactive dialoguer menus and prompts
- `format_output(output: &AppOutput) -> String` — single source of truth for plain text formatting (defined in `main.rs`, used by both `main.rs` and `ui.rs`)

---

## Data flow

```text
CLI path:
  args → main.rs (clap) → app::process_*() → AppOutput → format_output() → stdout

TUI path:
  prompts → ui.rs (dialoguer) → app::process_*() → AppOutput → format_output() → stdout
```

Both paths converge at `app.rs` before reaching the domain. Any logic added to the application layer (validation, logging, etc.) automatically applies to both frontends.

---

## File structure

```text
src/
  main.rs          Crate root: CLI parsing, output formatting, exit codes
  app.rs           Application layer: orchestration and use cases
  calweek.rs       Domain: calweek ↔ date conversions
  dates.rs         Domain: date string parsing
  ui.rs            Presentation: interactive TUI
  app_tests.rs     Tests for the application layer
  calweek_tests.rs Tests for the domain (calweek)
  dates_tests.rs   Tests for the domain (dates)
  main_tests.rs    Tests for CLI parsing and output formatting
```

Test files are linked via `#[cfg(test)] #[path = "..."] mod tests;` in each source file.

---

## Key design decisions

**No `Display` on domain structs.** `CalWeekResult`, `WeekRangeResult`, and `CalWeekDayResult` are plain data structs. Formatting is the presentation layer's responsibility, centralised in `format_output()`.

**`ui.rs` does not access the domain directly.** Before this architecture was adopted, `ui.rs` called `calweek.rs` and `dates.rs` directly, creating two parallel execution paths that could diverge. Now both frontends go through `app.rs`.

**5-digit calweek support is transparent to the TUI.** The TUI previously only supported 4-digit calweeks because it called `get_monday_and_sunday()` directly. Now it calls `app::process_week()`, which handles both 4-digit (week range) and 5-digit (specific day) inputs automatically.

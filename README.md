# Hexplore

An hexadecimal editor tailored for reverse-engineering tasks written in Rust.

# TODO

- [x] Basic hexdump
- [x] Add basic Ratatui display (TUI rendered)
- [x] Add help popup with all keybinds
- [x] Add scrollbar support
- [x] Add resize event handling
- [ ] Add different event handling when popup is visible
- [x] Add file information (sha, `file` output, size, filename) popup
- [ ] Performance: lazy load file content (read when displayed and not try to read/store whole file content)
- [ ] Add confirm exit dialog
- [ ] Add style to TUI application
- [ ] Add command prompt (triggered by `SPACE`)
- [ ] Add logs
- [ ] Add edit features (terminal raw-mode)
- [ ] Add disassembly mode (based on Capstone)
- [ ] Patch assembly
- [ ] Keybind configuration

@AGENTS.md

## Claude Code

- Use plan mode for anything that changes the public API or the module layout.
- You may fetch Home Assistant docs and GitHub source files to verify fields and
  enum values. Do this instead of relying on memory; the schema changes often.
- In the devcontainer, `CLAUDE_CONFIG_DIR` is `/home/vscode/.claude`, a named
  volume that survives rebuilds. Settings and memory stored there persist; other
  changes outside `/workspace` do not.

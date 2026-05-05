# RTL Mouse Click Fix: Attempted, Not Viable in Current VS Code/Cursor

**Date**: 2026-02-10
**Status**: Not implemented. Removed from extension. Deferred post-launch (tracked in PUBLISHING_ROADMAP.mdc).

## The Problem

When you click past the end of text on a right-to-left line in VS Code/Cursor, the
cursor jumps to position 0 (the logical start of the line, which is the visual RIGHT
for RTL text). The user expects the cursor at the visual position nearest to where
they clicked -- which for a click on the left side past the text means the logical
END of the line (visual LEFT).

This is a VS Code core behavior. VS Code's editor maps mouse coordinates to logical
positions without considering paragraph direction.

## Approaches Attempted

### Approach 1: Direct selection assignment on Mouse kind
Listen for `onDidChangeTextEditorSelection` with `e.kind === Mouse`, detect cursor
at position 0 on an RTL line, set `editor.selection` to end of line.
**Result**: No effect. Cursor/VS Code may not report `Mouse` kind reliably; `e.kind`
is often `undefined` in Cursor forks.

### Approach 2: Broader kind filter + direct selection
Changed to exclude only `Keyboard` kind (allowing `undefined` and `Mouse` through).
Added try/catch around WASM call, re-verification in timeout, 20ms debounce.
**Result**: No effect. Direct `editor.selection` assignment during mouse event
processing appears to be overwritten by VS Code's internal selection finalization.

### Approach 3: Regex RTL detection + cursorEnd command
Removed WASM dependency entirely (regex for RTL character ranges). Used
`vscode.commands.executeCommand('cursorEnd')` via VS Code's command pipeline instead
of direct selection. Re-entrancy guard, 30ms initial delay + 50ms cooldown.
**Result**: No effect. The VS Code/Cursor mouse click processing pipeline does not
appear to yield to extension-initiated cursor corrections regardless of mechanism.

## Root Cause Analysis

VS Code's mouse click handling is deeply integrated into its editor core (Monaco).
The click-to-position mapping, including the "past end of line" heuristic, happens
at a layer below the extension API. The `onDidChangeTextEditorSelection` event fires
after VS Code has finalized the cursor position, but:

1. **Kind reporting is unreliable** in Cursor and possibly some VS Code versions
2. **Selection assignments are overwritten** by subsequent internal processing steps
3. **Command-based cursor moves** (`cursorEnd`) also appear to be superseded

This would likely require a VS Code core patch or a Monaco editor overlay to fix
properly, which is beyond extension API capabilities.

## Recommendation

- Track as a known limitation
- File upstream if VS Code gains an API for mouse click interception or post-click
  cursor correction
- Revisit if Cursor exposes additional editor hooks in future versions
- The keyboard-based visual cursor (arrow keys follow visual direction) works correctly
  and covers the primary RTL editing workflow

## Reflection: HAIMU Process

The human developer proposed a clean heuristic: detect position 0 on RTL lines after
mouse clicks and correct to line end. The logic was sound and the AI implemented three
progressively different approaches. The limitation was not in the heuristic's logic but
in VS Code's extension API not providing sufficient control over mouse-driven cursor
placement. This is an example where HAIMU correctly identified and simplified the
problem, but the platform constraint prevented the solution regardless of implementation
strategy. The decision to remove and defer was appropriate -- shipping non-functional
code would mislead users.

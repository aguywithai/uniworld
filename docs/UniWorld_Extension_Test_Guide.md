# UniWorld VS Code Extension -- Test Guide

This document contains test content for each UniWorld toggle and command.
Open this file in Cursor with the UniWorld extension installed, then follow
the instructions in each section.

---

## 1. Grapheme Cursor (uniworld.enableGraphemeCursor)

**Setting**: Settings > UniWorld > Enable Grapheme Cursor
**What it does**: Left/Right arrow keys move by grapheme cluster instead of
by UTF-16 code unit. Prevents the cursor from landing inside multi-codepoint
characters.

### Test 1a: Emoji ZWJ sequences

Place your cursor before the first emoji and press Right repeatedly.

Family emoji: 👨‍👩‍👧‍👦 done
Flag emoji: 🇦🇺 done
Skin tone: 👋🏽 done

**Without toggle**: cursor may take 7+ Right presses to cross the family emoji,
landing on invisible ZWJ and variation selector characters in between.

**With toggle ON**: each Right press jumps over the entire emoji cluster in one
step. Three Right presses from the start of the line should reach "done":
Right 1 = past family, Right 2 = past space, Right 3 = on "d".

### Test 1b: Combining marks (accents)

Place cursor before each word and press Right.

Cafe with combining accent: cafe\u0301 (cafe + combining acute)
Precomposed form: caf\u00e9

**Without toggle**: in the combining form, cursor stops between "e" and the
combining acute accent, which is not a meaningful position.

**With toggle ON**: cursor moves from "f" directly past the full "e + accent"
cluster in one step. Both words take 4 Right presses to traverse.

### Test 1c: Devanagari conjuncts

Place cursor before the word and press Right.

Hindi: हिन्दी (2 grapheme clusters: हि + न्दी)
Sanskrit: संस्कृतम् (complex conjuncts)

**Without toggle**: cursor may stop in the middle of conjuncts, breaking
the visual ligature. Multiple presses needed to cross each cluster.

**With toggle ON**: cursor moves cleanly between each visual cluster.
For हिन्दी, exactly 2 Right presses should traverse the word (the
virama and vowel signs bind multiple code points into single clusters).

---

## 2. Grapheme Delete (uniworld.enableGraphemeDelete)

**Setting**: Settings > UniWorld > Enable Grapheme Delete
**What it does**: Backspace/Delete removes an entire grapheme cluster in one
keypress instead of removing individual code points.

### Test 2a: Backspace on emoji

Type or paste this emoji at end of a line, then press Backspace:

Test: 👨‍👩‍👧‍👦

**Without toggle**: Backspace removes one code point at a time -- you'd need
7 presses and see broken partial emoji in between.

**With toggle ON**: single Backspace deletes the entire family emoji.

### Test 2b: Delete on combining text

Place cursor before the accent in this word and press Delete:

cafe\u0301

**Without toggle**: Delete removes just the combining acute, orphaning it.

**With toggle ON**: Delete removes the entire "e + accent" cluster.

---

## 3. Visual Bidi Cursor (uniworld.enableBidiVisualCursor)

**Setting**: Settings > UniWorld > Enable Bidi Visual Cursor
**What it does**: Left/Right arrow keys follow the visual (screen) direction
rather than the logical (memory) direction. In RTL text, Left moves visually
left, Right moves visually right.

**NOTE**: This setting takes precedence over Enable Grapheme Cursor when both
are on. It includes grapheme-aware movement.

### Test 3a: Pure Arabic text

Place cursor at the start of this line and press Right repeatedly:

مرحبا بالعالم

This is pure RTL text (paragraph level = RTL). Visual display order is
right-to-left.

**Without toggle**: Right arrow moves through characters in memory order
(logically right), which is visually LEFT in RTL text -- confusing.

**With toggle ON**: Right arrow moves visually right on screen. Since the
text is RTL, the cursor should move from the rightmost character toward the
left, one character at a time. Each press moves one Arabic letter.

### Test 3b: Pure Hebrew text

שלום עולם

Same behavior as Arabic. Right should move visually right (toward line end).
Left should move visually left (toward line start).

### Test 3c: Mixed LTR + RTL on one line

Hello مرحبا World

This line has three runs: LTR ("Hello "), RTL ("مرحبا"), LTR (" World").
Visual display: Hello ابحرم World (Arabic is reversed on screen).

**With toggle ON**: pressing Right from the start of the line will:
1. Move through "H-e-l-l-o- " (LTR, normal left-to-right),
2. Enter the Arabic run and continue rightward visually (which is logically
   backward through the Arabic word),
3. Continue through " -W-o-r-l-d" (LTR again).

The cursor should move smoothly one visual position at a time through all
three runs without skipping or \"bouncing\" at the LTR/RTL boundaries.

### Test 3d: Numbers inside RTL

ثمن: 42 دولار

Numbers (European digits) embedded in Arabic text have even embedding level.
With the visual cursor toggle ON, the cursor should:

1. Enter the Arabic text from either side without getting stuck at the word
   edges,
2. Move through \"42\" left-to-right (visually) even though it is embedded in
   an RTL paragraph,
3. Never jump back over the digits once it has traversed them.

If you see the cursor skipping the digits or oscillating at the boundary,
that indicates a regression in the visual bidi cursor logic.

### Typing in RTL: what to expect

UniWorld only overrides **Left and Right arrow keys**. It does not change how
typing works. Inserting characters is handled by the editor and OS:

- When you type with an **RTL keyboard** (e.g. Arabic or Hebrew), each
  character is inserted at the current logical position and the cursor
  advances one position in logical order. In RTL text that means the
  cursor moves visually in the direction you are typing (e.g. to the left
  when typing Arabic left-to-right on screen), which is the normal and
  natural behavior.
- So: **navigation** with arrow keys is visual (UniWorld); **typing** is
  logical (editor default). The two work together: you move with arrows
  in visual order, and when you type, the new character goes in at the
  cursor and the cursor advances in the usual way for that script.

**How to test typing without an RTL keyboard:**

1. **Add an RTL layout**: Windows Settings > Time & Language > Language &
   Region > Add a language (e.g. Arabic or Hebrew). Switch input with
   Win+Space or the taskbar language indicator. Type in the test lines
   above; the cursor should advance in the direction of typing and new
   characters should appear in the correct visual place.
2. **Paste and insert**: Paste RTL text (e.g. مرحبا), click in the middle,
   then type a Latin letter (e.g. X). The X should appear at the
   insertion point and the cursor should move one position logically.
   Pressing Left/Right afterward should move one visual step (UniWorld).

---

## 4. Line Break Decorations (uniworld.showLineBreakOpportunities)

**Setting**: Settings > UniWorld > Show Line Break Opportunities
**Also**: Ctrl+Shift+P > UniWorld: Toggle Line Break Opportunity Decorations

**What it does**: Shows subtle grey dots at every position where UAX #14 says
a line break is allowed. Red dots for mandatory breaks. Includes
dictionary-based word breaks for Thai/Lao/Khmer/Myanmar.

### Test 4a: English text

The quick brown fox jumps over the lazy dog.

**With toggle ON**: grey dots should appear between each word (after each
space), showing where a line could wrap. No dots inside words.

### Test 4b: CJK text (break between any characters)

漢字テスト中文测试

**With toggle ON**: dots should appear between every character, because CJK
allows line breaks between any adjacent characters.

### Test 4c: Thai text (dictionary-based breaks)

สวัสดีครับภาษาไทยไม่มีช่องว่าง

Thai has no spaces between words. Without UniWorld, editors break Thai text
at arbitrary positions.

**With toggle ON**: dots should appear at word boundaries detected by
UniWorld's dictionary segmentation, not between every character.

### Test 4d: URL (no breaks inside)

Visit https://github.com/aguywithai/uniworld for more.

**With toggle ON**: ideally dots should not appear inside the URL. However,
UAX #14 allows breaks after certain punctuation (slashes, hyphens) within
URLs. You may see dots after "/" characters. This is spec-compliant behavior
for the default line break algorithm (strict URL non-breaking requires
higher-level context not part of UAX #14).

---

## 5. Word Selection (UniWorld: Select Word at Cursor)

**Command**: Ctrl+Shift+P > UniWorld: Select Word at Cursor

**What it does**: Selects the word at the current cursor position using
UniWorld's UAX #29 word boundaries, which respect script-specific rules.

### Test 5a: English word

Place cursor anywhere inside "jumps" below and run the command:

The quick brown fox jumps over the lazy dog.

**Expected**: "jumps" is selected (same as VS Code default for English).

### Test 5b: Thai word selection

Place cursor in the middle of the Thai text and run the command:

สวัสดีครับภาษาไทยไม่มีช่องว่าง

**Without UniWorld**: VS Code selects the entire Thai string because it has
no spaces.

**With UniWorld**: the command should select a single Thai word
(e.g., "สวัสดี" = hello, "ครับ" = polite particle).

### Test 5c: CJK word selection

Place cursor on one of the characters:

東京都港区

**With UniWorld**: should select meaningful word units rather than single
characters or the entire string.

### Test 5d: Hyphenated word

Place cursor on "well" below:

This is a well-known fact.

**Expected**: "well" is selected (hyphen is a word boundary).

---

## 6. Other Commands (always available, no toggle needed)

### 6a: Inspect Selection

Select any text below and run Ctrl+Shift+P > UniWorld: Inspect Selection

Sample: Hello 世界 مرحبا 🌍

**Expected**: Opens a new document showing codepoints, grapheme clusters,
word/sentence boundaries, and display width for the selected text.

### 6b: Display Width in Status Bar

Click on different lines and check the status bar (bottom right).

Narrow:  hello (5w 5g)
Wide:    你好世界 (8w 4g -- CJK characters are 2 columns each)
Emoji:   👨‍👩‍👧‍👦 (2w 1g -- one cluster, 2 columns wide)
Mixed:   A你B (4w 3g)

The status bar (bottom right) should show "UW Line: Xw Yg" where X is the
display width in terminal columns and Y is the grapheme cluster count.
Look for "UW" text -- it appears at the far right of the status bar.
If your status bar is crowded, you may need to widen the window or look
carefully for it among other status items.

### 6c: Normalize NFC

Select the text below (which uses combining accent) and run
Ctrl+Shift+P > UniWorld: Normalize NFC:

cafe\u0301

**Expected**: replaced with "caf\u00e9" (precomposed form). Code unit count
decreases from 5 to 4.

### 6d: Truncate to Display Width

Select the line below and run Ctrl+Shift+P > UniWorld: Truncate to Display
Width. Enter "6" when prompted:

Hello 世界！

**Expected**: truncated to "Hello " (6 display columns). The CJK character
is 2 columns wide and would exceed 6, so it is excluded without splitting.

---

## 7. Hover Inspector

Hover your mouse over any character and wait for the tooltip.

Test characters:

A  (U+0041, width 1)
あ (U+3042, width 2)
🌍 (U+1F30D, width 2)
\u0301 (combining acute accent, width 0)

**Expected**: tooltip shows "UniWorld Unicode inspector" with the cluster
text, codepoints, and display width.

---

## Summary of toggles

| # | Setting | Default | Key override |
|---|---------|---------|-------------|
| 1 | enableGraphemeCursor | off | Left/Right |
| 2 | enableGraphemeDelete | off | Backspace/Delete |
| 3 | enableBidiVisualCursor | off | Left/Right (takes precedence over #1) |
| 4 | showLineBreakOpportunities | off | none (visual only) |
| 5 | enableGraphemeWordSelect | off | none (command only for now) |

Enable one toggle at a time to isolate behavior changes, especially #1 vs #3
which both override Left/Right.

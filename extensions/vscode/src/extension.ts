import * as vscode from 'vscode';
import * as path from 'path';

// UniWorld WASM module -- loaded lazily on first use
let uniworld: typeof import('../wasm/uniworld') | null = null;
let wasmLoadFailed = false;

// Shared encoders/decoders for UTF-8 <-> UTF-16 conversion
const utf8Encoder = new TextEncoder();
const utf8Decoder = new TextDecoder();

/**
 * Load the UniWorld WASM module from the extension's wasm/ directory.
 * Returns the module or null if loading fails.
 */
function loadWasm(context: vscode.ExtensionContext): typeof import('../wasm/uniworld') | null {
    if (uniworld) {
        return uniworld;
    }
    if (wasmLoadFailed) {
        return null;
    }
    try {
        // The wasm-pack nodejs build uses require('fs') and __dirname internally.
        // We override __dirname so it finds the .wasm file next to the .js glue.
        const wasmDir = path.join(context.extensionPath, 'wasm');
        const originalDirname = (global as any).__dirname;
        (global as any).__dirname = wasmDir;
        try {
            uniworld = require(path.join(wasmDir, 'uniworld.js'));
        } finally {
            (global as any).__dirname = originalDirname;
        }
        return uniworld;
    } catch (err: unknown) {
        wasmLoadFailed = true;
        const msg = err instanceof Error ? err.message : String(err);
        console.error('UniWorld: failed to load WASM module:', msg);
        return null;
    }
}

let statusBarItem: vscode.StatusBarItem;
let extensionContext: vscode.ExtensionContext;

// Line break decoration types (Phase 3)
let lineBreakDecorationType: vscode.TextEditorDecorationType | null = null;
let mandatoryBreakDecorationType: vscode.TextEditorDecorationType | null = null;
let lineBreakActive = false;

// Bidi visualization decoration types (Phase 3)
let bidiLtrDecorationType: vscode.TextEditorDecorationType | null = null;
let bidiRtlDecorationType: vscode.TextEditorDecorationType | null = null;
let bidiVisualizationActive = false;

export function activate(context: vscode.ExtensionContext): void {
    extensionContext = context;
    console.log('UniWorld extension activating from', context.extensionPath);

    // Load WASM eagerly so status bar works immediately
    let wasm: ReturnType<typeof loadWasm> = null;
    try {
        wasm = loadWasm(context);
        if (wasm) {
            console.log('UniWorld WASM module loaded successfully');
        } else {
            console.warn('UniWorld WASM module not available; using fallbacks');
        }
    } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err);
        console.error('UniWorld: WASM load threw during activation:', msg);
    }

    // Status bar: display width of current selection or line
    // Use Left alignment at high priority so it appears near the left of the
    // status bar, making it easier to spot in busy status bars (like Cursor).
    statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Left,
        1000
    );
    statusBarItem.tooltip = 'UniWorld: Display Width (true terminal columns)';
    statusBarItem.command = 'uniworld.inspectSelection';
    context.subscriptions.push(statusBarItem);

    // Update status bar on selection change
    context.subscriptions.push(
        vscode.window.onDidChangeTextEditorSelection(updateStatusBar)
    );
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(() => updateStatusBar())
    );

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.inspectSelection', inspectSelection)
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.truncateToDisplayWidth', truncateToDisplayWidth)
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.normalizeNFC', () => normalizeSelection('NFC'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.normalizeNFD', () => normalizeSelection('NFD'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.normalizeNFKC', () => normalizeSelection('NFKC'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.normalizeNFKD', () => normalizeSelection('NFKD'))
    );

    // Grapheme-aware cursor commands (Phase 2)
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.cursorLeftGrapheme', () => moveCursorByGrapheme('left'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.cursorRightGrapheme', () => moveCursorByGrapheme('right'))
    );

    // Grapheme-aware delete commands (Phase 2)
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.deleteLeftGrapheme', () => deleteByGrapheme('left'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.deleteRightGrapheme', () => deleteByGrapheme('right'))
    );

    // Visual bidi cursor commands (Phase 3)
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.cursorLeftVisual', () => moveCursorVisual('left'))
    );
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.cursorRightVisual', () => moveCursorVisual('right'))
    );

    // Grapheme-aware word selection command (Phase 3)
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.selectWord', selectWordAtCursor)
    );

    // Line break decorations toggle (Phase 3)
    context.subscriptions.push(
        vscode.commands.registerCommand('uniworld.toggleLineBreakDecorations', toggleLineBreakDecorations)
    );

    // Hover inspector (Phase 2)
    context.subscriptions.push(
        vscode.languages.registerHoverProvider({ scheme: '*', language: '*' }, {
            provideHover(document, position) {
                return provideUniWorldHover(document, position);
            }
        })
    );

    // Listen for config changes to update line break and bidi decorations
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration('uniworld.showLineBreakOpportunities')) {
                const enabled = vscode.workspace.getConfiguration('uniworld')
                    .get<boolean>('showLineBreakOpportunities', false);
                if (enabled && !lineBreakActive) {
                    activateLineBreakDecorations();
                } else if (!enabled && lineBreakActive) {
                    deactivateLineBreakDecorations();
                }
            }
            if (e.affectsConfiguration('uniworld.showBidiVisualization')) {
                const enabled = vscode.workspace.getConfiguration('uniworld')
                    .get<boolean>('showBidiVisualization', false);
                if (enabled && !bidiVisualizationActive) {
                    activateBidiVisualization();
                } else if (!enabled && bidiVisualizationActive) {
                    deactivateBidiVisualization();
                }
            }
            if (e.affectsConfiguration('uniworld.bidiHighlightOpacity')) {
                if (bidiVisualizationActive) {
                    deactivateBidiVisualization();
                    activateBidiVisualization();
                }
            }
        })
    );

    // Listen for document/editor changes to refresh line break and bidi decorations
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(() => {
            if (lineBreakActive) { refreshLineBreakDecorations(); }
            if (bidiVisualizationActive) { refreshBidiDecorations(); }
        })
    );
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(() => {
            if (lineBreakActive) { refreshLineBreakDecorations(); }
            if (bidiVisualizationActive) { refreshBidiDecorations(); }
        })
    );
    context.subscriptions.push(
        vscode.window.onDidChangeTextEditorVisibleRanges(() => {
            if (lineBreakActive) { refreshLineBreakDecorations(); }
            if (bidiVisualizationActive) { refreshBidiDecorations(); }
        })
    );

    // Initialize line break decorations if setting is already enabled
    const lbEnabled = vscode.workspace.getConfiguration('uniworld')
        .get<boolean>('showLineBreakOpportunities', false);
    if (lbEnabled) {
        activateLineBreakDecorations();
    }

    // Initialize bidi visualization if setting is already enabled
    const bidiEnabled = vscode.workspace.getConfiguration('uniworld')
        .get<boolean>('showBidiVisualization', false);
    if (bidiEnabled) {
        activateBidiVisualization();
    }

    statusBarItem.show();
    updateStatusBar();
}

export function deactivate(): void {
    deactivateLineBreakDecorations();
    deactivateBidiVisualization();
}

// ---------------------------------------------------------------------------
// Status bar: true display width via WASM
// ---------------------------------------------------------------------------

function updateStatusBar(_event?: vscode.TextEditorSelectionChangeEvent): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        statusBarItem.hide();
        return;
    }

    const selection = editor.selection;
    const text = selection.isEmpty
        ? editor.document.lineAt(selection.active.line).text
        : editor.document.getText(selection);

    const wasm = loadWasm(extensionContext);
    const label = selection.isEmpty ? 'Line' : 'Sel';

    if (wasm) {
        const width = wasm.display_width(text);
        const graphemes = wasm.grapheme_boundaries(text);
        // grapheme_boundaries returns byte offsets; count of clusters = offsets.length - 1
        // (or 0 for empty text)
        const clusterCount = graphemes.length > 0 ? graphemes.length - 1 : 0;
        statusBarItem.text = `UW ${label}: ${width}w ${clusterCount}g`;
        statusBarItem.tooltip =
            `UniWorld: ${width} display columns, ${clusterCount} grapheme clusters` +
            `\n${text.length} UTF-16 code units, ${[...text].length} code points`;
    } else {
        // Fallback: JS character count
        const charCount = [...text].length;
        statusBarItem.text = `UW ${label}: ${charCount} chars`;
        statusBarItem.tooltip = 'UniWorld: WASM not loaded; showing code point count';
    }
    statusBarItem.show();
}

// ---------------------------------------------------------------------------
// Inspect Selection: codepoints, grapheme clusters, display width
// ---------------------------------------------------------------------------

async function inspectSelection(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const text = editor.document.getText(editor.selection);
    if (!text) {
        vscode.window.showWarningMessage('No text selected');
        return;
    }

    const wasm = loadWasm(extensionContext);
    const lines: string[] = [];

    lines.push(`UniWorld: Inspect Selection`);
    lines.push(`==========================`);
    lines.push(``);
    lines.push(`Text: "${text}"`);
    lines.push(`UTF-16 code units: ${text.length}`);
    lines.push(`Code points: ${[...text].length}`);
    lines.push(``);

    // Codepoint listing
    const codepoints = [...text].map(ch => {
        const cp = ch.codePointAt(0);
        if (cp === undefined) { return '?'; }
        const hex = `U+${cp.toString(16).toUpperCase().padStart(4, '0')}`;
        // Show printable ASCII name hints
        return `${hex} (${cp >= 0x20 && cp < 0x7f ? ch : '.'})`;
    });
    lines.push(`Code points: ${codepoints.join(', ')}`);
    lines.push(``);

    if (wasm) {
        // Display width
        const width = wasm.display_width(text);
        lines.push(`Display width: ${width} columns`);

        // Grapheme clusters
        const offsets = wasm.grapheme_boundaries(text);
        const utf8 = utf8Encoder.encode(text);
        const clusterTexts: string[] = [];
        for (let i = 0; i < offsets.length - 1; i++) {
            const slice = utf8.slice(offsets[i], offsets[i + 1]);
            clusterTexts.push(utf8Decoder.decode(slice));
        }
        lines.push(`Grapheme clusters (${clusterTexts.length}):`);
        clusterTexts.forEach((cluster, idx) => {
            const clusterCps = [...cluster].map(ch => {
                const cp = ch.codePointAt(0);
                return cp !== undefined ? `U+${cp.toString(16).toUpperCase().padStart(4, '0')}` : '?';
            });
            const w = wasm.display_width(cluster);
            lines.push(`  [${idx}] "${cluster}" width=${w} codepoints=${clusterCps.join(' ')}`);
        });

        // Word boundaries
        const wordOffsets = wasm.word_boundaries(text);
        const wordTexts: string[] = [];
        for (let i = 0; i < wordOffsets.length - 1; i++) {
            const slice = utf8.slice(wordOffsets[i], wordOffsets[i + 1]);
            wordTexts.push(utf8Decoder.decode(slice));
        }
        lines.push(``);
        lines.push(`Word segments (${wordTexts.length}): ${wordTexts.map(w => `"${w}"`).join(', ')}`);

        // Sentence boundaries
        const sentOffsets = wasm.sentence_boundaries(text);
        const sentTexts: string[] = [];
        for (let i = 0; i < sentOffsets.length - 1; i++) {
            const slice = utf8.slice(sentOffsets[i], sentOffsets[i + 1]);
            sentTexts.push(utf8Decoder.decode(slice));
        }
        lines.push(`Sentence segments (${sentTexts.length}): ${sentTexts.map(s => `"${s}"`).join(', ')}`);
    } else {
        lines.push(`(WASM not loaded -- install UniWorld WASM for full inspection)`);
    }

    const doc = await vscode.workspace.openTextDocument({
        content: lines.join('\n'),
        language: 'plaintext'
    });
    await vscode.window.showTextDocument(doc, { preview: true });
}

// ---------------------------------------------------------------------------
// Truncate to Display Width
// ---------------------------------------------------------------------------

async function truncateToDisplayWidth(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        vscode.window.showWarningMessage('UniWorld WASM not loaded; truncation unavailable.');
        return;
    }

    const widthStr = await vscode.window.showInputBox({
        prompt: 'Truncate to how many display columns?',
        placeHolder: '80',
        validateInput: (val) => {
            const n = parseInt(val, 10);
            return (isNaN(n) || n < 1) ? 'Enter a positive integer' : null;
        }
    });

    if (!widthStr) {
        return;
    }

    const maxWidth = parseInt(widthStr, 10);
    const selection = editor.selection;
    const text = editor.document.getText(selection);
    if (!text) {
        vscode.window.showWarningMessage('No text selected');
        return;
    }

    const truncated = wasm.truncate_display_width(text, maxWidth);
    const originalWidth = wasm.display_width(text);
    const newWidth = wasm.display_width(truncated);

    await editor.edit(editBuilder => {
        editBuilder.replace(selection, truncated);
    });

    vscode.window.showInformationMessage(
        `Truncated: ${originalWidth} -> ${newWidth} display columns`
    );
}

// ---------------------------------------------------------------------------
// Normalization commands (NFC, NFD, NFKC, NFKD)
// ---------------------------------------------------------------------------

async function normalizeSelection(form: string): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const selection = editor.selection;
    const text = editor.document.getText(selection);
    if (!text) {
        vscode.window.showWarningMessage('No text selected');
        return;
    }

    const wasm = loadWasm(extensionContext);
    let normalized: string;

    if (wasm) {
        // Use UniWorld WASM normalization (spec-conformant, Unicode 17.0)
        switch (form) {
            case 'NFC':  normalized = wasm.normalize_nfc(text); break;
            case 'NFD':  normalized = wasm.normalize_nfd(text); break;
            case 'NFKC': normalized = wasm.normalize_nfkc(text); break;
            case 'NFKD': normalized = wasm.normalize_nfkd(text); break;
            default:     normalized = text; break;
        }
    } else {
        // Fallback: JS built-in normalize
        normalized = text.normalize(form as 'NFC' | 'NFD' | 'NFKC' | 'NFKD');
    }

    await editor.edit(editBuilder => {
        editBuilder.replace(selection, normalized);
    });

    const src = wasm ? 'UniWorld' : 'JS built-in';
    vscode.window.showInformationMessage(
        `Normalized to ${form} via ${src} (${text.length} -> ${normalized.length} code units)`
    );
}

// ---------------------------------------------------------------------------
// Grapheme-aware cursor movement (Phase 2)
// ---------------------------------------------------------------------------

function moveCursorByGrapheme(direction: 'left' | 'right'): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        // Fallback: defer to VS Code default cursor behaviour
        const fallbackCommand = direction === 'left' ? 'cursorLeft' : 'cursorRight';
        void vscode.commands.executeCommand(fallbackCommand);
        return;
    }

    const document = editor.document;
    const newSelections: vscode.Selection[] = [];

    for (const selection of editor.selections) {
        const active = selection.active;
        const line = document.lineAt(active.line);
        const text = line.text;

        if (text.length === 0) {
            newSelections.push(new vscode.Selection(active, active));
            continue;
        }

        // Compute byte offset of cursor within the line (UTF-8)
        const prefix = text.slice(0, active.character);
        const byteOffset = utf8Encoder.encode(prefix).length;

        let newByteOffset: number;
        if (direction === 'left') {
            newByteOffset = wasm.move_left(text, byteOffset);
        } else {
            newByteOffset = wasm.move_right(text, byteOffset);
        }

        // Clamp to valid range
        const fullBytes = utf8Encoder.encode(text);
        if (newByteOffset < 0) {
            newByteOffset = 0;
        } else if (newByteOffset > fullBytes.length) {
            newByteOffset = fullBytes.length;
        }

        // Convert byte offset back to UTF-16 character index
        const newPrefix = utf8Decoder.decode(fullBytes.slice(0, newByteOffset));
        const newCharIndex = newPrefix.length;
        const newPos = new vscode.Position(active.line, newCharIndex);
        newSelections.push(new vscode.Selection(newPos, newPos));
    }

    editor.selections = newSelections;
    editor.revealRange(newSelections[0]);
}

// ---------------------------------------------------------------------------
// Hover inspector (Phase 2)
// ---------------------------------------------------------------------------

function provideUniWorldHover(
    document: vscode.TextDocument,
    position: vscode.Position
): vscode.Hover | undefined {
    const enabled = vscode.workspace.getConfiguration('uniworld')
        .get<boolean>('enableHoverInspector', true);
    if (!enabled) {
        return undefined;
    }
    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        return undefined;
    }

    const line = document.lineAt(position.line);
    const text = line.text;
    if (!text) {
        return undefined;
    }

    // Byte offset for cursor position within line
    const prefix = text.slice(0, position.character);
    const utf8 = utf8Encoder.encode(text);
    const offset = utf8Encoder.encode(prefix).length;

    const boundaries = wasm.grapheme_boundaries(text);
    if (boundaries.length < 2) {
        return undefined;
    }

    // Find the grapheme cluster containing this offset
    let clusterIndex = 0;
    for (let i = 0; i < boundaries.length - 1; i++) {
        const start = boundaries[i];
        const end = boundaries[i + 1];
        if (offset >= start && offset < end) {
            clusterIndex = i;
            break;
        }
    }

    const startByte = boundaries[clusterIndex];
    const endByte = boundaries[clusterIndex + 1];
    const clusterBytes = utf8.slice(startByte, endByte);
    const cluster = utf8Decoder.decode(clusterBytes);

    const width = wasm.display_width(cluster);
    const cps = [...cluster].map(ch => {
        const cp = ch.codePointAt(0);
        if (cp === undefined) {
            return '?';
        }
        return `U+${cp.toString(16).toUpperCase().padStart(4, '0')}`;
    });

    const md = new vscode.MarkdownString();
    md.appendMarkdown('**UniWorld Unicode inspector**\n\n');
    md.appendMarkdown(`Text: \`${cluster}\`\n\n`);
    md.appendMarkdown(`Code points: ${cps.join(', ')}\n\n`);
    md.appendMarkdown(`Display width: ${width} columns\n`);
    md.isTrusted = false;

    const hoverRange = new vscode.Range(
        new vscode.Position(position.line, 0),
        new vscode.Position(position.line, line.text.length)
    );

    return new vscode.Hover(md, hoverRange);
}

// ---------------------------------------------------------------------------
// Grapheme-aware backspace / delete (Phase 2)
// ---------------------------------------------------------------------------

async function deleteByGrapheme(direction: 'left' | 'right'): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        // Fallback to VS Code default
        const fallback = direction === 'left' ? 'deleteLeft' : 'deleteRight';
        void vscode.commands.executeCommand(fallback);
        return;
    }

    const document = editor.document;

    await editor.edit(editBuilder => {
        for (const selection of editor.selections) {
            // If there is a non-empty selection, just delete it (normal behaviour)
            if (!selection.isEmpty) {
                editBuilder.delete(selection);
                continue;
            }

            const pos = selection.active;
            const line = document.lineAt(pos.line);
            const text = line.text;

            if (direction === 'left') {
                if (pos.character === 0) {
                    // At start of line: join with previous line (default behaviour)
                    if (pos.line > 0) {
                        const prevLine = document.lineAt(pos.line - 1);
                        const range = new vscode.Range(
                            new vscode.Position(pos.line - 1, prevLine.text.length),
                            pos
                        );
                        editBuilder.delete(range);
                    }
                    continue;
                }
                // Find previous grapheme boundary
                const prefix = text.slice(0, pos.character);
                const byteOffset = utf8Encoder.encode(prefix).length;
                const prevByteOffset = wasm.move_left(text, byteOffset);
                const fullBytes = utf8Encoder.encode(text);
                const prevPrefix = utf8Decoder.decode(fullBytes.slice(0, prevByteOffset));
                const prevCharIndex = prevPrefix.length;
                const range = new vscode.Range(
                    new vscode.Position(pos.line, prevCharIndex),
                    pos
                );
                editBuilder.delete(range);
            } else {
                if (pos.character >= text.length) {
                    // At end of line: join with next line
                    if (pos.line < document.lineCount - 1) {
                        const range = new vscode.Range(
                            pos,
                            new vscode.Position(pos.line + 1, 0)
                        );
                        editBuilder.delete(range);
                    }
                    continue;
                }
                // Find next grapheme boundary
                const prefix = text.slice(0, pos.character);
                const byteOffset = utf8Encoder.encode(prefix).length;
                const nextByteOffset = wasm.move_right(text, byteOffset);
                const fullBytes = utf8Encoder.encode(text);
                const nextPrefix = utf8Decoder.decode(fullBytes.slice(0, nextByteOffset));
                const nextCharIndex = nextPrefix.length;
                const range = new vscode.Range(
                    pos,
                    new vscode.Position(pos.line, nextCharIndex)
                );
                editBuilder.delete(range);
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Visual bidi cursor movement (Phase 3)
//
// Uses an index-based visual cursor stop system from the Rust core.
// The Rust side builds a list of cursor stops in screen-left-to-right order.
// The visual_cursor_stops WASM function returns a deduplicated list of byte
// offsets ordered from screen-left to screen-right.  We simply track an
// INDEX into this list:  Right = index+1, Left = index-1.  No disambiguation,
// no hints, no back-jumps.
//
// Left key  = screen-leftward  (index - 1)
// Right key = screen-rightward (index + 1)
// ---------------------------------------------------------------------------

// Cached stop list and current index for the active line.
let cachedStops: number[] = [];    // byte offsets, left-to-right
let cachedStopChars: number[] = []; // corresponding UTF-16 char positions
let cachedStopLine = -1;
let cachedStopVersion = -1;
let currentStopIdx = 0;
let expectedChar = -1;             // char position we last set

// When we cross to a new line, we record the direction AND which line
// we are crossing TO.  The next ensureStopCache call uses this to seed
// the stop index at the correct end (0 for right-crossing, last for
// left-crossing).  The target-line check prevents stale flags from
// contaminating unrelated navigations (e.g. after the user clicks
// somewhere or presses Up/Down).
let pendingCrossDirection: 'left' | 'right' | null = null;
let pendingCrossTargetLine = -1;

/**
 * Build (or reuse) the stop list for the given line and locate the
 * cursor's current position in it.
 */
function ensureStopCache(
    wasm: any,
    doc: vscode.TextDocument,
    lineNum: number,
    charPos: number
): void {
    const text = doc.lineAt(lineNum).text;

    // Rebuild if line, version, or cursor position changed
    const cacheValid =
        lineNum === cachedStopLine &&
        doc.version === cachedStopVersion &&
        charPos === expectedChar;

    if (cacheValid) {
        // Still on the same line at the expected position -- clear any
        // stale crossing flag that might have been left behind.
        pendingCrossDirection = null;
        return;
    }

    // (Re)compute stops for this line
    const rawStops: number[] = Array.from(wasm.visual_cursor_stops(text));
    const fullBytes = utf8Encoder.encode(text);

    // Pre-compute the UTF-16 char position for each byte offset
    const chars: number[] = rawStops.map((b: number) => {
        if (b === 0) { return 0; }
        if (b >= fullBytes.length) { return text.length; }
        return utf8Decoder.decode(fullBytes.slice(0, b)).length;
    });

    cachedStops = rawStops;
    cachedStopChars = chars;
    cachedStopLine = lineNum;
    cachedStopVersion = doc.version;

    if (pendingCrossDirection !== null && lineNum === pendingCrossTargetLine) {
        // We just crossed onto this specific line -- seed the index from
        // the direction we came from, not from the character position.
        // Right-crossing: start at index 0 (visual left).
        // Left-crossing:  start at last index (visual right).
        if (pendingCrossDirection === 'right') {
            currentStopIdx = 0;
        } else {
            currentStopIdx = chars.length - 1;
        }
    } else {
        // Normal case (click, Up/Down, or stale crossing flag):
        // find the nearest stop for the current char position.
        currentStopIdx = 0;
        let bestDist = Math.abs(chars[0] - charPos);
        for (let i = 1; i < chars.length; i++) {
            const dist = Math.abs(chars[i] - charPos);
            if (dist < bestDist) {
                bestDist = dist;
                currentStopIdx = i;
            }
        }
    }

    // Always clear the pending state after consuming or discarding it
    pendingCrossDirection = null;
    pendingCrossTargetLine = -1;

    expectedChar = chars[currentStopIdx];
}

function moveCursorVisual(direction: 'left' | 'right'): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        const fallbackCmd = direction === 'left' ? 'cursorLeft' : 'cursorRight';
        void vscode.commands.executeCommand(fallbackCmd);
        return;
    }

    const doc = editor.document;
    const newSelections: vscode.Selection[] = [];

    for (const sel of editor.selections) {
        const pos = sel.active;
        const lineText = doc.lineAt(pos.line).text;

        // --- Empty line: cross to adjacent line ---
        if (lineText.length === 0) {
            if (direction === 'left' && pos.line > 0) {
                const targetLine = pos.line - 1;
                const prevText = doc.lineAt(targetLine).text;
                let targetChar = prevText.length;
                if (prevText.length > 0) {
                    const paraLevel = wasm.bidi_paragraph_level(prevText);
                    if (paraLevel % 2 === 1) {
                        targetChar = 0;
                    }
                }
                const p = new vscode.Position(targetLine, targetChar);
                newSelections.push(new vscode.Selection(p, p));
                cachedStopLine = -1;
                pendingCrossDirection = 'left';
                pendingCrossTargetLine = targetLine;
            } else if (direction === 'right' && pos.line < doc.lineCount - 1) {
                const targetLine = pos.line + 1;
                const nextText = doc.lineAt(targetLine).text;
                let targetChar = 0;
                if (nextText.length > 0) {
                    const paraLevel = wasm.bidi_paragraph_level(nextText);
                    if (paraLevel % 2 === 1) {
                        targetChar = nextText.length;
                    }
                }
                const p = new vscode.Position(targetLine, targetChar);
                newSelections.push(new vscode.Selection(p, p));
                cachedStopLine = -1;
                pendingCrossDirection = 'right';
                pendingCrossTargetLine = targetLine;
            } else {
                newSelections.push(new vscode.Selection(pos, pos));
            }
            continue;
        }

        // --- Build / reuse stop cache for this line ---
        ensureStopCache(wasm, doc, pos.line, pos.character);

        if (direction === 'right') {
            if (currentStopIdx < cachedStops.length - 1) {
                // Normal: advance one visual position to the right
                currentStopIdx++;
            } else {
                // At last stop -> cross to next line
                if (pos.line < doc.lineCount - 1) {
                    const targetLine = pos.line + 1;
                    // Determine visual-left char position for the target line.
                    // LTR line: visual left = char 0.
                    // RTL line: visual left = text.length (VS Code renders
                    //   char 0 at the right for RTL paragraphs).
                    const nextText = doc.lineAt(targetLine).text;
                    let targetChar = 0;
                    if (nextText.length > 0) {
                        const paraLevel = wasm.bidi_paragraph_level(nextText);
                        if (paraLevel % 2 === 1) {
                            targetChar = nextText.length;
                        }
                    }
                    const p = new vscode.Position(targetLine, targetChar);
                    newSelections.push(new vscode.Selection(p, p));
                    cachedStopLine = -1;
                    pendingCrossDirection = 'right';
                    pendingCrossTargetLine = targetLine;
                } else {
                    newSelections.push(new vscode.Selection(pos, pos));
                }
                continue;
            }
        } else {
            // direction === 'left'
            if (currentStopIdx > 0) {
                // Normal: advance one visual position to the left
                currentStopIdx--;
            } else {
                // At first stop -> cross to prev line
                if (pos.line > 0) {
                    const targetLine = pos.line - 1;
                    // Determine visual-right char position for the target line.
                    // LTR line: visual right = text.length.
                    // RTL line: visual right = char 0.
                    const prevText = doc.lineAt(targetLine).text;
                    let targetChar = prevText.length;
                    if (prevText.length > 0) {
                        const paraLevel = wasm.bidi_paragraph_level(prevText);
                        if (paraLevel % 2 === 1) {
                            targetChar = 0;
                        }
                    }
                    const p = new vscode.Position(targetLine, targetChar);
                    newSelections.push(new vscode.Selection(p, p));
                    cachedStopLine = -1;
                    pendingCrossDirection = 'left';
                    pendingCrossTargetLine = targetLine;
                } else {
                    newSelections.push(new vscode.Selection(pos, pos));
                }
                continue;
            }
        }

        // --- Apply the new position ---
        const newCharIdx = cachedStopChars[currentStopIdx];
        expectedChar = newCharIdx;
        const p = new vscode.Position(pos.line, newCharIdx);
        newSelections.push(new vscode.Selection(p, p));
    }

    editor.selections = newSelections;
    if (newSelections.length > 0) {
        editor.revealRange(newSelections[0]);
    }
}

// ---------------------------------------------------------------------------
// Line break opportunity decorations (Phase 3)
//
// Shows subtle markers in the editor at positions where UAX #14 line breaking
// allows a break. Includes dictionary-based breaks for Thai/Lao/Khmer/Myanmar.
// Toggled via the uniworld.showLineBreakOpportunities setting or command.
// ---------------------------------------------------------------------------

function activateLineBreakDecorations(): void {
    if (lineBreakDecorationType) {
        return; // Already active
    }
    // Allowed break: subtle grey vertical bar
    lineBreakDecorationType = vscode.window.createTextEditorDecorationType({
        after: {
            contentText: '\u00B7', // middle dot
            color: 'rgba(128,128,128,0.5)',
            fontWeight: 'normal',
            margin: '0 0 0 -0.1em'
        }
    });
    // Mandatory break: more visible red dot
    mandatoryBreakDecorationType = vscode.window.createTextEditorDecorationType({
        after: {
            contentText: '\u25CF', // black circle
            color: 'rgba(220,80,80,0.6)',
            fontWeight: 'normal',
            margin: '0 0 0 -0.1em'
        }
    });
    lineBreakActive = true;
    refreshLineBreakDecorations();
}

function deactivateLineBreakDecorations(): void {
    if (lineBreakDecorationType) {
        lineBreakDecorationType.dispose();
        lineBreakDecorationType = null;
    }
    if (mandatoryBreakDecorationType) {
        mandatoryBreakDecorationType.dispose();
        mandatoryBreakDecorationType = null;
    }
    lineBreakActive = false;
}

function toggleLineBreakDecorations(): void {
    if (lineBreakActive) {
        deactivateLineBreakDecorations();
        vscode.workspace.getConfiguration('uniworld')
            .update('showLineBreakOpportunities', false, vscode.ConfigurationTarget.Global);
        vscode.window.showInformationMessage('UniWorld: Line break decorations disabled');
    } else {
        activateLineBreakDecorations();
        vscode.workspace.getConfiguration('uniworld')
            .update('showLineBreakOpportunities', true, vscode.ConfigurationTarget.Global);
        vscode.window.showInformationMessage('UniWorld: Line break decorations enabled');
    }
}

// Simple pattern to detect URL-like spans in a line of text.
// Returns an array of [start, end] char-index pairs for each URL found.
const URL_PATTERN = /https?:\/\/[^\s<>\[\]()'"]+/gi;

function findUrlSpans(text: string): Array<[number, number]> {
    const spans: Array<[number, number]> = [];
    let match: RegExpExecArray | null;
    URL_PATTERN.lastIndex = 0;
    while ((match = URL_PATTERN.exec(text)) !== null) {
        spans.push([match.index, match.index + match[0].length]);
    }
    return spans;
}

function isInsideUrl(charIdx: number, urlSpans: Array<[number, number]>): boolean {
    for (const [start, end] of urlSpans) {
        if (charIdx > start && charIdx < end) {
            return true;
        }
    }
    return false;
}

function refreshLineBreakDecorations(): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor || !lineBreakDecorationType || !mandatoryBreakDecorationType) {
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        return;
    }

    const doc = editor.document;
    const visibleRanges = editor.visibleRanges;
    if (visibleRanges.length === 0) {
        return;
    }

    const allowedDecorations: vscode.DecorationOptions[] = [];
    const mandatoryDecorations: vscode.DecorationOptions[] = [];

    // Only process visible lines for performance
    const startLine = visibleRanges[0].start.line;
    const endLine = visibleRanges[visibleRanges.length - 1].end.line;

    for (let lineNum = startLine; lineNum <= endLine && lineNum < doc.lineCount; lineNum++) {
        const line = doc.lineAt(lineNum);
        const text = line.text;
        if (text.length === 0) {
            continue;
        }

        // Get line break opportunities from WASM
        // Returns flat array: [byte_offset, action, byte_offset, action, ...]
        // action: 0 = Mandatory, 1 = Allowed
        const raw = Array.from(wasm.line_break_opportunities(text));
        const fullBytes = utf8Encoder.encode(text);

        // Detect URL spans so we can suppress breaks inside them
        const urlSpans = findUrlSpans(text);

        for (let i = 0; i < raw.length; i += 2) {
            const byteOffset = raw[i];
            const action = raw[i + 1];

            // Skip breaks at start (byte 0) and end of line
            if (byteOffset === 0 || byteOffset >= fullBytes.length) {
                continue;
            }

            // Convert byte offset to UTF-16 char index
            const prefix = utf8Decoder.decode(fullBytes.slice(0, byteOffset));
            const charIdx = prefix.length;

            // Suppress breaks that fall inside a URL
            if (isInsideUrl(charIdx, urlSpans)) {
                continue;
            }

            const range = new vscode.Range(
                new vscode.Position(lineNum, charIdx),
                new vscode.Position(lineNum, charIdx)
            );

            if (action === 0) {
                // Mandatory break
                mandatoryDecorations.push({ range });
            } else {
                // Allowed break
                allowedDecorations.push({ range });
            }
        }
    }

    editor.setDecorations(lineBreakDecorationType, allowedDecorations);
    editor.setDecorations(mandatoryBreakDecorationType, mandatoryDecorations);
}

// ---------------------------------------------------------------------------
// Bidi visualization (Phase 3): highlight LTR vs RTL runs in different colours.
// Toggled via uniworld.showBidiVisualization. Uses WASM bidi_levels().
// ---------------------------------------------------------------------------

function activateBidiVisualization(): void {
    if (bidiLtrDecorationType || bidiRtlDecorationType) {
        return;
    }
    const opacity = vscode.workspace.getConfiguration('uniworld')
        .get<number>('bidiHighlightOpacity', 15);
    const bgAlpha = (Math.max(5, Math.min(80, opacity)) / 100).toFixed(2);
    const borderAlpha = (Math.min(0.80, parseFloat(bgAlpha) + 0.13)).toFixed(2);
    bidiLtrDecorationType = vscode.window.createTextEditorDecorationType({
        backgroundColor: `rgba(70, 150, 255, ${bgAlpha})`,
        border: `1px solid rgba(70, 150, 255, ${borderAlpha})`,
        borderRadius: '3px'
    });
    bidiRtlDecorationType = vscode.window.createTextEditorDecorationType({
        backgroundColor: `rgba(255, 160, 60, ${bgAlpha})`,
        border: `1px solid rgba(255, 160, 60, ${borderAlpha})`,
        borderRadius: '3px'
    });
    bidiVisualizationActive = true;
    refreshBidiDecorations();
}

function deactivateBidiVisualization(): void {
    if (bidiLtrDecorationType) {
        bidiLtrDecorationType.dispose();
        bidiLtrDecorationType = null;
    }
    if (bidiRtlDecorationType) {
        bidiRtlDecorationType.dispose();
        bidiRtlDecorationType = null;
    }
    bidiVisualizationActive = false;
}

/**
 * Build bidi run ranges for one line. bidi_levels returns one level per code point.
 * We group consecutive same-level runs and map to UTF-16 offsets for VS Code Range.
 * Even level = LTR, odd = RTL. Level 255 (X9-removed) is treated as LTR.
 */
function bidiRunsForLine(
    text: string,
    levels: Uint8Array
): { ltr: vscode.Range[]; rtl: vscode.Range[] } {
    const ltrRanges: vscode.Range[] = [];
    const rtlRanges: vscode.Range[] = [];
    const codePoints = [...text];
    if (codePoints.length === 0 || levels.length === 0) {
        return { ltr: ltrRanges, rtl: rtlRanges };
    }
    let u16Offset = 0;
    let runLevel = levels[0] === 255 ? 0 : levels[0];
    let runStartU16 = 0;

    for (let cpIdx = 0; cpIdx < codePoints.length; cpIdx++) {
        const rawLevel = cpIdx < levels.length ? levels[cpIdx] : 0;
        const level = rawLevel === 255 ? 0 : rawLevel;
        if (cpIdx > 0 && level !== runLevel) {
            const range = new vscode.Range(0, runStartU16, 0, u16Offset);
            if (runLevel % 2 === 0) {
                ltrRanges.push(range);
            } else {
                rtlRanges.push(range);
            }
            runStartU16 = u16Offset;
            runLevel = level;
        }
        const cp = text.codePointAt(u16Offset)!;
        u16Offset += cp >= 0x10000 ? 2 : 1;
    }

    const range = new vscode.Range(0, runStartU16, 0, u16Offset);
    if (runLevel % 2 === 0) {
        ltrRanges.push(range);
    } else {
        rtlRanges.push(range);
    }
    return { ltr: ltrRanges, rtl: rtlRanges };
}

function refreshBidiDecorations(): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor || !bidiLtrDecorationType || !bidiRtlDecorationType) {
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        return;
    }

    const doc = editor.document;
    const visibleRanges = editor.visibleRanges;
    if (visibleRanges.length === 0) {
        return;
    }

    const ltrDecorations: vscode.DecorationOptions[] = [];
    const rtlDecorations: vscode.DecorationOptions[] = [];

    const startLine = visibleRanges[0].start.line;
    const endLine = visibleRanges[visibleRanges.length - 1].end.line;

    for (let lineNum = startLine; lineNum <= endLine && lineNum < doc.lineCount; lineNum++) {
        const line = doc.lineAt(lineNum);
        const text = line.text;
        if (text.length === 0) {
            continue;
        }

        const levels = wasm.bidi_levels(text);
        const { ltr, rtl } = bidiRunsForLine(text, levels);
        for (const range of ltr) {
            ltrDecorations.push({
                range: new vscode.Range(lineNum, range.start.character, lineNum, range.end.character)
            });
        }
        for (const range of rtl) {
            rtlDecorations.push({
                range: new vscode.Range(lineNum, range.start.character, lineNum, range.end.character)
            });
        }
    }

    editor.setDecorations(bidiLtrDecorationType, ltrDecorations);
    editor.setDecorations(bidiRtlDecorationType, rtlDecorations);
}

// ---------------------------------------------------------------------------
// Grapheme-aware word selection (Phase 3)
//
// Selects the word at cursor using UniWorld word_boundaries() which respects
// script-specific word boundaries (Thai, Lao, Khmer, Myanmar, CJK).
// ---------------------------------------------------------------------------

function selectWordAtCursor(): void {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const wasm = loadWasm(extensionContext);
    if (!wasm) {
        // Fallback to VS Code default word selection
        void vscode.commands.executeCommand('editor.action.addSelectionToNextFindMatch');
        return;
    }

    const doc = editor.document;
    const newSelections: vscode.Selection[] = [];

    for (const sel of editor.selections) {
        const pos = sel.active;
        const line = doc.lineAt(pos.line);
        const text = line.text;

        if (text.length === 0) {
            newSelections.push(new vscode.Selection(pos, pos));
            continue;
        }

        // Get word boundaries with dictionary segmentation for Thai/CJK
        const wordOffsets = Array.from(wasm.word_boundaries_with_dictionary(text));
        const fullBytes = utf8Encoder.encode(text);

        // Convert cursor char position to byte offset
        const cursorPrefix = text.slice(0, pos.character);
        const cursorByte = utf8Encoder.encode(cursorPrefix).length;

        // Find the word segment containing the cursor byte position
        let wordStart = 0;
        let wordEnd = fullBytes.length;

        for (let i = 0; i < wordOffsets.length - 1; i++) {
            const segStart = wordOffsets[i];
            const segEnd = wordOffsets[i + 1];
            if (cursorByte >= segStart && cursorByte < segEnd) {
                wordStart = segStart;
                wordEnd = segEnd;
                break;
            }
        }

        // Convert byte offsets back to UTF-16 char indices
        const startPrefix = utf8Decoder.decode(fullBytes.slice(0, wordStart));
        const endPrefix = utf8Decoder.decode(fullBytes.slice(0, wordEnd));
        const startChar = startPrefix.length;
        const endChar = endPrefix.length;

        // Check if the segment is whitespace-only; if so, try to select adjacent word
        const selectedText = text.slice(startChar, endChar);
        if (selectedText.trim().length === 0 && endChar < text.length) {
            // Cursor is on whitespace; select the next word instead
            for (let i = 0; i < wordOffsets.length - 1; i++) {
                if (wordOffsets[i] === wordEnd) {
                    const nextEnd = wordOffsets[i + 1];
                    const nextEndPrefix = utf8Decoder.decode(fullBytes.slice(0, nextEnd));
                    const nextEndChar = nextEndPrefix.length;
                    const anchor = new vscode.Position(pos.line, endChar);
                    const active = new vscode.Position(pos.line, nextEndChar);
                    newSelections.push(new vscode.Selection(anchor, active));
                    break;
                }
            }
            if (newSelections.length > sel.active.line) {
                continue;
            }
        }

        const anchor = new vscode.Position(pos.line, startChar);
        const active = new vscode.Position(pos.line, endChar);
        newSelections.push(new vscode.Selection(anchor, active));
    }

    if (newSelections.length > 0) {
        editor.selections = newSelections;
        editor.revealRange(newSelections[0]);
    }
}

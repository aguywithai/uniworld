/* tslint:disable */
/* eslint-disable */

/**
 * Return the resolved embedding level for each character in the text.
 * Result is a Vec<u8> with one entry per char (indexed by char position).
 * Level 0 = LTR, odd levels = RTL.
 */
export function bidi_levels(text: string): Uint8Array;

/**
 * Return the paragraph embedding level (0 = LTR, 1 = RTL).
 */
export function bidi_paragraph_level(text: string): number;

/**
 * Return the visual reorder indices for the text.
 * reorder[visual_position] = logical_char_index.
 * Characters removed by X9 are omitted.
 */
export function bidi_reorder(text: string): Uint32Array;

export function case_fold(text: string): string;

export function display_width(text: string): number;

/**
 * Return grapheme cluster boundary byte offsets.
 */
export function grapheme_boundaries(text: string): Uint32Array;

/**
 * Return byte offsets where line breaks are allowed or mandatory.
 * Returns a flat array of pairs: [offset, action, offset, action, ...].
 * action: 0 = Mandatory, 1 = Allowed, 2 = Prohibited.
 * Only Mandatory and Allowed entries are included (Prohibited are omitted
 * for compactness).
 */
export function line_break_opportunities(text: string): Uint32Array;

export function move_left(text: string, current: number): number;

/**
 * Move cursor one grapheme cluster to the left in visual (screen) order.
 * `stop_hint` is the callers current stop index (pass 0xFFFFFFFF if unknown).
 * Returns a two-element array: [new_byte_offset, new_stop_index].
 */
export function move_left_visual(text: string, current: number, stop_hint: number): Uint32Array;

export function move_right(text: string, current: number): number;

/**
 * Move cursor one grapheme cluster to the right in visual (screen) order.
 * `stop_hint` is the callers current stop index (pass 0xFFFFFFFF if unknown).
 * Returns a two-element array: [new_byte_offset, new_stop_index].
 */
export function move_right_visual(text: string, current: number, stop_hint: number): Uint32Array;

export function normalize_nfc(text: string): string;

export function normalize_nfd(text: string): string;

export function normalize_nfkc(text: string): string;

export function normalize_nfkd(text: string): string;

/**
 * Return sentence boundary byte offsets.
 */
export function sentence_boundaries(text: string): Uint32Array;

export function to_lowercase(text: string): string;

export function to_titlecase(text: string): string;

export function to_uppercase(text: string): string;

export function truncate_display_width(text: string, max_width: number): string;

export function truncate_graphemes(text: string, max_graphemes: number): string;

/**
 * Return visual cursor stop byte offsets in screen-left-to-right order.
 * The same byte offset may appear more than once at bidi boundaries.
 * Callers should navigate by index, not by searching for byte offsets.
 */
export function visual_cursor_stops(text: string): Uint32Array;

/**
 * Return word boundary byte offsets (UAX #29 only).
 */
export function word_boundaries(text: string): Uint32Array;

/**
 * Return word boundary byte offsets with dictionary segmentation for
 * Thai/Lao/Khmer/Myanmar. Falls back to UAX #29 for other scripts.
 * For mixed text, applies dictionary segmentation to SA-script runs
 * and UAX #29 to everything else.
 */
export function word_boundaries_with_dictionary(text: string): Uint32Array;

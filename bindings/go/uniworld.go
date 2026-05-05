// Package uniworld provides Go bindings to the UniWorld Unicode library.
//
// This package uses CGo to call the UniWorld C FFI. To use it:
//
//  1. Build the UniWorld C library: cargo build --release --features cffi
//  2. Set CGO_LDFLAGS to link against the built library:
//     export CGO_LDFLAGS="-L/path/to/target/release -luniworld"
//
// All text is UTF-8 encoded.
package uniworld

/*
#cgo LDFLAGS: -luniworld
#include <stdlib.h>

// UniWorld C FFI declarations.
extern char* uniworld_normalize_nfc(const char* text);
extern char* uniworld_normalize_nfd(const char* text);
extern char* uniworld_to_lowercase(const char* text);
extern char* uniworld_to_uppercase(const char* text);
extern unsigned int uniworld_display_width(const char* text);
extern void uniworld_free_string(char* ptr);
extern unsigned long long uniworld_move_right(const char* text, unsigned long long current);
extern unsigned long long uniworld_move_left(const char* text, unsigned long long current);
*/
import "C"
import "unsafe"

// NormalizeNFC normalizes text to NFC form.
func NormalizeNFC(text string) string {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	result := C.uniworld_normalize_nfc(ctext)
	if result == nil {
		return ""
	}
	defer C.uniworld_free_string(result)
	return C.GoString(result)
}

// NormalizeNFD normalizes text to NFD form.
func NormalizeNFD(text string) string {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	result := C.uniworld_normalize_nfd(ctext)
	if result == nil {
		return ""
	}
	defer C.uniworld_free_string(result)
	return C.GoString(result)
}

// ToLowercase converts text to lowercase using full Unicode mapping.
func ToLowercase(text string) string {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	result := C.uniworld_to_lowercase(ctext)
	if result == nil {
		return ""
	}
	defer C.uniworld_free_string(result)
	return C.GoString(result)
}

// ToUppercase converts text to uppercase using full Unicode mapping.
func ToUppercase(text string) string {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	result := C.uniworld_to_uppercase(ctext)
	if result == nil {
		return ""
	}
	defer C.uniworld_free_string(result)
	return C.GoString(result)
}

// DisplayWidth computes the display width of text in column cells.
func DisplayWidth(text string) uint32 {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	return uint32(C.uniworld_display_width(ctext))
}

// MoveRight moves the cursor one grapheme cluster right.
func MoveRight(text string, current uint64) uint64 {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	return uint64(C.uniworld_move_right(ctext, C.ulonglong(current)))
}

// MoveLeft moves the cursor one grapheme cluster left.
func MoveLeft(text string, current uint64) uint64 {
	ctext := C.CString(text)
	defer C.free(unsafe.Pointer(ctext))
	return uint64(C.uniworld_move_left(ctext, C.ulonglong(current)))
}

# UniWorld PowerShell Module
# Provides Unicode text handling cmdlets backed by UniWorld (Rust FFI).
#
# Architecture:
#   UniWorld Rust core -> cdylib (uniworld.dll / libuniworld.so) -> P/Invoke -> PowerShell cmdlets
#
# Build the native library: cargo build --release --features cffi

# --- Native library loading via P/Invoke ---

$script:NativeLoaded = $false

function Initialize-UniWorldNative {
    <#
    .SYNOPSIS
        Load the UniWorld native library via Add-Type P/Invoke. Called automatically on first cmdlet use.
    #>
    if ($script:NativeLoaded) { return $true }

    # Determine platform and library name
    # $IsWindows exists in PS 7+; in Windows PS 5.1 it doesn't exist, but we're on Windows
    $onWindows = ($null -eq $IsWindows) -or $IsWindows
    $libName = if ($onWindows) {
        'uniworld.dll'
    } elseif ($IsMacOS) {
        'libuniworld.dylib'
    } else {
        'libuniworld.so'
    }

    # RID-based paths (from CI: extensions/powershell/native/win-x64/uniworld.dll etc.)
    $rid = if ($onWindows) { 'win-x64' } elseif ($IsMacOS) { 'osx-arm64' } else { 'linux-x64' }
    $candidates = @(
        (Join-Path (Join-Path (Join-Path $PSScriptRoot 'native') $rid) $libName),
        (Join-Path (Join-Path $PSScriptRoot 'native') $libName),
        (Join-Path (Join-Path (Join-Path (Join-Path $PSScriptRoot '..') '..') 'target') (Join-Path 'release' $libName))
    )

    $libPath = $null
    foreach ($path in $candidates) {
        if (Test-Path $path) {
            $libPath = (Resolve-Path $path).Path
            break
        }
    }

    if (-not $libPath) {
        Write-Warning "UniWorld native library ($libName) not found."
        Write-Warning "Build with: cargo build --release --features cffi"
        Write-Warning "Searched: $($candidates -join ', ')"
        return $false
    }

    # Define P/Invoke interop class via Add-Type
    $interopCode = @"
using System;
using System.Runtime.InteropServices;
using System.Text;

public static class UniWorldInterop
{
    private const string LIB = "$($libPath.Replace('\', '\\'))";

    // --- Memory management ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern void uniworld_free_string(IntPtr ptr);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern void uniworld_free_array(IntPtr ptr, uint len);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern void uniworld_free_u8_array(IntPtr ptr, uint len);

    // --- Normalization ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_normalize_nfc(IntPtr text);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_normalize_nfd(IntPtr text);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_normalize_nfkc(IntPtr text);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_normalize_nfkd(IntPtr text);

    // --- Case mapping ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_to_lowercase(IntPtr text);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_to_uppercase(IntPtr text);

    // --- Display width and truncation ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern uint uniworld_display_width(IntPtr text);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_truncate_display_width(IntPtr text, uint maxWidth);

    // --- Segmentation ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_grapheme_boundaries(IntPtr text, out uint outLen);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_word_boundaries(IntPtr text, out uint outLen);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_sentence_boundaries(IntPtr text, out uint outLen);

    // --- Bidi ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_bidi_levels(IntPtr text, out uint outLen);

    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern byte uniworld_bidi_paragraph_level(IntPtr text);

    // --- Line breaking ---
    [DllImport(LIB, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr uniworld_line_break_opportunities(IntPtr text, out uint outLen);

    // --- Helpers ---

    /// Encode a .NET string to a null-terminated UTF-8 IntPtr.
    /// Caller must free with Marshal.FreeHGlobal.
    public static IntPtr ToUtf8(string s)
    {
        if (s == null) return IntPtr.Zero;
        byte[] bytes = Encoding.UTF8.GetBytes(s);
        IntPtr ptr = Marshal.AllocHGlobal(bytes.Length + 1);
        Marshal.Copy(bytes, 0, ptr, bytes.Length);
        Marshal.WriteByte(ptr, bytes.Length, 0); // null terminator
        return ptr;
    }

    /// Read a UTF-8 IntPtr returned by Rust, convert to .NET string, free it.
    public static string FromUtf8AndFree(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero) return null;
        // Find null terminator
        int len = 0;
        while (Marshal.ReadByte(ptr, len) != 0) len++;
        byte[] bytes = new byte[len];
        Marshal.Copy(ptr, bytes, 0, len);
        uniworld_free_string(ptr);
        return Encoding.UTF8.GetString(bytes);
    }

    /// Read a u32 array returned by Rust into a managed uint array, then free it.
    public static uint[] ReadU32ArrayAndFree(IntPtr ptr, uint len)
    {
        if (ptr == IntPtr.Zero || len == 0) return new uint[0];
        uint[] arr = new uint[len];
        for (int i = 0; i < (int)len; i++)
        {
            arr[i] = (uint)Marshal.ReadInt32(ptr, i * 4);
        }
        uniworld_free_array(ptr, len);
        return arr;
    }

    /// Read a u8 array returned by Rust into a managed byte array, then free it.
    public static byte[] ReadU8ArrayAndFree(IntPtr ptr, uint len)
    {
        if (ptr == IntPtr.Zero || len == 0) return new byte[0];
        byte[] arr = new byte[len];
        Marshal.Copy(ptr, arr, 0, (int)len);
        uniworld_free_u8_array(ptr, len);
        return arr;
    }
}
"@

    try {
        Add-Type -TypeDefinition $interopCode -Language CSharp -ErrorAction Stop
        $script:NativeLoaded = $true
        Write-Verbose "UniWorld native library loaded: $libPath"
        return $true
    }
    catch {
        Write-Warning "Failed to load UniWorld interop: $_"
        return $false
    }
}

# --- Helper: call a Rust string->string function ---
function Invoke-UniWorldStringFunc {
    param([string]$InputText, [string]$FuncName)
    $utf8Ptr = [UniWorldInterop]::ToUtf8($InputText)
    try {
        $resultPtr = [UniWorldInterop]::$FuncName($utf8Ptr)
        return [UniWorldInterop]::FromUtf8AndFree($resultPtr)
    }
    finally {
        [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
    }
}

# --- Helper: call a Rust string->u32[] boundary function ---
function Invoke-UniWorldBoundaryFunc {
    param([string]$InputText, [string]$FuncName)
    $utf8Ptr = [UniWorldInterop]::ToUtf8($InputText)
    try {
        [uint32]$outLen = 0
        $arrPtr = [UniWorldInterop]::$FuncName($utf8Ptr, [ref]$outLen)
        $offsets = [UniWorldInterop]::ReadU32ArrayAndFree($arrPtr, $outLen)
        # Ensure the final byte offset (string end) is included as closing boundary
        $utf8Len = [uint32][System.Text.Encoding]::UTF8.GetByteCount($InputText)
        if ($offsets.Count -eq 0 -or $offsets[$offsets.Count - 1] -ne $utf8Len) {
            $offsets = @($offsets) + @($utf8Len)
        }
        return $offsets
    }
    finally {
        [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
    }
}

# --- Helper: convert byte offsets to substrings ---
function Convert-OffsetsToSegments {
    param([string]$Text, [uint32[]]$Offsets)
    $utf8Bytes = [System.Text.Encoding]::UTF8.GetBytes($Text)
    $segments = @()
    for ($i = 0; $i -lt $Offsets.Count - 1; $i++) {
        $start = [int]$Offsets[$i]
        $end = [int]$Offsets[$i + 1]
        if ($end -gt $start -and $end -le $utf8Bytes.Length) {
            $segBytes = $utf8Bytes[$start..($end - 1)]
            $segments += [System.Text.Encoding]::UTF8.GetString($segBytes)
        }
    }
    return $segments
}

# =========================================================================
# Cmdlets
# =========================================================================

function Get-GraphemeBoundaries {
    <#
    .SYNOPSIS
        Get grapheme cluster boundaries for a Unicode string.
    .DESCRIPTION
        Returns an array of grapheme cluster strings from the input text.
        Uses UniWorld's UAX #29 grapheme segmentation (Rust FFI).
    .PARAMETER InputObject
        The text to segment into grapheme clusters.
    .EXAMPLE
        "Hello" | Get-GraphemeBoundaries
    .OUTPUTS
        System.String[]
    .EXAMPLE
        Get-GraphemeBoundaries -InputObject "cafe`u{0301}"
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $offsets = Invoke-UniWorldBoundaryFunc -InputText $InputObject -FuncName 'uniworld_grapheme_boundaries'
        Convert-OffsetsToSegments -Text $InputObject -Offsets $offsets
    }
}

function Get-WordBoundaries {
    <#
    .SYNOPSIS
        Get word boundaries for a Unicode string (UAX #29).
    .DESCRIPTION
        Segments the input text into word-level units using the Unicode Text
        Segmentation algorithm (UAX #29). Returns an array of strings, including
        words and inter-word segments (spaces, punctuation). Powered by UniWorld
        Rust FFI for full Unicode conformance.
    .PARAMETER InputObject
        The text to segment into words.
    .OUTPUTS
        System.String[]
    .EXAMPLE
        "Hello World" | Get-WordBoundaries
        # Returns: "Hello", " ", "World"
    .EXAMPLE
        Get-WordBoundaries -InputObject "It's a test."
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $offsets = Invoke-UniWorldBoundaryFunc -InputText $InputObject -FuncName 'uniworld_word_boundaries'
        Convert-OffsetsToSegments -Text $InputObject -Offsets $offsets
    }
}

function Get-SentenceBoundaries {
    <#
    .SYNOPSIS
        Get sentence boundaries for a Unicode string (UAX #29).
    .DESCRIPTION
        Segments the input text into sentence-level units using the Unicode Text
        Segmentation algorithm (UAX #29). Returns an array of sentence strings.
        Handles abbreviations, terminal punctuation (.!?), and inter-sentence spacing.
        Powered by UniWorld Rust FFI for full Unicode conformance.
    .PARAMETER InputObject
        The text to segment into sentences.
    .OUTPUTS
        System.String[]
    .EXAMPLE
        "Hello. World." | Get-SentenceBoundaries
        # Returns two sentence strings
    .EXAMPLE
        Get-SentenceBoundaries -InputObject "First sentence. Second one!"
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $offsets = Invoke-UniWorldBoundaryFunc -InputText $InputObject -FuncName 'uniworld_sentence_boundaries'
        Convert-OffsetsToSegments -Text $InputObject -Offsets $offsets
    }
}

function Get-DisplayWidth {
    <#
    .SYNOPSIS
        Get the display width of a Unicode string (East Asian Width aware).
    .DESCRIPTION
        Returns the number of terminal columns the string occupies.
        CJK ideographs and fullwidth characters count as 2; most others as 1.
        Combining marks add 0 width. Powered by UniWorld Rust FFI.
    .PARAMETER InputObject
        The text to measure.
    .OUTPUTS
        System.UInt32
    .EXAMPLE
        Get-DisplayWidth "Hello"
        # Returns: 5
    .EXAMPLE
        Get-DisplayWidth "`u{4E16}`u{754C}"
        # Returns: 4 (two CJK ideographs, each width 2)
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $utf8Ptr = [UniWorldInterop]::ToUtf8($InputObject)
        try {
            [UniWorldInterop]::uniworld_display_width($utf8Ptr)
        }
        finally {
            [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
        }
    }
}

function Limit-DisplayWidth {
    <#
    .SYNOPSIS
        Truncate a string to a maximum display width without breaking grapheme clusters.
    .DESCRIPTION
        Truncates the input text so that its display width does not exceed MaxWidth
        terminal columns. Unlike simple substring, this respects grapheme cluster
        boundaries (never splits emoji, combining marks, or conjuncts) and accounts
        for double-width CJK/fullwidth characters. Powered by UniWorld Rust FFI.
    .PARAMETER InputObject
        The text to truncate.
    .PARAMETER MaxWidth
        Maximum number of display columns. CJK characters count as 2.
    .OUTPUTS
        System.String
    .EXAMPLE
        Limit-DisplayWidth -InputObject "Hello World" -MaxWidth 7
        # Returns: "Hello W"
    .EXAMPLE
        "Long text here" | Limit-DisplayWidth -MaxWidth 4
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject,
        [Parameter(Mandatory)]
        [int]$MaxWidth
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $utf8Ptr = [UniWorldInterop]::ToUtf8($InputObject)
        try {
            $resultPtr = [UniWorldInterop]::uniworld_truncate_display_width($utf8Ptr, [uint32]$MaxWidth)
            [UniWorldInterop]::FromUtf8AndFree($resultPtr)
        }
        finally {
            [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
        }
    }
}

function ConvertTo-NFC {
    <#
    .SYNOPSIS
        Normalize a Unicode string to NFC (Canonical Decomposition, followed by Canonical Composition).
    .DESCRIPTION
        Applies Unicode Normalization Form C (UAX #15) to the input text.
        NFC first decomposes characters canonically, then recomposes them. This is
        the most common normalization form and is recommended for text interchange.
        For example, "e" + combining acute (U+0301) becomes precomposed e-acute (U+00E9).
        Uses UniWorld Rust FFI; falls back to .NET normalization if the native library is unavailable.
    .PARAMETER InputObject
        The text to normalize.
    .OUTPUTS
        System.String
    .EXAMPLE
        "cafe`u{0301}" | ConvertTo-NFC
        # Returns: "cafe" with precomposed e-acute
    .EXAMPLE
        ConvertTo-NFC -InputObject "already NFC text"
        # Returns unchanged text
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) {
            # Fallback to .NET
            return $InputObject.Normalize([System.Text.NormalizationForm]::FormC)
        }
        Invoke-UniWorldStringFunc -InputText $InputObject -FuncName 'uniworld_normalize_nfc'
    }
}

function ConvertTo-NFD {
    <#
    .SYNOPSIS
        Normalize a Unicode string to NFD (Canonical Decomposition).
    .DESCRIPTION
        Applies Unicode Normalization Form D (UAX #15) to the input text.
        NFD decomposes characters canonically without recomposing. For example,
        precomposed e-acute (U+00E9) becomes "e" + combining acute (U+0301).
        Useful for canonical comparison and text analysis.
        Uses UniWorld Rust FFI; falls back to .NET normalization if the native library is unavailable.
    .PARAMETER InputObject
        The text to normalize.
    .OUTPUTS
        System.String
    .EXAMPLE
        "`u{00E9}" | ConvertTo-NFD
        # Decomposes precomposed e-acute to e + combining accent
    .EXAMPLE
        ConvertTo-NFD -InputObject "Hello"
        # ASCII-only text is unchanged
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) {
            return $InputObject.Normalize([System.Text.NormalizationForm]::FormD)
        }
        Invoke-UniWorldStringFunc -InputText $InputObject -FuncName 'uniworld_normalize_nfd'
    }
}

function ConvertTo-NFKC {
    <#
    .SYNOPSIS
        Normalize a Unicode string to NFKC (Compatibility Decomposition, followed by Canonical Composition).
    .DESCRIPTION
        Applies Unicode Normalization Form KC (UAX #15) to the input text.
        NFKC first decomposes characters by compatibility, then recomposes canonically.
        This collapses compatibility variants: fullwidth "A" (U+FF21) becomes "A",
        the fi ligature (U+FB01) becomes "fi". Useful for search and identifier comparison.
        Uses UniWorld Rust FFI; falls back to .NET normalization if the native library is unavailable.
    .PARAMETER InputObject
        The text to normalize.
    .OUTPUTS
        System.String
    .EXAMPLE
        "`u{FB01}" | ConvertTo-NFKC
        # Returns: "fi" (decomposes the ligature)
    .EXAMPLE
        "`u{FF21}" | ConvertTo-NFKC
        # Returns: "A" (fullwidth to ASCII)
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) {
            return $InputObject.Normalize([System.Text.NormalizationForm]::FormKC)
        }
        Invoke-UniWorldStringFunc -InputText $InputObject -FuncName 'uniworld_normalize_nfkc'
    }
}

function ConvertTo-NFKD {
    <#
    .SYNOPSIS
        Normalize a Unicode string to NFKD (Compatibility Decomposition).
    .DESCRIPTION
        Applies Unicode Normalization Form KD (UAX #15) to the input text.
        NFKD decomposes characters by compatibility without recomposing. This is the
        most aggressive decomposition: compatibility variants are expanded and characters
        are left in decomposed form. For example, the fi ligature (U+FB01) becomes "fi"
        and precomposed e-acute (U+00E9) becomes "e" + combining acute (U+0301).
        Uses UniWorld Rust FFI; falls back to .NET normalization if the native library is unavailable.
    .PARAMETER InputObject
        The text to normalize.
    .OUTPUTS
        System.String
    .EXAMPLE
        "`u{FB01}" | ConvertTo-NFKD
        # Returns: "fi"
    .EXAMPLE
        "`u{00E9}" | ConvertTo-NFKD
        # Returns: "e" + combining acute (2 characters)
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) {
            return $InputObject.Normalize([System.Text.NormalizationForm]::FormKD)
        }
        Invoke-UniWorldStringFunc -InputText $InputObject -FuncName 'uniworld_normalize_nfkd'
    }
}

function Get-BidiClasses {
    <#
    .SYNOPSIS
        Get the resolved bidi embedding level for each character in a Unicode string.
    .DESCRIPTION
        Returns an array of objects with Character, CodePoint, and BidiLevel.
        Level 0 = LTR, odd levels = RTL. Implements the Unicode Bidirectional
        Algorithm (UAX #9). Powered by UniWorld Rust FFI.
    .PARAMETER InputObject
        The text to analyze.
    .OUTPUTS
        PSCustomObject[] with properties: Character, CodePoint, BidiLevel, Direction
    .EXAMPLE
        Get-BidiClasses "Hello"
        # Returns 5 objects, all Direction = LTR, BidiLevel = 0
    .EXAMPLE
        Get-BidiClasses "`u{0639}`u{0631}"
        # Returns 2 objects with Direction = RTL, BidiLevel = 1
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $utf8Ptr = [UniWorldInterop]::ToUtf8($InputObject)
        try {
            [uint32]$outLen = 0
            $arrPtr = [UniWorldInterop]::uniworld_bidi_levels($utf8Ptr, [ref]$outLen)
            $levels = [UniWorldInterop]::ReadU8ArrayAndFree($arrPtr, $outLen)
        }
        finally {
            [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
        }

        $codePoints = @([System.Globalization.StringInfo]::GetTextElementEnumerator($InputObject))
        $chars = @()
        $enumerator = [System.Globalization.StringInfo]::GetTextElementEnumerator($InputObject)
        $cpIdx = 0
        while ($enumerator.MoveNext()) {
            $ch = $enumerator.GetTextElement()
            $cp = [char]::ConvertToUtf32($ch, 0)
            $level = if ($cpIdx -lt $levels.Count) { $levels[$cpIdx] } else { 0 }
            $direction = if ($level % 2 -eq 0) { 'LTR' } else { 'RTL' }
            [PSCustomObject]@{
                Character  = $ch
                CodePoint  = 'U+{0:X4}' -f $cp
                BidiLevel  = $level
                Direction  = $direction
            }
            $cpIdx++
        }
    }
}

function Get-LineBreakOpportunities {
    <#
    .SYNOPSIS
        Get line break opportunities in a Unicode string (UAX #14).
    .DESCRIPTION
        Returns an array of objects with ByteOffset and Action (Mandatory or Allowed).
        Includes dictionary-based segmentation for Thai, Lao, Khmer, Myanmar.
        Powered by UniWorld Rust FFI.
    .PARAMETER InputObject
        The text to analyze.
    .OUTPUTS
        PSCustomObject[] with properties: ByteOffset, Action (Mandatory or Allowed)
    .EXAMPLE
        Get-LineBreakOpportunities "Hello World"
        # Returns break opportunities at spaces and end-of-text
    .EXAMPLE
        "Line one.`nLine two." | Get-LineBreakOpportunities
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        if (-not (Initialize-UniWorldNative)) { return }
        $utf8Ptr = [UniWorldInterop]::ToUtf8($InputObject)
        try {
            [uint32]$outLen = 0
            $arrPtr = [UniWorldInterop]::uniworld_line_break_opportunities($utf8Ptr, [ref]$outLen)
            $raw = [UniWorldInterop]::ReadU32ArrayAndFree($arrPtr, $outLen)
        }
        finally {
            [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr)
        }

        for ($i = 0; $i -lt $raw.Count; $i += 2) {
            $action = if ($raw[$i + 1] -eq 0) { 'Mandatory' } else { 'Allowed' }
            [PSCustomObject]@{
                ByteOffset = $raw[$i]
                Action     = $action
            }
        }
    }
}

function Get-UnicodeInfo {
    <#
    .SYNOPSIS
        Get detailed Unicode information for each character in a string.
    .DESCRIPTION
        Shows codepoint, Unicode category, display width, and bidi level for each
        text element (grapheme cluster). Useful for debugging encoding issues,
        inspecting unknown characters, and verifying display properties.
        Powered by UniWorld Rust FFI for accurate display width.
    .PARAMETER InputObject
        The text to inspect.
    .OUTPUTS
        PSCustomObject[] with properties: Character, CodePoint, Category, DisplayWidth
    .EXAMPLE
        Get-UnicodeInfo "A"
        # Returns: Character=A, CodePoint=U+0041, Category=UppercaseLetter, DisplayWidth=1
    .EXAMPLE
        "Hello" | Get-UnicodeInfo | Format-Table
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory, ValueFromPipeline)]
        [string]$InputObject
    )
    process {
        $nativeOk = Initialize-UniWorldNative
        $graphemes = @()
        if ($nativeOk) {
            $graphemes = @(Get-GraphemeBoundaries -InputObject $InputObject)
        }

        $enumerator = [System.Globalization.StringInfo]::GetTextElementEnumerator($InputObject)
        $idx = 0
        while ($enumerator.MoveNext()) {
            $ch = $enumerator.GetTextElement()
            $cp = [char]::ConvertToUtf32($ch, 0)
            $category = [System.Globalization.CharUnicodeInfo]::GetUnicodeCategory($ch, 0)
            $width = if ($nativeOk) {
                $utf8Ptr = [UniWorldInterop]::ToUtf8($ch)
                try { [UniWorldInterop]::uniworld_display_width($utf8Ptr) }
                finally { [System.Runtime.InteropServices.Marshal]::FreeHGlobal($utf8Ptr) }
            } else { $ch.Length }

            [PSCustomObject]@{
                Character    = $ch
                CodePoint    = 'U+{0:X4}' -f $cp
                Category     = $category
                DisplayWidth = $width
            }
            $idx++
        }
    }
}

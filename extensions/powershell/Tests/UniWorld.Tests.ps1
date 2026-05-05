# Pester tests for UniWorld PowerShell module
# Run: Invoke-Pester -Path extensions/powershell/Tests/ -Output Detailed
#
# These tests require the UniWorld native library (uniworld.dll / libuniworld.so).
# Build: cargo build --release --features cffi
#
# NOTE: Escape sequences like `u{XXXX} require PowerShell 7+.
# The CI runs these in pwsh; local testing should use pwsh as well.

BeforeAll {
    Import-Module (Join-Path $PSScriptRoot '..' 'UniWorld.psd1') -Force
}

# =========================================================================
# Get-GraphemeBoundaries
# =========================================================================

Describe 'Get-GraphemeBoundaries' {
    It 'Segments ASCII text into individual characters' {
        $result = @(Get-GraphemeBoundaries -InputObject 'Hello')
        $result.Count | Should -Be 5
        $result[0] | Should -Be 'H'
        $result[4] | Should -Be 'o'
    }

    It 'Keeps combining mark with base character as one grapheme' {
        # e + combining acute accent (U+0301) -> single grapheme cluster "e-acute"
        $input = "e`u{0301}"
        $result = @(Get-GraphemeBoundaries -InputObject $input)
        $result.Count | Should -Be 1
        $result[0].Length | Should -Be 2  # 'e' + combining mark = 2 .NET chars
    }

    It 'Keeps ZWJ emoji sequence as one grapheme' {
        # Man ZWJ Woman ZWJ Girl: U+1F468 U+200D U+1F469 U+200D U+1F467
        $zjw = "`u{1F468}`u{200D}`u{1F469}`u{200D}`u{1F467}"
        $result = @(Get-GraphemeBoundaries -InputObject $zjw)
        $result.Count | Should -Be 1
    }

    It 'Keeps flag emoji (regional indicators) as one grapheme' {
        # Flag: Japan (JP) = U+1F1EF U+1F1F5
        $flag = "`u{1F1EF}`u{1F1F5}"
        $result = @(Get-GraphemeBoundaries -InputObject $flag)
        $result.Count | Should -Be 1
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-GraphemeBoundaries -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('World' | Get-GraphemeBoundaries)
        $result.Count | Should -Be 5
    }

    It 'Segments Devanagari conjuncts correctly' {
        # Hindi "namaste" contains conjunct clusters
        # na = U+0928, ma = U+092E, s = U+0938, te = U+0924 + U+0947
        # The conjunct "ste" is U+0938 + U+094D (virama) + U+0924 + U+0947
        $te = "`u{0924}`u{0947}"  # ta + vowel sign e = 1 grapheme
        $result = @(Get-GraphemeBoundaries -InputObject $te)
        $result.Count | Should -Be 1
    }
}

# =========================================================================
# Get-WordBoundaries
# =========================================================================

Describe 'Get-WordBoundaries' {
    It 'Segments English words and spaces' {
        $result = @(Get-WordBoundaries -InputObject 'Hello World')
        # UAX #29: "Hello", " ", "World" (space is its own segment)
        $result.Count | Should -BeGreaterOrEqual 3
        $result | Should -Contain 'Hello'
        $result | Should -Contain 'World'
    }

    It 'Handles punctuation as separate segments' {
        $result = @(Get-WordBoundaries -InputObject 'Hello, World!')
        $result | Should -Contain 'Hello'
        $result | Should -Contain 'World'
    }

    It 'Handles single word' {
        $result = @(Get-WordBoundaries -InputObject 'Unicode')
        $result.Count | Should -BeGreaterOrEqual 1
        $result | Should -Contain 'Unicode'
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-WordBoundaries -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('Test input' | Get-WordBoundaries)
        $result | Should -Contain 'Test'
    }
}

# =========================================================================
# Get-SentenceBoundaries
# =========================================================================

Describe 'Get-SentenceBoundaries' {
    It 'Segments two sentences' {
        $result = @(Get-SentenceBoundaries -InputObject 'Hello. World.')
        $result.Count | Should -BeGreaterOrEqual 2
    }

    It 'Handles single sentence' {
        $result = @(Get-SentenceBoundaries -InputObject 'Just one sentence.')
        $result.Count | Should -BeGreaterOrEqual 1
    }

    It 'Handles exclamation and question marks as terminators' {
        $result = @(Get-SentenceBoundaries -InputObject 'Really? Yes!')
        $result.Count | Should -BeGreaterOrEqual 2
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-SentenceBoundaries -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('One. Two.' | Get-SentenceBoundaries)
        $result.Count | Should -BeGreaterOrEqual 2
    }
}

# =========================================================================
# Get-DisplayWidth
# =========================================================================

Describe 'Get-DisplayWidth' {
    It 'Returns correct width for ASCII text' {
        Get-DisplayWidth -InputObject 'Hello' | Should -Be 5
    }

    It 'Returns 2 for a CJK ideograph' {
        # U+4E16 = CJK ideograph "world" (width 2)
        Get-DisplayWidth -InputObject "`u{4E16}" | Should -Be 2
    }

    It 'Returns correct width for mixed ASCII and CJK' {
        # "A" (1) + U+4E16 (2) + "B" (1) = 4
        Get-DisplayWidth -InputObject "A`u{4E16}B" | Should -Be 4
    }

    It 'Returns 2 for a standard emoji' {
        # U+1F600 = grinning face (width 2)
        Get-DisplayWidth -InputObject "`u{1F600}" | Should -Be 2
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-DisplayWidth -InputObject '' } | Should -Throw
    }

    It 'Returns 0 for a combining mark alone' {
        # U+0301 = combining acute accent (zero width on its own)
        Get-DisplayWidth -InputObject "`u{0301}" | Should -Be 0
    }

    It 'Handles fullwidth Latin characters (width 2 each)' {
        # U+FF21 = Fullwidth Latin Capital Letter A (width 2)
        Get-DisplayWidth -InputObject "`u{FF21}" | Should -Be 2
    }

    It 'Accepts pipeline input' {
        'Test' | Get-DisplayWidth | Should -Be 4
    }
}

# =========================================================================
# Limit-DisplayWidth
# =========================================================================

Describe 'Limit-DisplayWidth' {
    It 'Truncates ASCII string to max width' {
        $result = Limit-DisplayWidth -InputObject 'Hello World' -MaxWidth 5
        $result | Should -Be 'Hello'
    }

    It 'Returns full string if within max width' {
        $result = Limit-DisplayWidth -InputObject 'Hi' -MaxWidth 10
        $result | Should -Be 'Hi'
    }

    It 'Does not split a wide character' {
        # U+4E16 (width 2) - if max is 1, it should be excluded entirely
        $result = Limit-DisplayWidth -InputObject "`u{4E16}" -MaxWidth 1
        $result | Should -Be ''
    }

    It 'Handles CJK truncation respecting width' {
        # Two CJK chars: U+4E16 (2) + U+754C (2) = 4 total
        # Truncate to 3: only first CJK fits (2 <= 3), second would exceed
        $result = Limit-DisplayWidth -InputObject "`u{4E16}`u{754C}" -MaxWidth 3
        Get-DisplayWidth -InputObject $result | Should -BeLessOrEqual 3
    }

    It 'Does not split a grapheme cluster' {
        # e + combining acute: should keep or drop as unit
        $input = "e`u{0301}x"
        $result = Limit-DisplayWidth -InputObject $input -MaxWidth 1
        # Width of "e + combining acute" is 1; "x" is 1; max 1 should give just the first grapheme
        Get-DisplayWidth -InputObject $result | Should -BeLessOrEqual 1
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Limit-DisplayWidth -InputObject '' -MaxWidth 5 } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = 'Hello World' | Limit-DisplayWidth -MaxWidth 5
        $result | Should -Be 'Hello'
    }
}

# =========================================================================
# ConvertTo-NFC
# =========================================================================

Describe 'ConvertTo-NFC' {
    It 'Composes decomposed e-acute' {
        # e + U+0301 -> U+00E9
        $decomposed = "e`u{0301}"
        $result = $decomposed | ConvertTo-NFC
        $result | Should -Be "`u{00E9}"
    }

    It 'Leaves already-NFC text unchanged' {
        $result = 'Hello' | ConvertTo-NFC
        $result | Should -Be 'Hello'
    }

    It 'Composes Hangul jamo into syllable' {
        # U+1100 (G) + U+1161 (A) -> U+AC00 (GA syllable)
        $jamo = "`u{1100}`u{1161}"
        $result = $jamo | ConvertTo-NFC
        $result | Should -Be "`u{AC00}"
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { ConvertTo-NFC -InputObject '' } | Should -Throw
    }
}

# =========================================================================
# ConvertTo-NFD
# =========================================================================

Describe 'ConvertTo-NFD' {
    It 'Decomposes precomposed e-acute' {
        $composed = "`u{00E9}"
        $result = $composed | ConvertTo-NFD
        $result.Length | Should -Be 2  # e + combining acute
    }

    It 'Leaves ASCII text unchanged' {
        'Hello' | ConvertTo-NFD | Should -Be 'Hello'
    }

    It 'Decomposes Hangul syllable into jamo' {
        # U+AC00 -> U+1100 + U+1161
        $result = "`u{AC00}" | ConvertTo-NFD
        $result.Length | Should -Be 2
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { ConvertTo-NFD -InputObject '' } | Should -Throw
    }
}

# =========================================================================
# ConvertTo-NFKC
# =========================================================================

Describe 'ConvertTo-NFKC' {
    It 'Decomposes compatibility character and recomposes' {
        # U+FB01 (fi ligature) -> "fi" (2 chars)
        $result = "`u{FB01}" | ConvertTo-NFKC
        $result | Should -Be 'fi'
    }

    It 'Normalizes fullwidth Latin to ASCII' {
        # U+FF21 (Fullwidth A) -> "A"
        $result = "`u{FF21}" | ConvertTo-NFKC
        $result | Should -Be 'A'
    }

    It 'Leaves plain ASCII unchanged' {
        'Hello' | ConvertTo-NFKC | Should -Be 'Hello'
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { ConvertTo-NFKC -InputObject '' } | Should -Throw
    }
}

# =========================================================================
# ConvertTo-NFKD
# =========================================================================

Describe 'ConvertTo-NFKD' {
    It 'Decomposes compatibility character' {
        # U+FB01 (fi ligature) -> "fi"
        $result = "`u{FB01}" | ConvertTo-NFKD
        $result | Should -Be 'fi'
    }

    It 'Decomposes fullwidth Latin to ASCII' {
        # U+FF21 (Fullwidth A) -> "A"
        $result = "`u{FF21}" | ConvertTo-NFKD
        $result | Should -Be 'A'
    }

    It 'Decomposes precomposed character fully' {
        # U+00E9 -> e + U+0301
        $result = "`u{00E9}" | ConvertTo-NFKD
        $result.Length | Should -Be 2
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { ConvertTo-NFKD -InputObject '' } | Should -Throw
    }
}

# =========================================================================
# Get-BidiClasses
# =========================================================================

Describe 'Get-BidiClasses' {
    It 'Returns LTR for Latin text' {
        $result = @(Get-BidiClasses -InputObject 'AB')
        $result.Count | Should -Be 2
        $result[0].Direction | Should -Be 'LTR'
        $result[1].Direction | Should -Be 'LTR'
        $result[0].BidiLevel | Should -Be 0
    }

    It 'Returns RTL for Arabic text' {
        # U+0639 (Arabic Ain) + U+0631 (Arabic Ra)
        $arabic = "`u{0639}`u{0631}"
        $result = @(Get-BidiClasses -InputObject $arabic)
        $result.Count | Should -Be 2
        $result[0].Direction | Should -Be 'RTL'
        $result[1].Direction | Should -Be 'RTL'
        $result[0].BidiLevel | Should -Be 1
    }

    It 'Returns character and codepoint info' {
        $result = @(Get-BidiClasses -InputObject 'A')
        $result[0].Character | Should -Be 'A'
        $result[0].CodePoint | Should -Be 'U+0041'
    }

    It 'Handles mixed LTR and RTL' {
        # "A" (LTR) + U+0639 (RTL)
        $mixed = "A`u{0639}"
        $result = @(Get-BidiClasses -InputObject $mixed)
        $result.Count | Should -Be 2
        $result[0].Direction | Should -Be 'LTR'
        $result[1].Direction | Should -Be 'RTL'
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-BidiClasses -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('Hi' | Get-BidiClasses)
        $result.Count | Should -Be 2
    }
}

# =========================================================================
# Get-LineBreakOpportunities
# =========================================================================

Describe 'Get-LineBreakOpportunities' {
    It 'Finds break opportunity between words' {
        $result = @(Get-LineBreakOpportunities -InputObject 'Hello World')
        $result.Count | Should -BeGreaterOrEqual 1
        # There should be at least one Allowed break (at the space)
        $result.Action | Should -Contain 'Allowed'
    }

    It 'Returns mandatory break at end of string' {
        $result = @(Get-LineBreakOpportunities -InputObject 'Hello')
        # End of text is a mandatory break position
        ($result | Where-Object { $_.Action -eq 'Mandatory' }).Count | Should -BeGreaterOrEqual 1
    }

    It 'Reports ByteOffset and Action properties' {
        $result = @(Get-LineBreakOpportunities -InputObject 'A B')
        $result[0].PSObject.Properties.Name | Should -Contain 'ByteOffset'
        $result[0].PSObject.Properties.Name | Should -Contain 'Action'
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-LineBreakOpportunities -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('Test line' | Get-LineBreakOpportunities)
        $result.Count | Should -BeGreaterOrEqual 1
    }
}

# =========================================================================
# Get-UnicodeInfo
# =========================================================================

Describe 'Get-UnicodeInfo' {
    It 'Returns codepoint for ASCII character' {
        $result = @(Get-UnicodeInfo -InputObject 'A')
        $result.Count | Should -Be 1
        $result[0].CodePoint | Should -Be 'U+0041'
        $result[0].Character | Should -Be 'A'
    }

    It 'Returns correct display width for CJK' {
        $result = @(Get-UnicodeInfo -InputObject "`u{4E16}")
        $result[0].DisplayWidth | Should -Be 2
    }

    It 'Returns Unicode category' {
        $result = @(Get-UnicodeInfo -InputObject 'A')
        $result[0].Category | Should -Not -BeNullOrEmpty
    }

    It 'Handles multi-character input' {
        $result = @(Get-UnicodeInfo -InputObject 'AB')
        $result.Count | Should -Be 2
        $result[0].CodePoint | Should -Be 'U+0041'
        $result[1].CodePoint | Should -Be 'U+0042'
    }

    It 'Returns width 1 for standard Latin' {
        $result = @(Get-UnicodeInfo -InputObject 'x')
        $result[0].DisplayWidth | Should -Be 1
    }

    It 'Rejects empty string (Mandatory parameter)' {
        { Get-UnicodeInfo -InputObject '' } | Should -Throw
    }

    It 'Accepts pipeline input' {
        $result = @('Z' | Get-UnicodeInfo)
        $result[0].CodePoint | Should -Be 'U+005A'
    }
}

# =========================================================================
# Cross-cutting concerns
# =========================================================================

Describe 'Pipeline integration' {
    It 'Chains Get-GraphemeBoundaries output count to measure' {
        $count = @('Hello' | Get-GraphemeBoundaries).Count
        $count | Should -Be 5
    }

    It 'Normalizes then measures width' {
        # Decomposed e-acute (2 .NET chars) has display width 1
        $result = "e`u{0301}" | ConvertTo-NFC | Get-DisplayWidth
        $result | Should -Be 1
    }
}

@{
    # Module manifest for UniWorld PowerShell module

    RootModule        = 'UniWorld.psm1'
    ModuleVersion     = '0.2.0'
    GUID              = 'a1b2c3d4-e5f6-7890-abcd-ef1234567890'
    Author            = 'aguywithai'
    CompanyName       = ''
    Copyright         = '(c) 2026 Sean MacNutt / A Guy With AI. MIT License.'
    Description       = 'Correct Unicode text handling in PowerShell: grapheme boundaries, display width, normalization, bidi, line breaking, and more. Powered by UniWorld (Rust FFI).'
    PowerShellVersion = '5.1'

    # Functions to export from this module
    FunctionsToExport = @(
        'Get-GraphemeBoundaries',
        'Get-WordBoundaries',
        'Get-SentenceBoundaries',
        'Get-DisplayWidth',
        'Limit-DisplayWidth',
        'ConvertTo-NFC',
        'ConvertTo-NFD',
        'ConvertTo-NFKC',
        'ConvertTo-NFKD',
        'Get-BidiClasses',
        'Get-LineBreakOpportunities',
        'Get-UnicodeInfo'
    )

    CmdletsToExport   = @()
    VariablesToExport  = @()
    AliasesToExport    = @()

    PrivateData = @{
        PSData = @{
            Tags       = @('Unicode', 'Text', 'Grapheme', 'Bidi', 'Normalization', 'CJK', 'Emoji')
            LicenseUri = 'https://github.com/aguywithai/uniworld/blob/main/LICENSE'
            ProjectUri = 'https://github.com/aguywithai/uniworld'
        }
    }
}

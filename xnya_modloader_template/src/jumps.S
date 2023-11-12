.globl GetFileVersionInfoA
GetFileVersionInfoA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 0 * 4]
.globl GetFileVersionInfoByHandle
GetFileVersionInfoByHandle:
    jmp ds:[_ORIGINAL_FUNCTIONS + 1 * 4]
.globl GetFileVersionInfoExA
GetFileVersionInfoExA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 2 * 4]
.globl GetFileVersionInfoExW
GetFileVersionInfoExW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 3 * 4]
.globl GetFileVersionInfoSizeA
GetFileVersionInfoSizeA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 4 * 4]
.globl GetFileVersionInfoSizeExA
GetFileVersionInfoSizeExA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 5 * 4]
.globl GetFileVersionInfoSizeExW
GetFileVersionInfoSizeExW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 6 * 4]
.globl GetFileVersionInfoSizeW
GetFileVersionInfoSizeW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 7 * 4]
.globl GetFileVersionInfoW
GetFileVersionInfoW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 8 * 4]
.globl VerFindFileA
VerFindFileA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 9 * 4]
.globl VerFindFileW
VerFindFileW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 10 * 4]
.globl VerInstallFileA
VerInstallFileA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 11 * 4]
.globl VerInstallFileW
VerInstallFileW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 12 * 4]
.globl VerLanguageNameA
VerLanguageNameA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 13 * 4]
.globl VerLanguageNameW
VerLanguageNameW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 14 * 4]
.globl VerQueryValueA
VerQueryValueA:
    jmp ds:[_ORIGINAL_FUNCTIONS + 15 * 4]
.globl VerQueryValueW
VerQueryValueW:
    jmp ds:[_ORIGINAL_FUNCTIONS + 16 * 4]

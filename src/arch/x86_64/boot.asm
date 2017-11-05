global start

section .text
bits 32
start:
    mov esp, stack_top
    ; print 'OK' to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

error:
    mov dword [0xb8000], 0x4f524f45 ; 0x52=R 0x45=E
    mov dword [0xb8004], 0x4f3a4f52 ; 0x3a=colon
    mov dword [0xb8008], 0x4f204f20 ; 0x20=whitespace
    mov dword [0xb800a], al
    hlt

section .bss
stack_bottom:
    resb 64
stack_top:
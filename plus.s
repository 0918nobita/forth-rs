; [How to build]
; nasm -f macho64 plus.s
; ld -macos_version_min 10.7.0 -lSystem -no_pie -o plus plus.s

extern _printf

global start

section .data
fmt:  db  "%05X", 10, 0

section .text
start:
  push rbp      ; set up stack frame
  mov rax, 5
  mov rdi, 3
  add rax, rdi
  mov rdi, fmt  ; format for printf
  mov rsi, rax  ; first param for printf
  mov rax, 0    ; no xmm registers
  call _printf  ; call C function
  pop rbp       ; restore stack

  mov rax, 0x2000001 ; exit
  mov rdi, 0
  syscall

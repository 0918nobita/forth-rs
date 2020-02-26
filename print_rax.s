; nasm -f macho64 print_rax.s
; ld -macos_version_min 10.7.0 -lSystem -no_pie -o print_rax print_rax.o
; ./print_rax

global start

section .data
codes: db '0123456789ABCDEF'

section .text
start:
  ; 出力したい数値を格納
  mov rax, 13

  ; 1st arg of write syscall (descriptor = stdout)
  mov rdi, 1
  ; 2nd arg of write syscall (count = 1)
  mov rdx, 1

  mov cl, 8 ; bit count

.loop:
  ; 16 進数表記での 1 桁 (4 bit) ずつ処理していく
  push rax
  sub cl, 4
  ; shift arithmetic (right)
  sar rax, cl
  and rax, 0xf

  lea rbx, [rel codes]
  lea rsi, [rbx + rax]
  mov rax, 0x2000004  ; write
  push rcx
  syscall
  pop rcx

  pop rax
  test cl, cl  ; cl が 0 になっているかチェック
  jnz .loop

  mov rax, 0x2000001 ; exit
  xor rdi, rdi       ; rdi = 0
  syscall

section .data
  x times 9999 db 0
section .text
global _start
_start:
  mov rax, 55
  mov rdi, 0
  mov rsi, 3
  mov rdx, 0
  syscall

  or rax, 0x800
  mov rdi, 0
  mov rsi, 4
  mov rdx, rax
  mov rax, 55
  syscall

  mov rax, 0
  mov rdi, 0
  mov rsi, x
  mov rdx, 9999
  syscall

  mov rcx, rax
  mov rdi, x
  mov rax, 0
a:
  mov rbx, rax
b:
  inc rbx
  mov cl, [rdi+rbx]
  cmp cl, [rdi+rax]
  je next
  cmp rbx, 3
  jl b

  inc rax
  cmp rax, 3
  jl a

  sub rdi, x
  add rdi, 4

  lea rsi, [x+30]
  mov r9, rsi
  mov byte [rsi], 0
  dec rsi
  mov rax, rdi
  xor rdx, rdx
  mov r8, 10
c:
  div r8
  add rdx, 48
  mov [rsi], dl
  test rax, rax
  jz d
  xor rdx,rdx
  dec rsi
  jmp c
d:

  mov r10, rsi
  mov rax, 1
  mov rdi, 1
  mov rdx, r9
  sub rdx, r10
  syscall

  mov rax, 60
  syscall

next:
  mov rax, 0
  inc rdi
  dec rcx
  jnz a

  mov rdi, 0
  mov rax, 60
  syscall

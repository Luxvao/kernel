global gdt_init

section .text

;; FFI to access from Rust
gdt_init:
  ;; Initialisation of the function
  push rbp
  mov rbp, rsp

  ;; Disable interrupts
  cli

  ;; Setup GDT - TODO
  
  ;; Set the registers
  jmp set_registers
post_cs:
  
  ;; Exit the stack frame and function
  mov rsp, rbp
  pop rbp
  ret

;; Function for setting the necessary registers
set_registers:
  ;; Setup the usual registers
  mov ah, 0x10
  mov ds, ah
  mov ss, ah
  mov es, ah
  mov fs, ah
  mov gs, ah

  ;; Setup CS
  mov ah, 0x08
  push ah
  lea rax, [post_cs]
  push rax
  retf

section .data

gdt_start:
  ;; Null descriptor
  dq 

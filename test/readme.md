# test binaries

## mov_r32_imm32
```
hex:
  b8 12 34 56 78 b9 87 65 43 21 ba 12 34 56 78
asm:
  0x0000000000000000:  B8 12 34 56 78    mov eax, 0x78563412
  0x0000000000000005:  B9 87 65 43 21    mov ecx, 0x21436587
  0x000000000000000a:  BA 12 34 56 78    mov edx, 0x78563412
result:
  EAX = 2018915346
  ECX = 558065031
  EDX = 2018915346
  EBX = 0
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31759
```

## mov_r32_rm32
```
hex:
  b9 02 00 00 00 8b 01
asm:
  0x0000000000000000:  B9 02 00 00 00    mov ecx, 2
  0x0000000000000005:  8B 01             mov eax, dword ptr [ecx]
result:
  EAX = 0 // garbage value at [0x02]
  ECX = 2
  EDX = 0
  EBX = 0
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31751
```

## mov_rm32_r32
```
hex:
  b8 02 00 00 00 89 01 8b 01
asm:
  0x0000000000000000:  B8 02 00 00 00    mov eax, 2
  0x0000000000000005:  89 01             mov dword ptr [ecx], eax
  0x0000000000000007:  8B 01             mov eax, dword ptr [ecx]
result:
  EAX = 2
  ECX = 0
  EDX = 0
  EBX = 0
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31753
```

## short_jump
```
hex:
  bb 10 00 00 00 eb 05 bb 20 00 00 00
asm:
0x0000000000000000:  BB 10 00 00 00    mov ebx, 0x10
0x0000000000000005:  EB 05             jmp 0xc
0x0000000000000007:  BB 20 00 00 00    mov ebx, 0x20
result:
  EAX = 0
  ECX = 0
  EDX = 0
  EBX = 16
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31756
```

## near_jump
```
hex:
  bb 10 00 00 00 e9 05 00 00 00 bb 20 00 00 00
asm:
  0x0000000000000000:  BB 10 00 00 00    mov ebx, 0x10
  0x0000000000000005:  E9 05 00 00 00    jmp 0xf
  0x000000000000000a:  BB 20 00 00 00    mov ebx, 0x20
result:
  EAX = 0
  ECX = 0
  EDX = 0
  EBX = 16
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31759
```

## add_rm32_r32
```
hex:
  b8 03 00 00 00 b9 02 00 00 00 01 c8
asm:
  0x0000000000000000:  B8 03 00 00 00    mov eax, 3
  0x0000000000000005:  B9 02 00 00 00    mov ecx, 2
  0x000000000000000a:  01 C8             add eax, ecx
result:
  EAX = 5
  ECX = 2
  EDX = 0
  EBX = 0
  ESP = 31744
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 31756
```

## cmp_rm32_imm8-jle
```
hex:
  0000000 55 89 e5 c7 45 fc 08 00 00 00 83 7d fc 05 7e 06
  0000010 83 6d fc 01 eb f4 8b 45 fc 5d c3
  000001b
asm:
  0x0000000000000000:  55                      push rbp
  0x0000000000000001:  89 E5                   mov  ebp, esp
  0x0000000000000003:  C7 45 FC 08 00 00 00    mov  dword ptr [rbp - 4], 8
  0x000000000000000a:  83 7D FC 05             cmp  dword ptr [rbp - 4], 5
  0x000000000000000e:  7E 06                   jle  0x16
  0x0000000000000010:  83 6D FC 01             sub  dword ptr [rbp - 4], 1
  0x0000000000000014:  EB F4                   jmp  0xa
  0x0000000000000016:  8B 45 FC                mov  eax, dword ptr [rbp - 4]
  0x0000000000000019:  5D                      pop  rbp
  0x000000000000001a:  C3                      ret
c:
  int main() {
    int a = 8;
    while (5 < a) {
        a -= 1;
    }
    return a;
  }
result:
  EAX = 5
  ECX = 0
  EDX = 0
  EBX = 0
  ESP = 31748
  EBP = 0
  ESI = 0
  EDI = 0
  EIP = 3353708885
```

## Fibonacci numbers
https://godbolt.org/z/PIHnBZ

code
``` c
int main() {
  return fib(5);
}

int fib(int n) {
  if (n <= 1)
   return n;
  return fib(n-1) + fib(n-2);
}
```

asm
``` asm
0x0000000000000000:  55                push ebp
0x0000000000000001:  89 E5             mov  ebp, esp
0x0000000000000003:  BF 0A 00 00 00    mov  edi, 0xa
0x0000000000000008:  E8 03 00 00 00    call 0x10
0x000000000000000d:  90                nop
0x000000000000000e:  5D                pop  ebp
0x000000000000000f:  C3                ret
0x0000000000000010:  55                push ebp
0x0000000000000011:  89 E5             mov  ebp, esp
0x0000000000000013:  53                push ebx
0x0000000000000014:  83 EC 18          sub  esp, 0x18
0x0000000000000017:  89 7D EC          mov  dword ptr [ebp - 0x14], edi
0x000000000000001a:  83 7D EC 01       cmp  dword ptr [ebp - 0x14], 1
0x000000000000001e:  7F 05             jg   0x25
0x0000000000000020:  8B 45 EC          mov  eax, dword ptr [ebp - 0x14]
0x0000000000000023:  EB 1E             jmp  0x43
0x0000000000000025:  8B 45 EC          mov  eax, dword ptr [ebp - 0x14]
0x0000000000000028:  83 E8 01          sub  eax, 1
0x000000000000002b:  89 C7             mov  edi, eax
0x000000000000002d:  E8 DE FF FF FF    call 0x10
0x0000000000000032:  89 C3             mov  ebx, eax
0x0000000000000034:  8B 45 EC          mov  eax, dword ptr [ebp - 0x14]
0x0000000000000037:  83 E8 02          sub  eax, 2
0x000000000000003a:  89 C7             mov  edi, eax
0x000000000000003c:  E8 CF FF FF FF    call 0x10
0x0000000000000041:  01 D8             add  eax, ebx
0x0000000000000043:  83 C4 18          add  esp, 0x18
0x0000000000000046:  5B                pop  ebx
0x0000000000000047:  5D                pop  ebp
0x0000000000000048:  C3                ret
```

binary
```
0000000 55 89 e5 bf 0a 00 00 00 e8 03 00 00 00 90 5d c3
0000010 55 89 e5 53 83 ec 18 89 7d ec 83 7d ec 01 7f 05
0000020 8b 45 ec eb 1e 8b 45 ec 83 e8 01 89 c7 e8 de ff
0000030 ff ff 89 c3 8b 45 ec 83 e8 02 89 c7 e8 cf ff ff
0000040 ff 01 d8 83 c4 18 5b 5d c3
0000049
```

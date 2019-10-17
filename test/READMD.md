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

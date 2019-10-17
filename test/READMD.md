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

## Fibonacci numbers
https://godbolt.org/z/PIHnBZ

code
``` c
int fib(int n) {
  if (n <= 1)
   return n;
  return fib(n-1) + fib(n-2);
}

int main() {
  return fib(5);
}
```

asm
``` asm
fib(int):
 push   rbp
 mov    rbp,rsp
 push   rbx
 sub    rsp,0x18
 mov    DWORD PTR [rbp-0x14],edi
 cmp    DWORD PTR [rbp-0x14],0x1
 jg     400539 <fib(int)+0x17>
 mov    eax,DWORD PTR [rbp-0x14]
 jmp    400557 <fib(int)+0x35>
 mov    eax,DWORD PTR [rbp-0x14]
 sub    eax,0x1
 mov    edi,eax
 call   400522 <fib(int)>
 mov    ebx,eax
 mov    eax,DWORD PTR [rbp-0x14]
 sub    eax,0x2
 mov    edi,eax
 call   400522 <fib(int)>
 add    eax,ebx
 add    rsp,0x18
 pop    rbx
 pop    rbp
 ret
main:
 push   rbp
 mov    rbp,rsp
 mov    edi,0x5
 call   400522 <fib(int)>
 mov    eax,0x0
 pop    rbp
 ret
```

binary
```
55 48 89 e5 53 48 83 ec 18 89 7d ec 83 7d ec 01 7f 05 8b 45 ec eb 1e 8b 45 ec 83 e8 01 89 c7 e8 00 00 00 00 89 c3 8b 45 ec 83 e8 02 89 c7 e8 00 00 00 00 01 d8 48 83 c4 18 5b 5d c3 55 48 89 e5 bf 05 00 00 00 e8 00 00 00 00 b8 00 00 00 00 5d c3
```

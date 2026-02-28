# Registers
There are GPR(General Purpose Registers), and specific purpose registers. They being listed here:

## GPR
### 16-bit
AX, BX, CX, DX, DI, SI, BP, and SP
### 32-bit
EAX, EBX, ECX, EDX, EDI, ESI, EBP, and ESP
### 64-bit
 RAX, RBX, RCX, RDX, RDI, RSI, RBP, RSP, and R8–R15
## Specific purpose registers
- SP: Stack pointer
- SS: Stack segment
- SSP: Shadow stack pointer
- TPR: Task priority register
- TR: Task register
- eFlags: word or double word, used on compares and instructions 
- BP: Base pointer register
- CR: Control register (there are N of such)
- CS: Code segment register
- GDTR: Global descriptor table Register
- eIP: 16-bit or 32-bit instruction-pointer register
- EIP: 32-bit instruction-pointer register
- IP: 16-bit Instruction-pointer Register
- RIP: 64-bit Instruction-pointer Register
- IDPR: Interrupt Description Table Register
- LDTR: Local Descriptor Table Register
- MSR: Model Specific Register

# Details
All of the mentioned general purpose registers are obviously general purpose, even so, some of them might be conventionally used for a specific role. ESP for instance, it is the Stack Pointer register.

Worth mentioning instructions may use specific registers, for example: [MOVSQ] and variants, it uses RSI, RDI, and RCX.

In x86, the "named registers" usually have a specific role. Even while may not be enforced. EAX being an accumulator, also used for returning data, ECX a counter for loop operations, EDX for I/O, ESP stack pointer, EDI pointer to destination/data, EBP pointer to data in stack.

Some registers have a reference to a "half" of it, for example, AX being a half of the EAX register. Which is worth knowing, since if 16-bits is what you need, you can use instructions directly on these — Saving some register usage. They are all the 32-bit registers but without the prefix.

Specifically, AL being the first byte of RAX, AH the second byte of the RAX, AX both the AL and AH, EAX being the first half of RAX, composed of AH and AL along other bytes without dedicated register name.

With the caveat that when a REX prefix is present, AH, BH, CH, and DH are inaccessible, replaced by SPL, BPL, SIL, and DIL

In regards to "rN" registers, they allow you to access either a byte, or word. "rNB" for accessing a byte (e.g, r15b), "rNW" for accessing a word (e.g, r15w), "rnD" for accessing a double word (e.g, r15d).

There are many registers you cannot or shouldn't use in User Space, as they exists for the OS to handle the machine properly. They being mostly the Table registers. I will not enter in detail, as the documentation focus on user-space programming.
## Endian order
In regards to x86, it is all little-endian. 
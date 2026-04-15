> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SAHF

Copies the byte in the specified register to the AH register and then sets the EFLAGS status flags based on the value of AH. Specifically, it sets the Carry Flag (CF), Parity Flag (PF), Auxiliary Flag (AF), and Zero Flag (ZF) according to the bits of the AH register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 (implicit AH) |
| r16 | r16 (implicit AH) |
| r32 | r32 (implicit AH) |
| r64 | r64 (implicit AH) |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The instruction is available in 32-bit and 64-bit modes. In 64-bit mode, it is supported in both 64-bit and compatibility mode.

The instruction ONLY affects the CF, PF, AF, and ZF flags. All other flags in EFLAGS remain unchanged. Users MUST ensure that the source register contains the desired flag values in the lowest 8 bits, as SAHF utilizes the AH portion of the register (the high byte of the low-order word). Using a 64-bit register as a source will still only affect the AH register and the corresponding flags.
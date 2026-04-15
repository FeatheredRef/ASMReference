> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OUTSB

Outputs the byte at the address pointed to by rSI to the I/O port specified by rDX, then increments or decrements rSI by 1 based on the Direction Flag (DF).

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m8 | r8 (DX) |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit protected mode. In 64-bit mode, the rSI and rDX registers are used as 32-bit registers.

The instruction relies on the Direction Flag (DF) in the RFLAGS register; if DF=0, rSI is incremented; if DF=1, rSI is decremented. When used with the `REP` prefix, the instruction repeats until rCX is 0. If rCX is 0 initially, the instruction does not execute. Ensure that the memory region pointed to by rSI is accessible to avoid a general-protection exception (#GP).
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IDIV

Unsigned division of the dividend by the specified divisor. The dividend is implicitly stored in `RAX` and `RDX` (for 64-bit) or `EAX` and `EDX` (for 32-bit). The quotient is stored in `RAX`/`EAX` and the remainder is stored in `RDX`/`EDX`.

The following table covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | RAX, RDX |
| mN | RAX, RDX |
| imm | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, the operand size determines if the operation is 32-bit or 64-bit.

If the quotient exceeds the capacity of the destination register (RAX/EAX), a #O exception is generated. If the divisor is zero, a #Z exception is generated.

To avoid #O, the user MUST ensure that the dividend is properly scaled. For 64-bit division, `RDX` SHOULD be cleared (set to 0) before loading `RAX` if the dividend is a u64, or the dividend MUST be distributed across `RDX:RAX` such that the resulting quotient fits in r64.
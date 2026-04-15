> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB231PD

Computes a fused multiply-add or multiply-subtract operation on packed double-precision floating-point values. The operation is defined as: result = (a * b) + c or result = (a * b) - c, depending on the immediate operand. The "231" notation indicates that the destination register is the first operand, and the operation follows the order of operands 2, 3, and 1.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m64 | reg |
| reg, m64, reg | reg |
| m64, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES support for the AVX and FMA3 instruction sets. The destination register SHALL be an xmm or ymm register; if ymm registers are used, the upper bits are handled according to the VEX prefix encoding.

To avoid issues with performance and correctness:
- Mixed-width operands (combining xmm and ymm) MAY result in the upper bits of the destination register being zeroed.
- Precision and rounding are governed by the MXCSR register; failure to set the correct rounding mode MAY result in #P or #O.
- Use of this instruction on non-aligned memory operands MAY cause performance degradation or general protection faults depending on the alignment check settings.
- The operation produces a single rounding step at the end, which avoids intermediate precision loss compared to separate VMULPD and VADDPD/VSUBPD instructions.
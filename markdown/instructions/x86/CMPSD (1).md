> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSD (1)

Compares two packed single-precision floating-point values. The instruction compares the values in the destination and source operands and sets the EFLAGS.CF, EFLAGS.ZF, and EFLAGS.PF flags according to the result.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode.

The comparison follows the IEEE 754 standard for single-precision floating-point numbers. The EFLAGS.PF flag is set if either operand is a Signaling NaN (sNaN) or if the comparison is unordered. When comparing values, the instruction treats Quiet NaNs (qNaN) as unordered, which will set the EFLAGS.ZF flag to 0 and EFLAGS.PF to 1. The destination register is not modified.
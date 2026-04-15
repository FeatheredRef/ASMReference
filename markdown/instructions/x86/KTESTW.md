> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KTESTW

Performs a bitwise logical AND between a 16-bit word source and a 16-bit word destination. The result is not stored; instead, it is used to update the ZF (Zero Flag) based on whether the result is zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode.

The KTESTW instruction does not modify the destination register, unlike the standard TEST instruction which only affects flags. However, KTESTW is specifically designed for use with the `k` registers (mask registers) in AVX-512 contexts. Using general-purpose registers as operands for KTESTW is invalid; the destination MUST be a mask register (kN). Attempting to use standard GPRs will result in an encoding error.
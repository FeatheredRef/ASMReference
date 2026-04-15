> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# REPNZ

Repeats the `CMP` (Compare) instruction while the `CX`/`DX` (or `RCX`) counter is non-zero and the Zero Flag (ZF) is clear. Each iteration increments or decrements the source and destination pointers by the size of the operand and decrements the counter. The loop terminates when the counter reaches zero or ZF is set.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | mN |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand size modes. In 64-bit mode, the `RCX` register MUST be used as the counter.

The `DF` (Direction Flag) in `RFLAGS` determines the direction of pointer movement: if `DF` is 0, pointers are incremented; if `DF` is 1, pointers are decremented. If `RCX` is 0 at the start of the execution, the instruction does nothing and the `RCX` register remains 0. Failure to clear the `DF` flag before execution may result in memory access violations if the pointers are not correctly offset.
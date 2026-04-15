> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPS

Compares two operands by subtracting the source from the destination. The result is not stored, but the CPU flags (CF, ZF, SF, OF, PF) are updated based on the unsigned and signed difference. Both the source and destination pointers are incremented or decremented according to the Direction Flag (DF).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | mN |
| #I | #I |
| #I | reg |
| reg | #I |

DO NOT support LOCK

The instruction SHALL be used with the `RSI` register for the source and `RDI` register for the destination. It is available in 64-bit mode, compatibility mode, and 32-bit protected mode. The operand size is determined by the prefix (byte, word, dword, qword) or the default size of the current mode.

The behavior of `CMPS` is strictly dependent on the Direction Flag (DF). If DF is 0, the pointers `RSI` and `RDI` are incremented after the comparison; if DF is 1, they are decremented. Failure to clear (CLD) or set (STD) the Direction Flag before execution MAY lead to memory access violations or incorrect pointer offsets. The instruction MUST be used with the `REP` or `REPE/REPN` prefixes if comparing blocks of memory to avoid manually updating pointers in a loop.
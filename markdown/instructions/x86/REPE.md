> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# REPE

Repeats the `CMPS` instruction while the Zero Flag (ZF) is set and the `RCX` register is non-zero. After each iteration, `RCX` is decremented and the source and destination pointers are incremented or decremented based on the Direction Flag (DF). The operation terminates when ZF is cleared or `RCX` reaches zero.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | mN |

Support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit protected mode. The operand size is determined by the address prefix or the default operand size of the current mode.

If `RCX` is initialized to 0, the instruction does not execute any iterations and the pointers remain unchanged. To ensure the instruction processes at least one element, the programmer MUST ensure `RCX` is greater than 0. Failure to handle the `RCX = 0` case may lead to unexpected logic flow in comparison loops.
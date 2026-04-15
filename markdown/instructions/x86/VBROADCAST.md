> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VBROADCAST

Broadcasts a value from a source (register or memory) to all elements of a destination vector register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be an XMM, YMM, or ZMM register depending on the specific opcode variant. Using a register size smaller than the operand size will result in an invalid operation.

To avoid alignment faults, memory operands SHOULD be aligned to the size of the data being broadcasted. Accessing unaligned memory may result in performance degradation or general protection faults depending on the alignment check (AC) flag in the EFLAGS register.
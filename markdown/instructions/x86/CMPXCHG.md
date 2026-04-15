> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPXCHG

Compares the value in the accumulator register (AL, AX, EAX, or RAX) with the destination operand. If they are equal, the source operand is loaded into the destination operand. Otherwise, the destination operand is loaded into the accumulator.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| #I | imm |

Support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. The size of the operands MUST be consistent; if a memory operand is used, the size is determined by the register size or an operand-size override prefix.

To avoid race conditions in multi-processor environments, the LOCK prefix SHOULD be used when the destination is a memory operand (mN) to ensure atomicity of the compare-and-swap operation. Failure to use LOCK when accessing shared memory MAY lead to data corruption. The Zero Flag (ZF) is set if the comparison matches, and cleared otherwise; software MUST check ZF to determine if the exchange occurred.
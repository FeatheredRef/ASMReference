> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SUB

Subtracts the source operand from the destination operand and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mN | reg |
| reg | mN |
| imm | mN |

Support LOCK

The instruction is supported in 64-bit mode, 32-bit mode, and 16-bit mode. When operating on memory, the destination MUST NOT be a constant.

The EFLAGS register is updated based on the result: CF is set if there is a borrow-out of the most significant bit; ZF is set if the result is zero; SF is set if the result is negative; OF is set if a signed overflow occurs. Users SHOULD ensure the operand sizes match to avoid unexpected truncation or extension of values. When using `SUB` with a memory destination, the operation IS atomic if the `LOCK` prefix is applied and the memory operand is aligned.
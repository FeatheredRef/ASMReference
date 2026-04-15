> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INSW

Stores the word from the source operand into the destination word-sized memory location and increments or decrements the index register (SI/DI) based on the direction flag (DF).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | m2 |
| m2 | m2 |
| imm | #I |

DO NOT support LOCK

This instruction is available in 16-bit operand size mode. In x86-64, it is only supported when the processor is operating in compatibility mode or when the current privilege level is operating with a 16-bit operand size override.

The index registers used for the memory operands are restricted to ESI/EDI (or SI/DI in 16-bit mode). The behavior of the pointer increment/decrement is strictly tied to the state of the DF bit in the RFLAGS register. Failure to correctly initialize the direction flag may result in unintended memory corruption.
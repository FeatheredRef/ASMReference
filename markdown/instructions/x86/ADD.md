> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADD

Adds the source operand to the destination operand and stores the result in the destination.

The following table specifies the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mN | reg |
| reg | mN |
| imm | mN |

Support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode (compatibility mode). When operating on memory, the instruction MUST NOT have both operands be memory locations; one operand MUST be a register or immediate.

The ADD instruction modifies the EFLAGS register. Specifically, it updates the Carry Flag (CF), Zero Flag (ZF), Sign Flag (SF), Overflow Flag (OF), and Parity Flag (PF) based on the result of the operation.

When using the LOCK prefix with a memory destination, the processor ensures an atomic read-modify-write cycle. Failure to use LOCK in multi-threaded environments when modifying shared memory can lead to race conditions. When using immediate values with memory destinations, the operand size MUST match the size of the memory region (mN).
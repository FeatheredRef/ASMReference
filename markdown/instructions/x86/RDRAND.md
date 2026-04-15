> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDRAND

Loads a random number from the hardware random number generator into the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | reg |

DO NOT support LOCK

The instruction is available in 32-bit and 64-bit operating modes. It is not supported in 16-bit mode. The instruction is only available on processors that implement the RDRAND instruction; execution on unsupported processors results in an invalid opcode exception.

The instruction sets the Carry Flag (CF) to 1 if the hardware random number generator failed to provide a number. The software SHALL check the Carry Flag to verify that a random value was successfully returned. If CF is set, the destination register contains undefined data, and the software SHOULD retry the operation.

The hardware random number generator may have a limited entropy rate. If the instruction is executed in a tight loop without any other instructions, it may return CF=1 once the internal entropy pool is exhausted.
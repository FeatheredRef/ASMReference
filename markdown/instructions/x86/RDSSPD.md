> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDSSPD

Reads the Scalar Double-precision Floating-Point Streaming Dataset size from the specified source and stores it into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | r64 |
| m64 | r64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The instruction requires the processor to support the specific extension set associated with Streaming Datasets; attempting to execute this on a processor lacking this feature WILL result in an invalid opcode exception. Ensure the destination register is 64-bit to avoid truncation of the dataset size value.
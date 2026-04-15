> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFSD

Stores a scaled floating-point value from a register to a memory location. The instruction multiplies a floating-point value by a scale factor specified by an immediate value and stores the result in the destination.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64 / f32 | m64 / m32 |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It REQUIRES support for the AVX-512 foundation extensions.

The scale factor is encoded as an immediate value; using an unsupported immediate value SHALL result in an invalid opcode exception. Ensure that the destination memory address is properly aligned to the size of the operand to avoid performance penalties or alignment check exceptions if alignment checking is enabled.
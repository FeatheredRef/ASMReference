> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFPH

Scales a floating-point value by a specified scale factor. The instruction multiplies the source floating-point value by a scale factor derived from an immediate value, which represents a power of 2.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32/f64 | f32/f64 |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES support for the AVX-512 foundation instructions.

The scale factor is encoded as an immediate value; if the immediate is not a valid scale factor according to the ISA specification, the instruction SHALL trigger an invalid operation exception. Ensure the destination register is of the same precision (single or double) as the source to avoid unexpected type conversion or data corruption. Use of this instruction on non-AVX-512 capable hardware SHALL result in an undefined opcode exception.
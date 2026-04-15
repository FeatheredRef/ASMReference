> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPD

Compares the signed 32-bit integer elements in two zmm registers or a zmm register and a memory location according to the specified condition. The result is a mask of 1s and 0s stored in the destination register, where a 1 indicates the condition was true for the corresponding element.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m32 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512F instruction set to be supported and enabled by the processor.

The instruction uses a 3-operand format; if the destination register is the same as the first source register, it is overwritten by the mask result. The comparison is performed on signed i32 elements. Using this instruction on unsigned data will result in incorrect logical outcomes as the operation treats the most significant bit as a sign bit. Ensure the appropriate AVX-512 foundation is enabled in the CR4 register to avoid an invalid opcode exception.
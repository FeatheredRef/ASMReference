> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213SD

Performs a fused multiply-subtract operation on scalar double-precision floating-point values. It computes the result of (a * b) - c using the specified operands and stores the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the AVX-512 foundation instructions are supported. It requires the processor to be in 64-bit mode or a compatibility mode that supports the AVX-512 instruction set.

The operation is performed with a single rounding step at the end, which avoids intermediate rounding errors associated with separate multiply and subtract instructions. Ensure the destination register is not used as a source if the result must be preserved, as it is overwritten. The instruction supports masking; if a mask is used and a bit is 0, the corresponding element in the destination register remains unchanged (merging masking) or is zeroed (zeroing masking).
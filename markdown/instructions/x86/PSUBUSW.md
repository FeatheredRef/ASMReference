> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBUSW

Subtracts unsigned word integers from another set of unsigned word integers. Each 16-bit element in the source is subtracted from the corresponding 16-bit element in the destination. The result is saturated to the range of an unsigned 16-bit integer (0 to 65535); if the result is less than 0, it is saturated to 0.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set extension.

When utilizing memory operands, the memory region MUST be 16-byte aligned for optimal performance; unaligned memory accesses MAY result in performance penalties or exceptions depending on the CPU configuration and alignment check (AC) flag in EFLAGS. 

Because this instruction performs saturated subtraction, the result will NOT wrap around if the subtraction result is negative; it will instead be clamped to 0. This differs from standard integer subtraction.
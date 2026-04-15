> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPOPCNT

Counts the number of set bits (1s) for each element in the source vector and writes the result to the destination vector.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires support for the AVX-512 VPOPCNTDQ extension.

The behavior of the instruction is dependent on the selected vector length (VL) and the data type specified by the opcode (dword or qword). Using a vector register size that does not match the CPU's current supported AVX-512 feature set MAY result in an invalid opcode exception. Ensure the `zmm` registers are correctly aligned to avoid performance penalties, although the instruction SHALL function with unaligned memory operands if applicable to the specific encoding.
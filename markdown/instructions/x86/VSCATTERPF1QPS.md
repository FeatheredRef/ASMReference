> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF1QPS

VSCATTERPF1QPS scatters packed single-precision floating-point values from a ZMM register to memory locations specified by an index vector. For each single-precision value, the instruction calculates the destination address by adding the index (scaled by the size of a single-precision float) to a base address. It then stores the value at that address.

The following table describes the supported source and destination operands.

| Source | Destination(s) |
| :--- | :--- |
| zmmN | m32 |
| zmmN | zmmN |

DO NOT support LOCK

This instruction is available only when the processor supports AVX-512. It requires the AVX-512 foundation (AVX512F) and the AVX-512 Conflict Detection Instruction Set (AVX512CD) if specific conflict-avoidance logic is utilized, though the basic scatter operation is part of AVX512F. It operates in 64-bit mode.

To avoid incorrect memory operations, the user SHALL ensure that the index vector does not contain values that cause the calculated address to exceed the architectural address limit. Since this instruction performs multiple memory writes, it does not guarantee atomicity across the entire vector; each element is stored independently. If multiple indices point to the same memory location, the final value stored is determined by the order of execution, which is generally sequential relative to the element index.
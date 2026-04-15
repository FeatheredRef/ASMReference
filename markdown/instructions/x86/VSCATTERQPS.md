> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERQPS

VSCATTERQPS stores packed single-precision floating-point values from a zmm register to memory locations specified by a vector of 64-bit indices. Each element of the source vector is stored at a memory address calculated by adding the scaled index to a base address.

The following table describes the valid source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm (f32 x 16) | m64 (via zmm index) |
| zmm (i64 x 16) | #I |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is not supported in compatibility mode.

The instruction uses a mask register (k register) to determine which elements are stored. Elements with a mask bit set to 0 are not written to memory, and the corresponding index in the index vector is ignored.

Masking is essential to avoid illegal memory accesses; if an index would point to an invalid address, the mask bit MUST be 0 to prevent a general protection fault or page fault. Memory writes are not guaranteed to be atomic. To avoid data races when multiple elements in the same instruction point to the same memory address, the software MUST ensure that the indices are unique or that the operation is idempotent.
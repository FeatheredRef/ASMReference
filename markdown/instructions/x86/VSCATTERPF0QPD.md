> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF0QPD

Scatters packed 64-bit floating-point values from a ZMM register to memory locations specified by a ZMM register containing 64-bit indices, using a mask register to determine which elements are stored.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm (f64 $\times$ 8) | m64 (indexed by zmm) |
| zmm (i64 $\times$ 8) | #I |
| k register (mask) | #I |

DO NOT support LOCK

This instruction SHALL be executed only in 64-bit mode. It REQUIRES the AVX-512 foundation and the AVX-512PF (Prefetch) extension.

To avoid memory faults, the mask register MUST be used to disable any elements with invalid or out-of-bounds indices. If a masked-off element's index would cause a page fault, the fault SHALL NOT be triggered. Users MUST ensure that the memory region pointed to by the base address and the offsets provided by the index register are valid and writable to prevent general protection faults.
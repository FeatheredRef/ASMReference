> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERDPS

VSCATTERDPS scatters packed single-precision floating-point values from a zmm register to non-contiguous memory locations. It uses a set of indices stored in a second zmm register to calculate the destination addresses, where each index is scaled by the size of a single-precision floating-point value (4 bytes) and added to a base address.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | m32 |
| reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed on processors that support the AVX-512 foundation. It is not available in compatibility mode; it REQUIRES 64-bit mode.

The instruction utilizes a mask register (k register); elements in the source register are only scattered if the corresponding bit in the mask is set. If the mask bit is 0, the corresponding memory location remains unmodified. Memory accesses are performed independently for each active element, which MAY lead to multiple cache line accesses. Since the memory operations are not atomic, if multiple indices point to the same memory location, the final value stored is indeterminate.
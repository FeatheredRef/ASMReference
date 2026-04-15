> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WBNOINVD

Writes back modified data in the cache lines containing the specified linear address to main memory and invalidates the cache line without signaling other processors in the system.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | m64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It SHALL NOT be used in compatibility mode.

The instruction does not trigger the cache coherency protocol. Therefore, it SHALL NOT be used in multi-processor environments if other processors may have cached the same memory region, as this will result in cache incoherency. Use `WBINVD` if cache coherency across all processors is required.
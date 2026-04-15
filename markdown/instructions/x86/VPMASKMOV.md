> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMASKMOV

Moves a mask of bytes from a source to a destination based on a mask register. For each byte in the source, the corresponding bit in the mask register determines whether the byte is copied to the destination; if the bit is 0, the destination byte remains unchanged.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | m16 |
| m16 | xmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It REQUIRES the AVX instruction set to be enabled.

To avoid undefined behavior or general protection faults, the memory operand m16 MUST be aligned to a 16-byte boundary. Failure to ensure alignment SHALL result in an alignment check exception if alignment checking is enabled.
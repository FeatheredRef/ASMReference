> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SGDT

Stores the contents of the GDTR (Global Descriptor Table Register) into the specified destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal GDTR | m6 |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, the GDTR contains a 16-byte value consisting of a 16-bit limit and a 64-bit base address.

The destination operand MUST be a memory region (m6) capable of holding 6 bytes in 32-bit/16-bit mode or 10 bytes in 64-bit mode. Writing to an insufficiently sized memory region SHALL result in a buffer overflow of adjacent memory.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA256MSG1

SHA256MSG1 performs a SHA-256 message schedule update. It computes the sum of the specified source register, a rotated version of another register, and a third register, then stores the result in the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

The instruction is only available when the SHA extensions are supported by the processor. It operates exclusively on XMM registers.

The instruction requires the use of 128-bit XMM registers; attempting to use other register types SHALL result in an invalid operand. Ensure that the CPUID.0x00000001:ECX.bit[26] (SHA) flag is set before execution to avoid an illegal instruction exception.
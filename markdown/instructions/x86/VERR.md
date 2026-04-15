> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VERR

Reads a byte from the specified memory location and verifies that the read operation does not cause a fault or exception. If the read is successful, the ZF flag is cleared; if the read causes a fault or exception, the ZF flag is set and the exception is suppressed.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | None |

DO NOT support LOCK

VERR is not supported in 64-bit mode. It is only available in compatibility mode.

The instruction modifies the ZF flag based on the success of the memory access. It does not modify any general-purpose registers.

To avoid unexpected behavior in 64-bit native code, ensure that VERR is not used, as it will trigger an invalid opcode exception (#UD). When using VERR in compatibility mode, be aware that it only checks for read permissions and does not guarantee that subsequent write operations to the same address will succeed.
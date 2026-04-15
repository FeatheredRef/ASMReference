> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPA

Pops the general-purpose registers from the stack.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | reg |

DO NOT support LOCK

This instruction is only available in 32-bit mode and compatibility mode. It is NOT supported in 64-bit mode.

The stack pointer (ESP) is incremented by 32 bytes after the operation. Using this instruction in 64-bit mode will result in an invalid opcode exception. Ensure the stack contains enough valid data to avoid reading from unmapped memory regions, which could trigger a page fault.
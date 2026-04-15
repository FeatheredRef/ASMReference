> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INT1

Generates a breakpoint exception (INT 1), which triggers a software interrupt.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. It does not have architectural constraints regarding register usage as it takes no operands.

The INT 1 instruction is specifically used for hardware breakpoints and debugging. Users MUST be aware that if a debug exception handler is not properly configured in the IDT, this instruction WILL trigger a double fault. Additionally, while it behaves similarly to INT 3, INT 1 is typically used by the processor to signal a debug breakpoint; using it manually in code can interfere with hardware debugging tools.
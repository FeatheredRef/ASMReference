> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# NOP

The instruction does not affect the state of the processor, except for incrementing the instruction pointer.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The NOP instruction is available in all operating modes, including 64-bit mode, compatibility mode, and 32-bit protected mode.

The NOP instruction is encoded as `0x90`, which is technically a one-byte alias for `XCHG EAX, EAX`. Because it is an alias for an exchange operation on the same register, it produces no architectural side effects and does not trigger any exceptions. Use this instruction for padding or timing delays.
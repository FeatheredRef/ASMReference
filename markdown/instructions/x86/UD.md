> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UD

The instruction triggers an invalid opcode exception.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is intended to be used for padding or to explicitly mark undefined instructions. Execution of this opcode SHALL result in a `#UD` exception.

Since this instruction is designed to fail, any attempt to execute it will result in an immediate transfer of control to the exception handler. It does not modify any architectural state other than the instruction pointer (RIP) and the stack during the exception transition.
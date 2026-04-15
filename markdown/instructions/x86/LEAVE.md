> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LEAVE

The `LEAVE` instruction restores the stack frame by setting the stack pointer (`RSP`) to the value contained in the frame pointer (`RBP`) and then popping the value at the top of the stack into `RBP`.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 (via RSP) | r64 (RBP), r64 (RSP) |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit protected mode. In 64-bit mode, the operation utilizes r64 registers.

To avoid stack corruption or segmentation faults, the `RBP` register MUST be correctly initialized via a preceding `ENTER` instruction or a `PUSH RBP` / `MOV RBP, RSP` sequence. If `RBP` does not point to a valid saved frame pointer on the stack, the `POP RBP` phase of the instruction will load an arbitrary value into the frame pointer and leave the stack pointer in an inconsistent state.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VERW

Writes a dword to the destination operand and reads a dword from the source operand. In 64-bit mode, this instruction is frequently used to execute a serialization operation to flush cached memory translation entries, specifically to clear the write-combining buffers.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | m32 |
| reg | #I |
| imm | #I |

DO NOT support LOCK

The instruction MUST be used with memory operands. Using registers or immediates SHALL result in an invalid operation.

In 64-bit mode, VERW is specifically utilized as a mechanism to flush internal processor buffers. When the destination operand is a memory location, the processor ensures that previous stores are completed before the write occurs. This is critical for avoiding data corruption or synchronization issues when interacting with memory-mapped I/O or during specific power-management transitions.

The operand size MUST be dword. Using different operand sizes SHALL lead to unpredictable behavior or failure to trigger the intended buffer flush. To avoid issues when attempting to clear write-combining buffers, ensure the memory operand points to a valid, writable memory location.
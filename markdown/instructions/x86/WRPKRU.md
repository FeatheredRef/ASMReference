> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRPKRU

Writes the value specified in the source operand to the Protection Key Register 0 (PKRU).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (r32) | Internal PKRU register |

DO NOT support LOCK

The instruction is only available if the processor supports Protection Keys and the `CR4.PKE` bit is set. If `CR4.PKE` is cleared, executing this instruction SHALL result in an invalid opcode exception (#UD). This instruction is available in both 64-bit mode and compatibility mode.

The source register MUST contain the desired PKRU value; any bits beyond the 32-bit register size are ignored. Because WRPKRU modifies the access rights for the current thread, the effects are local to the logical processor executing the instruction. Failure to properly synchronize PKRU state during context switches in an OS kernel MAY lead to memory access violations or security vulnerabilities.
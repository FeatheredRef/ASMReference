> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PTWRITE

Writes a 64-bit value to a page-table entry specified by a linear address, ensuring the write is performed as an atomic operation if the memory is aligned.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m8 |
| imm | m8 |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode. Execution in compatibility mode or 32-bit mode SHALL result in an invalid opcode exception.

To avoid unexpected behavior and potential memory corruption, the destination address MUST be aligned on a qword boundary. If the address is not aligned, the atomicity of the update to the page-table entry is NOT guaranteed, which MAY lead to inconsistent paging structures in multiprocessor environments.
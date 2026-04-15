> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSCATTERDQ

This instruction stores 64-bit quadword elements from a zmm register into a memory region at addresses specified by an index vector. Each index is scaled by a constant specified in the immediate operand and added to a base address.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm | m64 |
| zmm | zmm |

DO NOT support LOCK

This instruction requires the AVX-512 foundation and the AVX-512DQ extension. It SHALL only be executed in 64-bit mode. If the instruction is executed in compatibility mode, it SHALL trigger an invalid opcode exception.

To avoid undefined behavior, the programmer MUST ensure that the destination memory region is properly aligned if the memory access crosses a page boundary. Since this instruction performs scatter operations, it is susceptible to memory aliasing; if multiple indices in the index vector point to the same memory location, the final value stored SHALL be the one from the element with the highest index. Masks SHALL be used to prevent illegal memory accesses for inactive elements.
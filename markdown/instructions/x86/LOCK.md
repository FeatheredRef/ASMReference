> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LOCK

The LOCK prefix asserts a hardware signal that ensures the processor has exclusive access to a shared memory location for the duration of the instruction. It prevents other processors or bus masters from modifying the targeted memory region until the instruction completes.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m8, m16, m32, m64 |
| imm | m8, m16, m32, m64 |
| m8, m16, m32, m64 | m8, m16, m32, m64 |

Support LOCK

The LOCK prefix SHALL only be used with instructions that perform a read-modify-write operation on memory. Use of the LOCK prefix with instructions that do not support it SHALL result in an invalid opcode exception. In 64-bit mode, memory operands MUST be aligned to their size to avoid potential performance degradation or faults, although the LOCK prefix is specifically required for atomic operations across multiple cores.

To avoid memory ordering issues, the programmer SHOULD be aware that LOCK instructions act as a memory fence, preventing the reordering of reads and writes across the instruction boundary. When using LOCK with unaligned memory operands, the operation MAY result in a locked bus cycle (bus lock) rather than a cache lock, which significantly degrades system performance.
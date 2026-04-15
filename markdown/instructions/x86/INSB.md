> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INSB

Reads a byte from the location pointed to by the `ESI` or `RSI` register (or `EDI`/`RDI` if the direction flag DF is set), stores it into the `AL` or `R8B` register, and then increments or decrements the index register by 1.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m1 | r8 |

DO NOT support LOCK

The instruction is only available when the processor is in 32-bit mode or 64-bit mode. In 64-bit mode, the index register used is `RSI` or `RDI` unless a 32-bit operand size prefix is used, in which case `ESI` or `EDI` is utilized.

The behavior of the index register modification is dependent on the Direction Flag (DF) in the `RFLAGS` register: if DF=0, the index register is incremented; if DF=1, the index register is decremented. Failure to correctly initialize the DF flag before execution MAY result in memory access violations or incorrect data retrieval.
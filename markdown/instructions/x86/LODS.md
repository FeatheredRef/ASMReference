> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LODS

Loads a value from the memory location pointed to by the address in the RSI (or RDI/RSI depending on the specific variant) register into the accumulator register (AL, AX, EAX, or RAX), then increments or decrements the source index register based on the Direction Flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m1 | r8 |
| m2 | r16 |
| m4 | r32 |
| m8 | r64 |

DO NOT support LOCK

In x86-64 mode, the instruction MUST use the RSI register as the source pointer. The behavior of the pointer update is governed by the DF bit in the RFLAGS register: if DF=0, the index is incremented; if DF=1, the index is decremented.

Users MUST ensure that the DF bit is explicitly set using `CLD` or `STD` before execution to avoid unpredictable pointer movement. Because this instruction relies on the implicit use of RSI and the accumulator, it cannot be used with arbitrary registers. The operand size is determined by the address size override or the specific mnemonic variant (LODSB, LODSW, LODSD, LODSQ).
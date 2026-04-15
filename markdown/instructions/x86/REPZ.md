> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# REPZ

Repeats the `CMPS` or `SCAS` instruction while the Zero Flag (ZF) is set to 1 and the `RCX` register is greater than 0. In each iteration, the instruction compares the operands, decrements `RCX`, and increments or decrements the index registers (`RSI` and `RDI`) based on the Direction Flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | mN |
| reg | mN |

Support LOCK

The `REPZ` prefix is only functional when used with `CMPS` or `SCAS`. The behavior depends on the current operating mode; in 64-bit mode, the memory operands are typically accessed as qwords unless a size override prefix is used.

To avoid unintended behavior, ensure `RCX` is initialized to the correct count before execution, as a value of 0 will result in the instruction being skipped entirely. Users SHALL be aware that if the Direction Flag (DF) is not cleared (via `CLD`) or set (via `STD`) explicitly, the pointer increments/decrements will be non-deterministic based on the previous state of the processor.
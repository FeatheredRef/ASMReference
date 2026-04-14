# x86-64 ISA Documentation generation

Author: ttd3v
Date: 13-4-2026

## Abstract

How a model shall generate documentation for the x86-64 ISA

## Introduction

This text covers how an LLM shall generate documentation about the x86-64 ISA, as reliable as possible. Trimming away redundant text, thus avoiding "slop". 

Still, acknowledging that no AI model can truly be 100% reliable, as those models are statistical, and therefore cannot self-check as a reasonable and sane human would.

The intent of generating these texts instead of writing from scratch, is because reading to validate data can be much faster than writing from scratch. One of the reasons being that AI rarely (arguably never) makes grammar and structural errors.

## Requirements

Before any prompt is inserted, the model shall have its context usage as short as possible. To avoid problems regarding the past context possibly decreasing the quality of the output. The model must be reasoning-capable.

## Methodology

You may generate text at any rate, all generated text have to be validated by a human peer. The AI generated version may be displayed without human validation, but shall have a warning mentioning the unverified validity of the content. And preferably, a link to the source of truth in the warning.

This warning must be removed from the text, once a human verifies the data. 

## Prompt

```
Write a text about <instruction> from x86-64 ISA, it HAVE to be written in markdown, HAVE to start ONLY with the instruction mnemonic name in uppercase and must be a markdown header (# <instruction>). 

The first paragraph must ONLY describe the behavior of the instruction.

Then, a description saying that the table after the description covers what the source and destinations can be. Which SHALL HAVE headers for *source*, and *destination*(s). All the rows displaying the supported imm/register/memory.

After the table, "Support LOCK" if it does, "DO NOT support LOCK" if doesn't support lock.

Then, you shall write texts regarding the instruction's architectural constraints and side effects that do directly affect the behavior of such. For example, its support (whether only available on compatibility mode).

After the mentioned writings, you shall write about details that are coherent to be put. The coherency being defined by whether the detail would help a person avoid an issue.

Regarding your tone and style, do not have a tone, and be concise.

NOTATION:
Throughout the generated text, you SHALL use the specified notation.
- imm: Immediate
- reg: Register
- rN: A register. Where N is the bit count of the register size.
- mN: A memory region, where N is the byte count of it.
- fN: A floating point number, where N is the bit count of the number size.
- uN: An unsigned number, where N is the bit count of the number size
- iN: A signed number, where N is the bit count of the number size
- word: 2 bytes
- dword/double word: 4 bytes
- qword/quad word: 8 bytes
- #I: Invalid operation.
- #Z: Divide-by-zero.
- #D: Denormalized operand.
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result (precision).

Besides the explicitly mentioned, use Intel SDM Notations.

CONSTRAINTS:
- Use RFC 2119 terminology
- Use Intel SDM terminology

Return ONLY the markdown content. Do not include any conversational introductions or conclusions.
```

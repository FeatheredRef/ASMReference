# Quality Control 
> Version: 0
## Introduction
This article covers the quality control standard of this project texts, from how to measure the quality of a text to what may or may not be suitable for publication.

## Terminology
Follows *RFC 2119*; The project has references for programming, thus "software implementations" refers to reader's own projects.

"Factual"/"Fact" in the context of this quality control standard means parity with the actual truth of what is being documented, if the text is not a documentation, then the word means parity with the truth defined within the text.

# Versioning
This document might change, you can use the "Version" line below the title to be certain of which version you're reading.

# Text Quality Measures
A text can fit into four categories: "Undefined", "Unsatisfactory", "Tolerable", and "Satisfactory". These being selected to frame a text as, based on the end result of points.

Starting as zero, a text earn one point by complying to each item:
- Is compliant to formal grammar
- Is factually correct
- Is factually accurate
- Is concise
- Have at least proper headings markdown formatting (title as "#" and subtitles as "##", "###", et cetera)
- Is compliant to the topic of the text
- All claims are accurate
- Cite repertory (markdown links, or explicit URLs), if describing it in text would be out of topic.
- Define terminology, or if don't, that the absence of it does not interfere negatively in the interpretation.

The text loses a point by complying to each item:
- Have ambiguous definitions
- Have ambiguous claims
- Have ambiguous declarations
- Have ambiguous sentences
- Have spelling errors
- Have factual inaccuracies that cannot cause issues in software implementations
- Have inaccurate claims that can cause volatile interpretation, and cannot raise any issue in software implementations.
- Have unused terminology that harms readability in any way.

The text cannot earn any point (only lose) points by complying to any of the following items:
- Having religious claims
- Having hate speech, or any instance that could be directly interpreted as such.
- Moving off topic, when a reference to another text could be done.
- Be factually incorrect.
- Cite incorrect, incoherent, or non-existent repertory.
- Have factual inaccuracies that can cause issues in software implementations
- Have inaccurate claims that can cause volatile interpretation, and can raise any issue in software implementations.
- Have more than five distinct spelling errors.
- Have more than three ambiguous definitions
- Have more than three ambiguous claims
- Have more than three ambiguous declarations
- Have more than three ambiguous sentences

## Classification
There's a maximum of 9 points (there are nine items that can yield points), a text is classified by the row category by being in the percentage range. Which percentage is calculated by the division `(points gained-lost)/maximum points`.

| Percentage range | Category |
|:-----------------|:---------|
| -100% - 44%      | Unsatisfactory |
| 45% - 74%        | Tolerable |
| 75% - 100%       | Satisfactory |

Whereas "Undefined" is for texts not yet reviewed

## Reviewer
A reviewer shall always be a human being different from the author of the text; If there is no human available, an AI model capable of reasoning can be used.

If the text written has an AI as author, an AI model cannot review it. Thus, the "Undefined" category SHOULD be applied unless an explicit warning before the first header bluntly warn about the potential unreliability of the content — "Tolerable" applied in this case.

# Publication Process

The publication process to ensure the quality of all texts prior to release, which SHALL verify if all the texts are "Tolerable" or "Satisfactory"; Trimming away texts on other categories.



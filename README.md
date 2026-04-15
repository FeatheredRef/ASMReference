# ASM Reference

This project is supposed to make available a compilation of specific data in regards to general low-level data, making it accessible for people demanding it, or for newbies to learn with less friction. Also, so people don't have to learn totally from AI.

## Contributions

Feel free to write for this project, you may write an article at any time, or a specific topic if missing. 
You may either make a pull request, or send me an email.

## License

This project is licensed under [GNU General Public License v2.0](LICENSE-GPL2.md)

---


# Terminology
The rest of this README follows the *RFC 2119*. The word SIGNIFICANT when, and only when, in all capital letters means something that impacts both the interpretation, and perception of an instance.

# Authorship
All texts have an authorship, as defined in `details.json`. A person may contribute to another's text. If it happens, if the contribution is SIGNIFICANT, the authorship of the contributor SHALL be added to the text.

No one can contribute to people's articles, nor can these be edited. The authorship of an article MUST be defined only once.

> `details.json` is a file that keeps track of categories, authors, texts, and some render related instances.

# Publication process

For a person to publish a text into the project or contribute to an existing text, they SHOULD send a request draft. It being done by either sending an email to featheredref@protonmail.com, or creating an issue with the label "PUBLISHING" or "CONTRIBUTING".

The draft request MUST be a message which SHALL not be blank, and a linked markdown file for the draft. In the message, the sender SHALL describe in what category the sender wants to publish on. 

If the project does not have the sender registered as an author, the sender SHOULD say which valid social link SHALL be linked to him, along the username, name (don't have to be the full name), and an image URL for it to be a profile picture. In case none is provided, the person verifying the message SHALL infer; No authorship if no inference can be done reasonably.

In the event that the sender sends an incoherent category, the person verifying the request will change it.

Once the request is sent, a collaborator will review it. If the draft has factually problematic text, and/or grammar issues, these SHALL be reported to the sender, who must edit and reply with another version.

If the draft state doesn't have grammar issues, and factually problematic text, the text will be inserted into the project. It being only publicly available on a web reference, after a release.

Upon publication, if the request is a GitHub issue, it might be closed.

> A markdown "h1" ("#") is mandatory on the start of a text/draft, as the renderer uses it as a reference.

# Releases

If any change happens, a new release of the web version SHALL be released in the period between a day, and three months. Generally a delay (more than a day to release) may occur if there is too many on-going requests, and the collaborators think that delaying a release is worth doing.

A delay may also happen if changes are being done into the website's renderer.

Releases are supposed to have factual data only, thus one will only be made once a certainty that the new versions of texts are accurate. Even so, it's not impossible for a lack of precision or blunt error to pass unnoticed. In these cases, if you notice some, please raise an issue or send an email to featheredref@protonmail.com in order to report the problem.

# Writing format

Even while no writing format is ensured, nor enforced, its good to recommend a standard for writers to follow. It being, in the given order:

- **Abstract**: Start with a brief explanation of the text's contents, so a reader can know what they are reading about. Since this project is a reference, these abstracts may improve searchability/queries.
- **Introduction**: It helps the reader to ground their view, using it as a definition of direction and subject may be useful.
- **Terminology**: If you have a writing bias, or have non-standard wording and/or expressions, defining their meaning in this section might be good for clarity.
- **Actual content**: At this "place" you write what you intend to.

# How text quality is verified
They are tested against the [Quality Control Standard](</markdown/article/quality-control.md>), published only if "Tolerable" or "Satisfactory".

> If the reasons your text lose points is minor spelling errors, the reviewer may fix those for you, therefore maybe increasing your points.

# Use of Generative AI
Preferably, don't use it. If you do, as long as the quality is decent, it may be merged. Be aware that it is seen peevishly. Even so, some topics may benefit from the use of it. And the structure which the AI should follow must be released as an specification and previously drafted and published as an article.

When a model generates an article, the authorship does not go to the person who made the prompt. This being to avoid rewarding people that generate out of stupidity/laziness, instead of reasonability.

# Categories
- **arch**: Directly related with hardware
- **linux**: Directly related with the Linux, specifically the interaction user-space with kernel, or specifically kernel internals.
- **instructions**: Assembly and Instructions of CPUs of an arch, if you pick this category, you must state the architecture (x86, ARM, Risc-V, et cetera).
- **network**: Specifically what is directly related with network protocols, and/or systems.
- **kernel**: Regarding kernels, kernel development.
- **disk**: Regarding disk (HDDs, SSDs, et cetera), and file systems.
- **encryption**: Regarding encryption
- **hashes**: Regarding hashes
- **algorithms**: Regarding algorithms
- **C**: Regarding C
- **compilers**: Regarding compilers
- **VM**: Regarding virtual machines
- **article**: This is category free of topic, as long as it is coherent to be published in this project. I recommend it to publish specifications, actual articles, et cetera.

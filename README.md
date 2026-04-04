# ASM Reference

This project is supposed to make available a compilation of specific data in regards to general low-level data, making it accessible for people demanding it, or for newbies to learn with less friction. Also, so people don't have to learn totally from AI.

## Contributions

Feel free to write for this project, you may write an article at any time, or a specific topic if missing. 
You may either make a pull request, or send me an email.

## Regarding AI

Unless I don't notice, I will deny any contribution with the use of AI. Assuming AI is bad, the exception "I don't notice" is because if I really do not perceive any AI trait, the quality might be good enough.

## License

This project is licensed under [GNU General Public License v2.0](LICENSE-GPL2.md)

at your option.

---


# Terminology
The rest of this README follows the *RFC 2119*. The word SIGNIFICANT when, and only when, in all capital letters means something that impacts both the interpretation, and perception of an instance.

# Authorship
All texts have an authorship, as defined in `details.json`. A person may contribute to another's text. If it happens, if the contribution is SIGNIFICANT, the authorship of the contributor SHALL be added to the text.

No one can contribute to people's articles, nor can these be edited. The authorship of an article MUST be defined only once.

> `details.json` is a file that keeps track of categories, authors, texts, and some render related instances.

# Publishment process

For a person to publish a text into the project, they SHOULD send a request draft. It being done by either sending an email to [featheredref@protonmail.com], or creating an issue with the label "WRITING".

The draft request MUST be a message which SHALL not be blank, and a linked markdown file for the draft. In the message, the sender SHALL describe in what category the sender wants to publish on. 

If the project does not have the sender registered as an author, the sender MUST say which valid social link SHALL be linked to him, along the username, name (don't have to be the full name), and an image url for it to be a profile picture. 

Case the sender sends an incoherent category, and refuse to change it, the request SHALL be ignored (closed if it is a github issue).

Once the request is sent, a collaborator will review it. If the draft have factually problematic text, and/or grammar issues, these SHALL be reported to the sender, who must edit and reply with another version.

If the draft state doesn't have grammar issues, and factually problematic text, the text will be inserted into the project. It being only publicly available on a web reference, after a release.

Upon publishment, if the request is a github issue, it might be closed.

> A markdown h1 ("#") is mandatory on the start of a text/draft, as the renderer uses it as a reference.

# Releases

If any mutation happen, a new release of the web version SHALL be released in the period between a day, and three months. Generally a delay (more than a day to release) may occur if there is too many on-going requests, and the collaborators think that delaying a release is worth doing.

A delay may also happen if changes are being done into the website's renderer.

# Writing format

Even while no writing format is ensured, nor enforced, its good to recommend a standard for writers to follow. It being, in the given order:

- **Abstract**: Start with a brief explanation of the text's contents, so a reader can know what they are reading about. Since this project is a reference, these abstracts may help queries.
- **Introduction**: It helps the reader to ground their view, using it as a definition of direction and subject may be useful.
- **Terminology**: If you have a writing bias, or have non-standard wording and/or expressions, defining their meaning in this section might be good for clarity.
- **Actual content**: At this "place" you write what you intend to.

# Categories
- **arch**: Directly related with hardware
- **linux**: Directly related with the linux, specifically the interaction Userpace with kernel, or specifically kernel internals.
- **instructions**: Assembly and Instructions of CPUs of an arch, if you pick this category, you must state the architecture (x86, ARM, Risc-V, et cetera).
- **network**: Specifically what is directly related with network protocols, and/or systems.
- **kernel**: Regarding kernels, kernel development.
- **disk**: Regarding disk (HDDs, SSDs, et cetera), and file systems.
- **encryption**: Regarding encryption
- **hashes**: Regarding hashes
- **algorithms**: Regarding algorithms
- **C**: Regarding the C Programming language
- **compilers**: Regarding compilers
- **VM**: Regarding virtual machines
- **article**: Keeping the mentioned regarding authorship, this is category free, as long as it is coherent to be published in this project.

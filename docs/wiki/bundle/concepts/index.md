# Concepts

* [Evaluation method of the earlier search tool](evaluation-method-of-the-earlier-search-tool.md) - How the earlier evaluation tool scored a search tool: snippets, gain 0/1/2, nDCG@10, three arms, terms from the question text.
* [serde-saphyr 2.0.0 reads a plain YAML number as text](serde-saphyr-2-0-0-reads-a-plain-yaml-number-as-text.md) - With default options, serde-saphyr 2.0.0 reads a plain scalar such as 4, 04, 0x0A, 4.0 or true into an Option<String> field as its text. An empty value or ~ gives None. A !!int tag, a list, or a duplicate key gives an error.

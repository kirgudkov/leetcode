class TrieNode {
	char: string;
	children = new Map<string, TrieNode>();

	heat?: number; // only terminal nodes have heat
	str?: string; // only terminal nodes have the whole string

	constructor(char: string) {
		this.char = char;
	}
}

class AutocompleteSystem {
	private trie_root = new TrieNode("-");
	private buf: string[] = [];

	constructor(sentences: string[], times: number[]) {
		sentences.forEach((str, i) => {
			this.insert_str(str, times[i]);
		});
	}

	input(c: string): string[] {
		if (c === "#") {
			this.insert_str(this.buf.join(""));
			this.buf = [];
			return [];
		} else {
			this.buf.push(c);
			return this.search_buf();
		}
	}

	private insert_str(str: string, heat: number = 1) {
		let node = this.trie_root;

		for (let sym of str) {
			if (!node.children.has(sym)) {
				node.children.set(sym, new TrieNode(sym));
			}

			node = node.children.get(sym)!;
		}

		// terminal node
		node.heat = (node.heat || 0) + heat;
		node.str = str;
	}

	private search_buf(): string[] {
		// 1. Find the last node that represents the current buffer
		let node = this.trie_root;

		for (const sym of this.buf) {
			if (!node.children.has(sym)) {
				// No words were found with the current prefix
				return [];
			} else {
				node = node.children.get(sym)!;
			}
		}

		// 2. Now we have the starting node from which we will collect all the words, sort them and return top 3
		let words: [word: string, heat: number][] = [];

		const q: TrieNode[] = [node];
		while (q.length) {
			const node = q.shift()!;

			// If we found a terminal node, add its string to the result
			if (node.heat && node.str) {
				words.push([node.str, node.heat]);
				continue;
			}

			for (const child of node.children.values()) {
				q.push(child);
			}
		}

		// Probably we could use a max heap here to avoid sorting
		return words.sort((a, b) => {
			if (a[1] === b[1]) {
				return a[0].localeCompare(b[0]);
			}

			return b[1] - a[1];
		}).slice(0, 3).map((word) => word[0]);
	}
}

export { AutocompleteSystem };

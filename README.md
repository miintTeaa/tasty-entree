# tasty-ntree
Generic rust n-trees implemented with generics and no dependencies (and that's a guarantree).

An [entrée](https://en.wikipedia.org/wiki/Entr%C3%A9e) is a dish that is served before a meal.

An [n-tree](https://en.wikipedia.org/wiki/M-ary_tree) however, is better explained by clicking the link, which will take you to wikipedia.

The implementation is also Send + Sync (as long as the data stored inside the tree is also Send + Sync).
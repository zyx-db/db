# BTree

## Clustered Vs Unclustered (Primary vs Secondary)

Clustered Trees maintain the records at the leaf nodes.

Unclustered Trees maintain the Primary Key for a record at the leaf node.
This means there must be a second lookup for the actual record contents!

## Optimistic and Pessimistic Locking

When traversing a tree, we always do so in one direction, to prevent deadlocks.

Also, when performing an update on the tree there is a chance we must modify the overall structure of the tree. 
If we need to split a node, we cannot go back up and obtain a write lock!
Therefore we must obtain the lock beforehand. Or do we?

Optimistic Locking is performing the traversal with hopes that our operation will not cause any structural modifications.
We only need to obtain a write lock on the leaf node, allowing for more concurrent access of the internal nodes.
However, what if we do actually need to modify our structure?

Pessimistic Locking is obtaining write locks as we traverse downwards, and only dropping the lock if we are certain that the node below will not be split or merged.
This allows for less concurrent access, as all locks obtained are write locks.

Since the overwhelming majority of operations on a BTree do not modify the structure, we go ahead and try to perform and optimistic traversal.
If it doesn't work, we go ahead and use a pessimistic traversal.

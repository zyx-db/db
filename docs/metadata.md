# Metadata format

Total Metadata Pages: 5

## Free Page List

The first 4 pages are reserved for the free page list. 
The first 16 bits indicate the capacity of the database, or the amount of pages currently in the file, not including the metadata files.
The next 16 bits store the amount of pages which are currently in use.
This is used to more quickly determine whether it is necessary to allocate more space.

The remaining bits indicate whether or not a given page is in use. (1 represents usage)
If not in use, it is free to be used as needed.

## Table Info

A BTree of all tables and indexes is to be maintained, with the root node on page 5.
The format for this table is as follows:
```
Master Table {
    type: INT (0 for table, 1 for index),
    name: Text256 (index name, or the table name if its the clustered index),
    table name: Text256 (the table this tree is for),
    root page: INT (the page for the root node of the tree)
}
```
This tree will be indexed on the "table name" field.
It is used to find where a tree is on disk, as well as verifying existence.

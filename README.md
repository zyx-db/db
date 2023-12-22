# db's db

i'm currently watching through the cmu 15-445 lecture series, and wanted to give building a dbms a try!

currently i'm working on figuring out my page format and implementing it, as well as implementing the BTree structure

# goals

- utilities to convert the raw bytes in my pages ([u8; 4096]) into meaningful data, and vice versa
- concurrent implementations of data structures used! i hope to make the bulk of my data structures i used throughout this process!
- b+ tree indexing
- storing simple data types in user defined schemas (ex. no variable sized data like strings)
- accepting connections
- parsing requests (using a subset of sql for simplicity's sake)
- query execution (likely no optimization)

## remaining work

### Metadata

 - create script to generate a fresh database file

 - we can store table info in a "master_table" BTree. The root node for this table can be on a consistent page, ex page 1.

### Disk Manager

 - Currently our program crashes if we try and read a page not in the file. Ex: read page 1 with only 4kb in the db.dat file

### Buffer Pool

 - currently our program crashes if our buffer pool is unable to read in the page
 - this happens when all the frames are pinned at once
 - potentially will fix by returning option, for time being just make pool size big :skull:

### Indexing

 - Create Hash Map Index and B+ Tree Indexing schemes

### Query Execution

 - If I am to interpret the page here, create utilities to do so
 - We can store the tuple as a Vector<TupleField>, where TupleField is an enum containing any of the possible data types for a column.

## General Thoughts

 - Should the notion of a "page" that the system interacts with be a raw buffer of bytes, or should it be some interpreted value, given by the disk manager? Essentially should interpreting a page be up to the disk manager or someone else?

 - For our BTree when we need a new node, we need to get a new page. Likewise, we need to delete our page when removing a node. The disk manager's new / delete page interface is essentially the dynamic memory allocation for our data structures.

# notes

usage from fresh install:

create file with path ./files/db.dat

command to make file of 0s:

dd if=/dev/zero of=<output_file> bs=4096 count=<amt_of_pages>

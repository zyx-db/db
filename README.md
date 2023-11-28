# db's db

i'm currently watching through the cmu 15-445 lecture series, and wanted to give building a dbms a try!

currently i'm working on implementing the file io with a buffer pool manager and a cache eviction strategy.

# goals

- a disk manager to convert the raw bytes in my pages ([u8; 4096]) into meaningful data, and vice versa
- concurrent implementations of data structures used! i hope to make the bulk of my data structures i used throughout this process!
- b+ tree indexing
- storing simple data types in user defined schemas (ex. no variable sized data like strings)
- accepting connections
- parsing requests (using a subset of sql for simplicity's sake)
- query execution (likely no optimization)

## TODO

currently our program crashes if our buffer pool is unable to read in the page
this happens when all the frames are pinned at once
potentially will fix by returning option, for time being just make pool size big :skull:

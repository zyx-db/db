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

currently the LruK has some issues at initialization

it has no "pages" so i can't just pop from the heap to find a victim. currently the way i have that data structure set up, i can't just add dummy pages

this is because the LRU cache is tightly coupled with the Pool's pages. it should not need to know what pages are cached, it should just track the different frames!!!! I need to fix this!!!!

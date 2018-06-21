VEC2BIN
=======

This is a toy project for learning Rust programming language. 
It converts a text file of word vectors into a binary format.
That is,
```
m n
w_1<tab>f_11<tab>f_12...f_1n
w_2<tab>f_21<tab>f_22...f_2n
...
w_m<tab>f_m1<tab>f_m2...f_mn
```
to a text file contain the list of words and a HDF5 file contain the matrix in binary format.
This is compatible with fastText format.

The release build can process 2.5 million words in 100 seconds.


NOTE
----

When learning the owner/borrow checker/lifetime, I got the feeling that Rust coders need to be more specific about what scope owns the variable (who will release the variable eventually).
The language seems designed around the concept of ownerships.

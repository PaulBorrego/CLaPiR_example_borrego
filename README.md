Notes:

Whenever we don't use a variable in the vector copy test the copy_from_slice it works instantly

Speed of Copying Vector to linked List versus vector

// Results    :

// Linked List: 0.5148971

// Vector Fair: 0.0199759

// Vector Fast Copy: 0.0000001


Speed of copying linked list to linked list versus vector

// Results    :

// Linked List: 0.5716007

// Vector Fair: 0.1087586


Speed of vector copy with iter vs copy_from_slice going forward

//Returns:

// At 10000: Speed up is 25.3137x faster

// At 100000: Speed up is 6.6592813x faster

// At 1000000: Speed up is 1.7620945x faster

// At 10000000: Speed up is 1.8490372x faster

// At 100000000: Speed up is 1.7937524x faster


Speed of vector copy with iter vs copy_from_slice going backward

//Returns:

// At 100000000: Speed up is 1.7929674x faster

// At 10000000: Speed up is 1.9418052x faster

// At 1000000: Speed up is 1.6404991x faster

// At 100000: Speed up is 4.636007x faster

// At 10000: Speed up is 25.574482x faster


Speed of vector copy with iter vs copy_from_slice when we just use 10_000_000 instead of a variable

// Results : 

// Naive copy took 0.0242707s

// Fast copy took 0.0000001s

// For 10000000: Fast copy is infx faster


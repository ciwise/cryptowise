# Solutions

## Set 1

### Challenge 1

### Challenge 2

### Challenge 3

### Challenge 4

This challenge was handled by using the function of challenge 3 and processing each line in 4.txt in a
similar manner. First, I looked for lines that would produce more than 2 words. In some cases, the from_utf8
function after the XOR function would return an error in the Result object. I used a match operation and if
the utf8 string could not be encoded from the bytes given by the XOR, then I would set the string result to
"UTF-8 Error" or two words. The lines that gave up more than 2 words were run again to get the secret message.
I printed the line no. the decimal key number to the character used, and the secret message given when the 
character was used in an XOR operation with the given hex encoded string line.

The solution to this is:

- C4:Line no:171 - 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
- C4:The decimal key is: 53
- C4:The secret message is: Now that the party is jumping

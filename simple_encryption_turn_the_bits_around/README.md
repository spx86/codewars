For encrypting the strings this region of chars is given (in this order!):

all letters (ascending, first all UpperCase, then all LowerCase)
all digits (ascending)
the both chars: " "(space) and "."
These are 64 chars! (This region is zero-based.)
So, every char from this region has 6 bits!

Write two methods:

string Encrypt(string text)
string Decrypt(string encryptedText)
Prechecks:

1. If the input-string (for both methods!) has chars, that are not in the region, 
   throw an Exception(C#, C++, Python) or Error(JavaScript).
2. If the input-string is null or empty return exactly this value!
For building the encrypted string:
For every char use these rules:

1. Change the fifth bit of the char and the first bit of the next char. (C1.5 <==> C2.1, only if there is a next char!)
2. Inverse the second and the forth bit of the char.           (2,4 => 0->1 or 1->0)
3. Change the first 3 bit against the last 3 bit of the char.  (1,2,3 <==> 4,5,6)
4. Change every odd bit against the following bit of the char. (1 <==> 2, 3 <==> 4, 5 <==> 6)
5. Reverse the whole line of bits of the char.
6. Change the first against the third bit of the char.         (1 <==> 3)
Of course every rule takes the changed char from the previous rule.
The position of a bit starts from the beginning and not from the end! (So maybe in different order as you thought! See the example.)

Example for the first rule for "B9":

pos: 1 2 3 4 5 6 
B -> 0 0 0 0 0 1
9 -> 1 1 1 1 0 1
Change pos 5 from "B" against pos 1 from "9".
 ->  0 0 0 0 1 1 
 ->  0 1 1 1 0 1 


This kata is part of the Simple Encryption Series:
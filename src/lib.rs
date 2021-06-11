/*
-- Pin Assignments --
Pin#    Function
1       BUSY
2       WRITE STROBE
3       D7 (MSB)
4       D6
5       D5
6       D4
7       D3
8       D2
9       D1
10      D0 (LSB)
11      +5V @ 370mA (TYP)
12      GROUND (COMMON)
13      NOT USED
14      !RESET
*/


/*
-- Control Codes --
Data    Description
08      BACK SPACE CURSOR LOCATION ONE POSITION
09      ADVANCE CURSOR LOCATION ONE POSITION
0A      LINE FEED (vertical scroll from bottom line; cursor positions to the left-most grid)
0D      CARRIAGE RETURN (returns cursor to left-most character position of the same line; does not clear display)
0E      MAKE CURSOR INDICATOR INVISIBLE (the cursor location counter continues to function but there is no visible indicator of next location)
0F      MAKE CURSOR INDICATOR VISIBLE (Flashing “reverse rubout character”)
<11>    NORMAL DATA ENTRY WITH WRAPAROUND TO HOME POSITION (data enters beginning at the home position)
<12>    OVERWRITE OF RIGHT-MOST CHARACTER ON THE BOTTOM LINE ONLY/ AUTOMATIC CARRIAGE RETURN OFF
<13>    HORIZONTAL SCROLL MODE (from right to left on bottom line only, after line has been filled)
14      RESET
15      DISPLAY CLEAR (returns cursor to upper left-most position of multi-line displays)
16      CURSOR HOME (returns cursor to upper left-most position)
18      BEGIN USER DEFINED CHARACTER LOADING
19      BIT 7 HIGH FOR NEXT BYTE ONLY
*/

/*
-- Brightness Levels --
Data    Description
1C      Dimmest (12%)
1D      Dim (25%)
1E      Bright (50%)
1F      Brightest (100%)
Display automatically defaults to "Brightest" after power-up.
*/

/*
-- Character Map --
00-20   [unused]
21      !
23      "
23      #
24      $
25      %
26      &
27      '
28      (
29      )
2A      *
2B      +
2C      ,
2D      -
2E      .
2F      /
30      0
31      1
32      2
33      3
34      4
35      5
36      6
37      7
38      8
39      9
3A      :
3B      ;
3C      <
3D      =
3E      >
3F      ?
40      @
41      A
42      B
43      C
44      D
45      E
46      F
47      G
48      H
49      I
4A      J
4B      K
4C      L
4D      M
4E      N
4F      O
50      P
51      Q
52      R
53      S
54      T
55      U
56      V
57      W
58      X
59      Y
5A      Z
5B      [
5C      ￥
5D      ]
5E      ^
5F      _
60      `
61      a
62      b
63      c
64      d
65      e
66      f
67      g
68      h
69      i
6A      j
6B      k
6C      l
6D      m
6E      n
6F      o
70      p
71      q
72      r
73      s
74      t
75      u
76      v
77      w
78      x
79      y
7A      z
7B      {
7C      €
7D      }
7E      °
7F      [alternating dots]
80-9F   [unused]
A0      Ä
A1      Å
A2      Á
A3      Ç
A4      Â
A5      Æ
A6      É
A7      È
A8      Ê
A9      Ï
AA      Í
AB      Ì
AC      Î
AD      Ö
AE      Ñ
AF      0
B0      Ó
B1      Ò
B2      À
B3      ß
B4      Ë
B5      Ü
B6      Ú
B7      Ù
B8      Û
B9       ֯
BA      ±
BB      ÷
BC      ➡ (filled)
BD      ➡ (unfilled)
BE      ♦
BF      Ô
C0      ä
C1      å
C2      á
C3      ç
C4      â
C5      æ
C6      é
C7      è
C8      ê
C9      ï
CA      í
CB      ì
CC      î
CD      ö
CE      ñ
CF      ø
D0      ó
D1      ò
D2      à
D3      ¢
D4      ë
D5      ü
D6      ú
D7      ù
D8      û
D9      ○
DA      ⊙
DB      [filled circle]
DC      [unfilled square]
DD      [filled square]
DE      [unfilled diamond]
DF      ô
E0      [0]
E1      [1]
E2      [2]
E3      [3]
E4      [4]
E5      [5]
E6      [6]
E7      [7]
E8      [8]
E9      [9]
EA      [empty]
EB      [empty]
EC      [empty]
ED      [empty]
EE      [empty]
EF      [filled]
F0-F5   [unused]
F6-FF   [user-defined char]
*/

// aiml2rs -- Generated on Sat Sep 26 06:35:51 2009

+ wordplay
- Welcome to my anagram game.\nI am going to jumble a nine letter word up and you have to guess what it is.\nYou must solve each anagram correctly to continue.\nType START to begin the anagram game.

+ start
% type start to begin the anagram game
- <set anagramscore=0>WORDPLAY
^ Here comes the first anagram.\nI've jumbled a word up, but which word is it?\n(Type QUITGAME if you give up).\n\n{@xanagram}

+ quitgame
- The answer was <get anagramanswer>!\nYour final score was :<get anagramscore>.\n\nType START to begin the anagram game.

+ xanagramyes
- {@xanagramscore}{random}Yes. Well done.|That's correct.|Excellent.|Very good.|That is the correct answer.|Well done! You are good at these.{/random}\nYour score is :<get anagramscore>\n{random}Here comes the next one.|Let's try another anagram.|Tell me which word this is?|Can you solve the next anagram?{/random}\n(Type QUITGAME if you give up).\n\n{@xanagram}

+ xanagram
- <li>Clue: Stars\nNOMSTORAY
^ <set anagramanswer=ASTRONOMY>
^ </li><li>Clue: Me! ;-)\nAFLUTUBEI
^ <set anagramanswer=BEAUTIFUL>
^ </li><li>Clue: Seashore\nSELCATION
^ <set anagramanswer=COASTLINE>
^ </li><li>Clue: Copy\nUPILACTED
^ <set anagramanswer=DUPLICATE>
^ </li><li>Clue: Creature\nTRAMWOREH
^ <set anagramanswer=EARTHWORM>
^ </li><li>Clue: Next\nWILOGNOLF
^ <set anagramanswer=FOLLOWING>
^ </li><li>Clue: Males\nNEEENTGML
^ <set anagramanswer=GENTLEMEN>
^ </li><li>Clue: Emphasise\nLIIGGHHHT
^ <set anagramanswer=HIGHLIGHT>
^ </li><li>Clue: Unreadable\nLIBELIGEL
^ <set anagramanswer=ILLEGIBLE>
^ </li><li>Clue: Escape\nIJKLARABE
^ <set anagramanswer=JAILBREAK>
^ </li><li>Clue: Typing\nBREAKYDOS
^ <set anagramanswer=KEYBOARDS>
^ </li><li>Clue: Oil\nBUCALRITE
^ <set anagramanswer=LUBRICATE>
^ </li><li>Clue: Strength\nDEMAUTING
^ <set anagramanswer=MAGNITUDE>
^ </li><li>Clue: Shopkeeper\nSWEETNANG
^ <set anagramanswer=NEWSAGENT>
^ </li><li>Clue: Rude\nFEFEVSION
^ <set anagramanswer=OFFENSIVE>
^ </li><li>Clue: Canopy\nUTRAPACHE
^ <set anagramanswer=PARACHUTE>
^ </li><li>Clue: Enquiries\nOESQUINTS
^ <set anagramanswer=QUESTIONS>
^ </li><li>Clue: Weather\nSPRAINDOR
^ <set anagramanswer=RAINDROPS>
^ </li><li>Clue: Name\nSTRANGEIU
^ <set anagramanswer=SIGNATURE>
^ </li><li>Clue: Fruit\nRANTINGEE
^ <set anagramanswer=TANGERINE>
^ </li><li>Clue: Challenge\nTUMTUMAIL
^ <set anagramanswer=ULTIMATUM>
^ </li><li>Clue: Food\nGEBLEVATE
^ <set anagramanswer=VEGETABLE>
^ </li><li>Clue: Great\nFLOUNDERW
^ <set anagramanswer=WONDERFUL>
^ </li><li>Clue: Foreign\nPHONEBEXO
^ <set anagramanswer=XENOPHOBE>
^ </li><li>Clue: Child\nSTRONGEYU
^ <set anagramanswer=YOUNGSTER>
^ </li><li>Clue: Scientist\nGLISTOOZO
^ <set anagramanswer=ZOOLOGIST>
^ </li><li>Clue: Cartoon\nIANMOTIAN
^ <set anagramanswer=ANIMATION>
^ </li><li>Clue: Mask\nFLOBINDDL
^ <set anagramanswer=BLINDFOLD>
^ </li><li>Clue: Occupation\nARETRACKE
^ <set anagramanswer=CARETAKER>
^ </li><li>Clue: Grower\nPEERVOLED
^ <set anagramanswer=DEVELOPER>
^ </li><li>Clue: Meeting\nNONETRUCE
^ <set anagramanswer=ENCOUNTER>
^ </li><li>Clue: Fierce\nOSCOURIFE
^ <set anagramanswer=FEROCIOUS>
^ </li><li>Clue: Vivid\nHARPGALIC
^ <set anagramanswer=GRAPHICAL>
^ </li><li>Clue: Greeting\nHESKANDAH
^ <set anagramanswer=HANDSHAKE>
^ </li><li>Clue: Vital\nTROMPAINT
^ <set anagramanswer=IMPORTANT>
^ </li><li>Clue: Animal\nJIFYSHELL
^ <set anagramanswer=JELLYFISH>
^ </li><li>Clue: Hostage\nDEPADPINK
^ <set anagramanswer=KIDNAPPED>
^ </li><li>Clue: Existence\nSILLYFEET
^ <set anagramanswer=LIFESTYLE>
^ </li><li>Clue: Players\nISCAMINUS
^ <set anagramanswer=MUSICIANS>
^ </li><li>Clue: Obvious\nYUNTRALAL
^ <set anagramanswer=NATURALLY>
^ </li><li>Clue: Awful\nUNSIXBOOO
^ <set anagramanswer=OBNOXIOUS>
^ </li><li>Clue: Dig\nTRAPETEEN
^ <set anagramanswer=PENETRATE>
^ </li><li>Clue: Faster\nDECKNIQUE
^ <set anagramanswer=QUICKENED>
^ </li><li>Clue: Parents\nSTARELION
^ <set anagramanswer=RELATIONS>
^ </li><li>Clue: Happy\nFEASTIDIS
^ <set anagramanswer=SATISFIED>
^ </li><li>Clue: Communicate\nHEEPLONET
^ <set anagramanswer=TELEPHONE>
^ </li><li>Clue: Capitals\nSCRAPEUPE
^ <set anagramanswer=UPPERCASE>
^ </li><li>Clue: Opportunities\nCACAVINES
^ <set anagramanswer=VACANCIES>
^ </li><li>Clue: Decoration\nLAPPERLAW
^ <set anagramanswer=WALLPAPER>
^ </li><li>Clue: Music\nHEXPLOONY
^ <set anagramanswer=XYLOPHONE>
^ </li><li>Clue: Time\nSTRAYEDEY
^ <set anagramanswer=YESTERDAY>
^ </li><li>Clue: Animals\nOKEPEROZE
^ <set anagramanswer=ZOOKEEPER>
^ </li></random>

+ xanagramscore
* <get anagramscore> eq 0 => <set anagramscore=1>
* <get anagramscore> eq 1 => <set anagramscore=2>
* <get anagramscore> eq 2 => <set anagramscore=3>
* <get anagramscore> eq 3 => <set anagramscore=4>
* <get anagramscore> eq 4 => <set anagramscore=5>
* <get anagramscore> eq 5 => <set anagramscore=6>
* <get anagramscore> eq 6 => <set anagramscore=7>
* <get anagramscore> eq 7 => <set anagramscore=8>
* <get anagramscore> eq 8 => <set anagramscore=9>
* <get anagramscore> eq 9 => <set anagramscore=10>
* <get anagramscore> eq 10 => <set anagramscore=11>
* <get anagramscore> eq 11 => <set anagramscore=12>
* <get anagramscore> eq 12 => <set anagramscore=13>
* <get anagramscore> eq 13 => <set anagramscore=14>
* <get anagramscore> eq 14 => <set anagramscore=15>
* <get anagramscore> eq 15 => <set anagramscore=16>
* <get anagramscore> eq 16 => <set anagramscore=17>
* <get anagramscore> eq 17 => <set anagramscore=18>
* <get anagramscore> eq 18 => <set anagramscore=19>
* <get anagramscore> eq 19 => <set anagramscore=20>
* <get anagramscore> eq 20 => <set anagramscore=21>
* <get anagramscore> eq 21 => <set anagramscore=22>
* <get anagramscore> eq 22 => <set anagramscore=23>
* <get anagramscore> eq 23 => <set anagramscore=24>
* <get anagramscore> eq 24 => <set anagramscore=25>
* <get anagramscore> eq 25 => <set anagramscore=26>
* <get anagramscore> eq 26 => <set anagramscore=27>
* <get anagramscore> eq 27 => <set anagramscore=28>
* <get anagramscore> eq 28 => <set anagramscore=29>
* <get anagramscore> eq 29 => <set anagramscore=30>
* <get anagramscore> eq 30 => <set anagramscore=31>
* <get anagramscore> eq 31 => <set anagramscore=32>
* <get anagramscore> eq 32 => <set anagramscore=33>
* <get anagramscore> eq 33 => <set anagramscore=34>
* <get anagramscore> eq 34 => <set anagramscore=35>
* <get anagramscore> eq 35 => <set anagramscore=36>
* <get anagramscore> eq 36 => <set anagramscore=37>
* <get anagramscore> eq 37 => <set anagramscore=38>
* <get anagramscore> eq 38 => <set anagramscore=39>
* <get anagramscore> eq 39 => <set anagramscore=40>
* <get anagramscore> eq 40 => <set anagramscore=41>
* <get anagramscore> eq 41 => <set anagramscore=42>
* <get anagramscore> eq 42 => <set anagramscore=43>
* <get anagramscore> eq 43 => <set anagramscore=44>
* <get anagramscore> eq 44 => <set anagramscore=45>
* <get anagramscore> eq 45 => <set anagramscore=46>
* <get anagramscore> eq 46 => <set anagramscore=47>
* <get anagramscore> eq 47 => <set anagramscore=48>
* <get anagramscore> eq 48 => <set anagramscore=49>
* <get anagramscore> eq 49 => <set anagramscore=50>
- 

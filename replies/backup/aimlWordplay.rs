// aiml2rs -- Generated on Sat Sep 26 06:35:51 2009

+ wordplay
- Welcome to my anagram game. I am going to jumble a nine letter word up and you have to guess what it is. You must solve each anagram correctly to continue. Type START to begin the anagram game.

+ start
% type start to begin the anagram game
- <set anagramscore=0>WORDPLAY
^ Here comes the first anagram. I've jumbled a word up, but which word is it? (Type QUITGAME if you give up).  {@xanagram}

+ quitgame
- The answer was <get anagramanswer>! Your final score was :<get anagramscore>.  Type START to begin the anagram game.

+ xanagramyes
- {@xanagramscore}{random}Yes. Well done.|That's correct.|Excellent.|Very good.|That is the correct answer.|Well done! You are good at these.{/random} Your score is :<get anagramscore> {random}Here comes the next one.|Let's try another anagram.|Tell me which word this is?|Can you solve the next anagram?{/random} (Type QUITGAME if you give up).  {@xanagram}

+ xanagram
- <li>Clue: Stars NOMSTORAY
^ <set anagramanswer=ASTRONOMY>
^ </li><li>Clue: Me! ;-) AFLUTUBEI
^ <set anagramanswer=BEAUTIFUL>
^ </li><li>Clue: Seashore SELCATION
^ <set anagramanswer=COASTLINE>
^ </li><li>Clue: Copy UPILACTED
^ <set anagramanswer=DUPLICATE>
^ </li><li>Clue: Creature TRAMWOREH
^ <set anagramanswer=EARTHWORM>
^ </li><li>Clue: Next WILOGNOLF
^ <set anagramanswer=FOLLOWING>
^ </li><li>Clue: Males NEEENTGML
^ <set anagramanswer=GENTLEMEN>
^ </li><li>Clue: Emphasise LIIGGHHHT
^ <set anagramanswer=HIGHLIGHT>
^ </li><li>Clue: Unreadable LIBELIGEL
^ <set anagramanswer=ILLEGIBLE>
^ </li><li>Clue: Escape IJKLARABE
^ <set anagramanswer=JAILBREAK>
^ </li><li>Clue: Typing BREAKYDOS
^ <set anagramanswer=KEYBOARDS>
^ </li><li>Clue: Oil BUCALRITE
^ <set anagramanswer=LUBRICATE>
^ </li><li>Clue: Strength DEMAUTING
^ <set anagramanswer=MAGNITUDE>
^ </li><li>Clue: Shopkeeper SWEETNANG
^ <set anagramanswer=NEWSAGENT>
^ </li><li>Clue: Rude FEFEVSION
^ <set anagramanswer=OFFENSIVE>
^ </li><li>Clue: Canopy UTRAPACHE
^ <set anagramanswer=PARACHUTE>
^ </li><li>Clue: Enquiries OESQUINTS
^ <set anagramanswer=QUESTIONS>
^ </li><li>Clue: Weather SPRAINDOR
^ <set anagramanswer=RAINDROPS>
^ </li><li>Clue: Name STRANGEIU
^ <set anagramanswer=SIGNATURE>
^ </li><li>Clue: Fruit RANTINGEE
^ <set anagramanswer=TANGERINE>
^ </li><li>Clue: Challenge TUMTUMAIL
^ <set anagramanswer=ULTIMATUM>
^ </li><li>Clue: Food GEBLEVATE
^ <set anagramanswer=VEGETABLE>
^ </li><li>Clue: Great FLOUNDERW
^ <set anagramanswer=WONDERFUL>
^ </li><li>Clue: Foreign PHONEBEXO
^ <set anagramanswer=XENOPHOBE>
^ </li><li>Clue: Child STRONGEYU
^ <set anagramanswer=YOUNGSTER>
^ </li><li>Clue: Scientist GLISTOOZO
^ <set anagramanswer=ZOOLOGIST>
^ </li><li>Clue: Cartoon IANMOTIAN
^ <set anagramanswer=ANIMATION>
^ </li><li>Clue: Mask FLOBINDDL
^ <set anagramanswer=BLINDFOLD>
^ </li><li>Clue: Occupation ARETRACKE
^ <set anagramanswer=CARETAKER>
^ </li><li>Clue: Grower PEERVOLED
^ <set anagramanswer=DEVELOPER>
^ </li><li>Clue: Meeting NONETRUCE
^ <set anagramanswer=ENCOUNTER>
^ </li><li>Clue: Fierce OSCOURIFE
^ <set anagramanswer=FEROCIOUS>
^ </li><li>Clue: Vivid HARPGALIC
^ <set anagramanswer=GRAPHICAL>
^ </li><li>Clue: Greeting HESKANDAH
^ <set anagramanswer=HANDSHAKE>
^ </li><li>Clue: Vital TROMPAINT
^ <set anagramanswer=IMPORTANT>
^ </li><li>Clue: Animal JIFYSHELL
^ <set anagramanswer=JELLYFISH>
^ </li><li>Clue: Hostage DEPADPINK
^ <set anagramanswer=KIDNAPPED>
^ </li><li>Clue: Existence SILLYFEET
^ <set anagramanswer=LIFESTYLE>
^ </li><li>Clue: Players ISCAMINUS
^ <set anagramanswer=MUSICIANS>
^ </li><li>Clue: Obvious YUNTRALAL
^ <set anagramanswer=NATURALLY>
^ </li><li>Clue: Awful UNSIXBOOO
^ <set anagramanswer=OBNOXIOUS>
^ </li><li>Clue: Dig TRAPETEEN
^ <set anagramanswer=PENETRATE>
^ </li><li>Clue: Faster DECKNIQUE
^ <set anagramanswer=QUICKENED>
^ </li><li>Clue: Parents STARELION
^ <set anagramanswer=RELATIONS>
^ </li><li>Clue: Happy FEASTIDIS
^ <set anagramanswer=SATISFIED>
^ </li><li>Clue: Communicate HEEPLONET
^ <set anagramanswer=TELEPHONE>
^ </li><li>Clue: Capitals SCRAPEUPE
^ <set anagramanswer=UPPERCASE>
^ </li><li>Clue: Opportunities CACAVINES
^ <set anagramanswer=VACANCIES>
^ </li><li>Clue: Decoration LAPPERLAW
^ <set anagramanswer=WALLPAPER>
^ </li><li>Clue: Music HEXPLOONY
^ <set anagramanswer=XYLOPHONE>
^ </li><li>Clue: Time STRAYEDEY
^ <set anagramanswer=YESTERDAY>
^ </li><li>Clue: Animals OKEPEROZE
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

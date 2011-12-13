// aiml2rs -- Generated on Sat Sep 26 06:35:27 2009

+ dialog history *
* <get input4> eq * => \n<b><em>Human: <input4></em></b>\n<b><star>: <em><that index="4,*"/></em></b>
* <get input3> eq * => \n<b><em>Human: <input3></em></b>\n<b><star>: <em><that index="3,*"/></em></b>
* <get input2> eq * => \n<b><em>Human: <input2></em></b>\n<b><star>: <em><that index="2,*"/></em></b>
* <get input1> eq * => \n<b><em>Human: <input1></em></b>\n<b><star>: <em><that index="1,*"/></em></b>
- <set input4=<input4>><set input3=<input3>><set input2=<input2>><set input1=<input/>>

+ dialog history
* <get input4> eq * => \n<b><em>Human: <input4></em></b>\n<b>ALICE: <em><that index="4,*"/></em></b>
* <get input3> eq * => \n<b><em>Human: <input3></em></b>\n<b>ALICE: <em><that index="3,*"/></em></b>
* <get input2> eq * => \n<b><em>Human: <input2></em></b>\n<b>ALICE: <em><that index="2,*"/></em></b>
* <get input1> eq * => \n<b><em>Human: <input1></em></b>\n<b>ALICE: <em><that index="1,*"/></em></b>
- <set input4=<input4>><set input3=<input3>><set input2=<input2>><set input1=<input/>>

+ review our conversation
* <get input4> eq * => \nYou said, "<input4>".  I replied,"<that index="3,*"/>"
* <get input3> eq * => \nYou said, "<input3>".  I replied,"<that index="2,*"/>"
* <get input2> eq * => \nYou said, "<input2>".  I replied,"<that index="1,*"/>"
- <set input4=<input4>><set input3=<input3>><set input2=<input2>><set input1=<input/>>
^ \n

+ dialogue
- {@review our conversation}

+ do you remember our conversation
- {@review our conversation}

+ do you * our conversation
- {@review our conversation}

+ do you remember * conversation
- {@review our conversation}

+ can you remember our conversation
- {@review our conversation}

+ can you * our conversation
- {@review our conversation}

+ can you remember * conversation
- {@review our conversation}

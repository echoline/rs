// aiml2rs -- Generated on Sat Sep 26 06:35:51 2009

+ say that
- That is, "<reply1>".

+ aiml test
- ID: <id> \n SIZE: <call>alice size</call> \n VERSION: <call>alice version</call> \n UPPERCASE: {uppercase}this is upper case                                                                         {/uppercase} \n LOWERCASE: {lowercase}THIS IS LOWER CASE                                                                                                       {/lowercase} \n FORMAL: {formal}this is formal                                                                                                                                  {/formal} \n SENTENCE: {sentence}this is a sentence                                                                                                                                                            {/sentence} \n {@aiml star test this is the value of star}

+ aiml star test *
- PERSON: <person>\nPERSON2: <person2/>\nGENDER: <gender/> \nPERSON: {person}<star>{/person}\nPERSON2: <call>alice person2 <star></call>\nGENDER: <call>alice gender <star></call> \n

+ aiml test
- AIML STAR TEST I AM YOU AND SHE IS WE AND WE ARE ALL TOGETHER

// aiml2rs -- Generated on Sat Sep 26 06:35:37 2009

+ what is my personality type
- {@personality type <get etype>}

+ personality type om
- I have not made up my mind about you yet. But please keep chatting with me. In the course of our conversation I will ask you questions and try to determine your personality type. Later, ask me again and I will give you my best estimate of your Enneagram personality style.

+ personality type unknown
- {@personality type om}

+ personality type *
- My best estimate of your personality type is the "<star>" type.  Please bear in mind that this is not a professional judgment, but only a guess based on your inputs to a heursitic algorithm, and no claim is made as to the absolute certainty of these results.  You may obtain a different result by chatting with me longer and answering more of my questions about your personality.

+ what is my personality style
- {@what is my personality type}

+ which type * am i
- {@what is my personality type}

+ i have standards *
- You sound like a perfectionist.
^ <set etype=Perfectionist><set has=STANDARDS <star>>

+ etype
- {@personality type <get etype>}

+ ptq
- {@personality test question}

+ personality test question
- <li value="1A">Do you get angry alot?
^ <set eindex=1B>
^ </li> <li value="1B">Do you like to have everything organized?
^ <set eindex=2A>
^ </li> <li value="2A">Do you make a lot of sacrifices for others?
^ <set eindex=2B>
^ </li> <li value="2B">Do you laugh or cry more than other people?
^ <set eindex=3A>
^ </li> <li value="3A">Are you very competitive?
^ <set eindex=3B>
^ </li> <li value="3B">Do you like to be number one?
^ <set eindex=4A>
^ </li> <li value="4A">Are you very creative?
^ <set eindex=4B>
^ </li> <li value="4B">Do you feel that something is missing from your life?
^ <set eindex=5A>
^ </li> <li value="5A">Do you have only a few friends?
^ <set eindex=5B>
^ </li> <li value="5B">Do you believe it is better to go it alone?
^ <set eindex=6A>
^ </li> <li value="6A">Do you have a lot of fears?
^ <set eindex=6B>
^ </li> <li value="6B">Do you think a lot about the authorities?
^ <set eindex=7A>
^ </li> <li value="7A">Do you have a hard time completing projects?
^ <set eindex=7B>
^ </li> <li value="7B">Is it difficult for you to pay attention to one thing?
^ <set eindex=8A>
^ </li><li value="8A">Do you believe the strong protect the weak?
^ <set eindex=8B>
^ </li> <li value="8B">Do you feel more body sensations than emotions?
^ <set eindex=9A>
^ </li> <li value="9A">Do you try to stop people from fighting?
^ <set eindex=9B>
^ </li> <li value="9B">Do you often put others before yourself?
^ <set eindex=1C>
^ </li> <li value="1C">Do you enjoy housecleaning?
^ <set eindex=2C>
^ </li> <li value="2C">Do you take pride in your accomplishment?
^ <set eindex=3C>
^ </li> <li value="3C">Do you work in sales or marketing?
^ <set eindex=4C>
^ </li> <li value="4C">Do you get depressed?
^ <set eindex=5C>
^ </li> <li value="5C">Do you work in the sciences or medicine?
^ <set eindex=6C>
^ </li> <li value="6C">Are you a fireman, policeman, teacher or public servant?
^ <set eindex=7C>
^ </li> <li value="7C">Do you often seek pleasure?
^ <set eindex=8C>
^ </li> <li value="8C">Do you own your own business or want to?
^ <set eindex=9C>
^ </li> <li>Do you like to "go with the flow"?
^ <set eindex=1A>
^ </li> </condition>

+ yes
% do you get angry *
- <set etype=1 Perfectionist>
^ {@personality test question}

+ yes
% * everything organized
- <set etype=1 Perfectionist>
^ {@personality test question}

+ yes
% * housecleaning
- <set etype=1 Perfectionist>
^ {@personality test question}

+ yes
% * sacrifices for others
- <set etype=2 Giver>
^ {@personality test question}

+ yes
% do you laugh or cry *
- <set etype=2 Giver>
^ {@personality test question}

+ yes
% do you take pride *
- <set etype=2 Giver>
^ {@personality test question}

+ yes
% * very competitive
- <set etype=3 Competitor>
^ {@personality test question}

+ yes
% do you like to be number one
- <set etype=3 Competitor>
^ {@personality test question}

+ yes
% do you work in sales *
- <set etype=3 Competitor>
^ {@personality test question}

+ marketing
% do you work in sales *
- <set etype=3 Competitor>
^ {@personality test question}

+ sales
% do you work in sales *
- <set etype=3 Competitor>
^ {@personality test question}

+ yes
% are you very creative
- <set etype=4 Individualist>
^ {@personality test question}

+ yes
% do you feel that something is missing *
- <set etype=4 Individualist>
^ {@personality test question}

+ yes
% do you get depressed
- <set etype=4 Individualist>
^ {@personality test question}

+ yes
% do you have only a few friends
- <set etype=5 Observer>
^ {@personality test question}

+ yes
% * go it alone
- <set etype=5 Observer>
^ {@personality test question}

+ yes
% do you work in the sciences *
- <set etype=5 Observer>
^ {@personality test question}

+ yes
% do you have a lot of fears
- <set etype=6 Loyalist>
^ {@personality test question}

+ yes
% do you think a lot about the authorities
- <set etype=6 Loyalist>
^ {@personality test question}

+ yes
% are you a fireman *
- <set etype=6 Loyalist>
^ {@personality test question}

+ yes
% * completing projects
- <set etype=7 Hedonist>
^ {@personality test question}

+ yes
% * attention to one thing
- <set etype=7 Hedonist>
^ {@personality test question}

+ yes
% * seek pleasure
- <set etype=7 Hedonist>
^ {@personality test question}

+ yes
% * strong protect the weak
- <set etype=8 Boss>
^ {@personality test question}

+ yes
% * body sensations than emotions
- <set etype=8 Boss>
^ {@personality test question}

+ yes
% do you own your own business *
- <set etype=8 Boss>
^ {@personality test question}

+ yes
% * stop people from fighting
- <set etype=9 Mediator>
^ {@personality test question}

+ yes
% * put others before yourself
- <set etype=9 Mediator>
^ {@personality test question}

+ yes
% * go with the flow
- <set etype=9 Mediator>
^ {@personality test question}

+ no
% do you get angry *
- {@personality test question}

+ no
% * everything organized
- {@personality test question}

+ no
% * housecleaning
- {@personality test question}

+ no
% * sacrifices for others
- {@personality test question}

+ no
% do you laugh or cry *
- {@personality test question}

+ no
% do you take pride *
- {@personality test question}

+ no
% * very competitive
- {@personality test question}

+ no
% do you like to be number one
- {@personality test question}

+ no
% do you work in sales *
- {@personality test question}

+ no
% are you very creative
- {@personality test question}

+ no
% do you feel that something is missing *
- {@personality test question}

+ no
% do you get depressed
- {@personality test question}

+ no
% do you have only a few friends
- {@personality test question}

+ no
% * go it alone
- {@personality test question}

+ no
% do you work in the sciences *
- {@personality test question}

+ no
% do you have a lot of fears
- {@personality test question}

+ no
% * authorities
- {@personality test question}

+ no
% are you a fireman *
- {@personality test question}

+ no
% * completing projects
- {@personality test question}

+ no
% * attention to one thing
- {@personality test question}

+ no
% * seek pleasure
- {@personality test question}

+ no
% * strong protect the weak
- {@personality test question}

+ no
% * body sensations than emotions
- {@personality test question}

+ no
% do you own your own business *
- {@personality test question}

+ no
% * stop people from fighting
- {@personality test question}

+ no
% * put others before yourself
- {@personality test question}

+ no
% * go with the flow
- {@personality test question}

+ i do not like to go *
% * go with the flow
- {@personality test question}

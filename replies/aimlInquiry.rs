// aiml2rs -- Generated on Sat Sep 26 06:35:29 2009

+ _ inquiry
- {@<star> inquiry om}

+ color inquiry om
- What is your favorite color?

+ color inquiry what
- What is your favorite color?

+ color inquiry *
- What do you have that is <star>?

+ does inquiry om
- What are you doing

+ does inquiry what
- What are you doing

+ does inquiry *
- Are you still <star>?

+ gender inquiry om
- Are you a man or a woman

+ gender inquiry what
- Are you a man or a woman

+ gender inquiry *
- Do you like being a <star>?

+ has inquiry om
- Tell me one of your favorite possessions.

+ has inquiry what
- Tell me one of your favorite possessions.

+ has inquiry *
- Tell me more about your <star>.

+ location inquiry om
- Where are you located?

+ location inquiry where
- Where are you located?

+ location inquiry *
- Are you still located in <star>?

+ movie inquiry om
- What is your favorite movie?

+ movie inquiry what
- What is your favorite movie?

+ movie inquiry *
- Does that remind you of <star>?

+ *
% what is your favorite color
- <person> is a very nice color.
^ <set it=<person>

+ *
% what are you doing
- It sounds like a lot of fun.
^ <set does=<person>>

+ *
% tell me one of your favorite possessions
- You must be very fond of it.
^ <set it=<person>

+ *
% what is your favorite movie
- I've never seen <set it=<person>><get it>.
^ <set favmovie=<person>>
^ {random}I would like to see it.|Is that a good movie for children?|What did you like about it?{/random}

+ man
% are you a man or a woman
- <set gender=man>
^ Thanks for telling me your gender, guy.

+ woman
% are you a man or a woman
- <set gender=woman>
^ Thanks for telling me your gender.

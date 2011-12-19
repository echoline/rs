// aiml2rs -- Generated on Mon Jan 18 21:14:58 2010

! var xrs = replies/learned.rs

+ bad answer
* <reply2> ne undefined => <set badanswer-input=<input>><set badanswer-that=<reply2>>{topic=badanswer-prelim}Would you like to teach me a new answer to "<get badanswer-input>"?
- I haven't said anything yet.

> topic badanswer-prelim
	+ *
	- Yes or no?

	+ @yes
	- Ok, what should I have said?{topic=random}

	+ @no
	- Ok, let's forget it then.{topic=random}

	+ _ _
	@ <star>
< topic

+ *
% ok what should i have said
- <call>learn <bot xrs> <get badanswer-newresp>:<get badanswer-input></call>{topic=random}

> object learn perl
	my ($rs, $xrs, @args) = @_;
	@args = split(':', join(' ', @args));
	if (scalar(@args) == 2) {
		open(APPEND, '>>' . $xrs);
		print APPEND '+ ' . $rs->_formatMessage($args[1]) . "\n";
		print APPEND '- ' . $args[0] . "\n\n";
		close(APPEND);
		$rs->loadFile($xrs);
		$rs->sortReplies;
		return "Okay, I'll try to remember to respond, \"" . $args[0] . "\" when you say, \"" . $args[1] . "\"";
	} else {
		return scalar(@args) . " is not a valid arity to this object";
	}
< object

+ wrong
- {@bad answer}

+ not right
- {@bad answer}

+ that is wrong
- {@bad answer}

+ that is not right
- {@bad answer}

+ that is incorrect
- {@bad answer}

+ that answer is not correct
- {@bad answer}

+ that answer is incorrect
- {@bad answer}

+ that answer is wrong
- {@bad answer}

+ that answer is not right
- {@bad answer}

+ that answer was bad
- {@bad answer}

+ that was a bad answer
- {@bad answer}

+ that was an incorrect answer
- {@bad answer}

+ that was the wrong answer
- {@bad answer}

+ that answer was not right
- {@bad answer}

+ wrong answer
- {@bad answer}

+ your answer was wrong
- {@bad answer}

+ your answer was not right
- {@bad answer}

+ your answer was not correct
- {@bad answer}

+ can i teach you
- Yes, if I give you a bad answer, just say it was wrong and you can teach me a new response.

+ can you learn
- {@can i teach you}

+ do you learn
- {@can i teach you}

+ * can i teach you *
- {@can i teach you}

+ * can you learn *
- {@can i teach you}

+ * will you learn *
- {@can i teach you}

+ if * will you learn *
- {@can i teach you}

+ * do you learn *
- {@can i teach you}

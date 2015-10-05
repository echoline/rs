// aiml2rs -- Generated on Mon Jan 18 21:14:58 2010

! var xrs = replies/learned.rs

+ bad answer{weight=1000}
* <reply2> ne undefined => <set badanswer-input=<input>>{topic=badanswer-prelim}Would you like to teach me a new answer to "<get badanswer-input>"?
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
- <call>learn <bot xrs> {sentence}<star>.{/sentence}:<get badanswer-input></call>{topic=random}

> object learn perl
	my ($rs, $xrs, @args) = @_;
	@args = split(':', join(' ', @args));
	if (scalar(@args) >= 2) {
		open(READ, '<' . $xrs);
		my $query = $rs->_formatMessage($args[1]);
		my $found = 0;
		my $contents = '';
		while (<READ>) {
			$contents .= $_;
			if (/^\+\ $query/) {
				$contents .= '- ' . $args[0] . "\n";
				$found = 1;
			}
		}
		close (READ);
		if ($found == 0) {
			open(APPEND, '>>' . $xrs);
			print APPEND '+ ' . $rs->_formatMessage($args[1]) . "\n";
			print APPEND '- ' . $args[0] . "\n\n";
			close(APPEND);
		} else {
			open (WRITE, ">" . $xrs);
			print WRITE $contents;
			close(WRITE);
		}
		$rs->loadFile($xrs);
		$rs->sortReplies;
		if (scalar(@args) == 3 && $args[2] eq 'silent') {
			return "";
		}
		return "Okay, I'll try to remember to respond, \"" . $args[0] . "\" when you say, \"" . $args[1] . "\"";
	} else {
		return scalar(@args) . " is not a valid arity to this object";
	}
< object

+ wrong{weight=1000}
- {@bad answer}

+ not right{weight=1000}
- {@bad answer}

+ that is wrong{weight=1000}
- {@bad answer}

+ that is not right{weight=1000}
- {@bad answer}

+ that is incorrect{weight=1000}
- {@bad answer}

+ that answer is not correct{weight=1000}
- {@bad answer}

+ that answer is incorrect{weight=1000}
- {@bad answer}

+ that answer is wrong{weight=1000}
- {@bad answer}

+ that answer is not right{weight=1000}
- {@bad answer}

+ that answer was bad{weight=1000}
- {@bad answer}

+ that was a bad answer{weight=1000}
- {@bad answer}

+ that was an incorrect answer{weight=1000}
- {@bad answer}

+ that was the wrong answer{weight=1000}
- {@bad answer}

+ that answer was not right{weight=1000}
- {@bad answer}

+ wrong answer{weight=1000}
- {@bad answer}

+ your answer was wrong{weight=1000}
- {@bad answer}

+ your answer was not right{weight=1000}
- {@bad answer}

+ your answer was not correct{weight=1000}
- {@bad answer}

+ can i teach you{weight=1000}
- You can just say my answer was wrong and teach me a new response.

+ can you learn{weight=1000}
- {@can i teach you}

+ do you learn{weight=1000}
- {@can i teach you}

+ * can i teach you *{weight=1000}
- {@can i teach you}

+ * can you learn *{weight=1000}
- {@can i teach you}

+ * will you learn *{weight=1000}
- {@can i teach you}

+ if * will you learn *{weight=1000}
- {@can i teach you}

+ * do you learn *{weight=1000}
- {@can i teach you}

+ you do not know what * is{weight=1000}
- {@can i teach you}


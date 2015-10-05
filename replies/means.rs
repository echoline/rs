> object means perl
	my ($rs, $xrs, @args) = @_;
	@args = split(':', join(' ', @args));
	if (scalar(@args) == 2) {
		open(READ, '<' . $xrs);
		my $query = $rs->_formatMessage($args[1]);
		my $found = 0;
		my $contents = '';
		while (<READ>) {
			$contents .= $_;
			if (/^\+\ $query/) {
				$contents .= '@ ' . $args[0] . "\n";
				$found = 1;
			}
		}
		close (READ);
		if ($found == 0) {
			open(APPEND, '>>' . $xrs);
			print APPEND '+ ' . $rs->_formatMessage($args[1]) . "\n";
			print APPEND '@ ' . $args[0] . "\n\n";
			close(APPEND);
		} else {
			open (WRITE, ">" . $xrs);
			print WRITE $contents;
			close(WRITE);
		}
		$rs->loadFile($xrs);
		$rs->sortReplies;
		return "";
	}
< object

+ * means *{weight=1000}
- <call>means <bot xrs> <star2> *:<star1> *</call>
^ <call>means <bot xrs> * <star2> *:* <star1> *</call>
^ <call>means <bot xrs> * <star2>:* <star1></call>
^ I will try to remember that.

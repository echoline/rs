const Discord = require('discord.js');
const client = new Discord.Client();
const net = require('net');
const fs = require('fs');

var words;
var terry;

client.on('ready', () => {
	console.log(`Logged in as ${client.user.tag}!`);
});

client.on('message', msg => {
	var author = (new String(msg.author))
	var who = author.substring(2, author.length-1);
	var input = msg.content;
	if (msg.content.toLowerCase().substring(0, 5) === 'alice' && msg.content.substring(5, 6) != ' ')
		input = msg.content.substring(6, msg.content.length);
	console.log(`${who} says "${input}"`);

	if (msg.content.toLowerCase() === '!test') {
		msg.reply('Congratulations, test failed successfully!');
	}

	if (msg.content.toLowerCase() === '!oracle') {
		var s = "";
		for (var i = 0; i < 10; i++) {
			s += words[Math.floor(Math.random() * words.length)] + " ";
		}
		msg.reply(s);
	}

	if (msg.content.toLowerCase() === '!terry') {
		msg.reply(terry[Math.floor(Math.random() * terry.length)]);
	}

	if (who != '513876366685634560' && (msg.content.indexOf('<@513876366685634560>') != -1 || msg.content.toLowerCase().indexOf('alice') != -1)) {
		var alicemsg = who + '\007' + input;
		var alice = net.createConnection("/tmp/alice");
		alice.on("connect", function() {
			alice.write(alicemsg);
		});
		alice.on("data", function(data) {
			console.log(`>>> "${data}"`);
			msg.reply(data.toString());
		});
	}
});

fs.readFile('./words', function(err, data) {
	if (err) {
		throw err;
	}
	words = new String(data);
	words = words.split('\n');
});

fs.readFile('./terry', function(err, data) {
	if (err) {
		throw err;
	}
	terry = new String(data);
	terry = terry.split('\n');
});

client.login('API-KEY-HERE');

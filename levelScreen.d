module levelScreen;

import tango.stdc.stringz;
import ncurses;
import tango.io.Stdout; 

import dataScore;
import arrowSection;
import dancingMan;
import asciiSprite;
import narwhal;

class LevelScreen {
	DataScore _score;
	ArrowSection _arrowSect;
	DancingMan _dancingMan;
	AsciiSprite _spotlight;
	Narwhal _narwhal;

	bool _playing;

	this(char[] title) {
		_score = new DataScore(title);
		_arrowSect = new ArrowSection();
		_dancingMan = new DancingMan();
		_playing = true;
		_spotlight = new AsciiSprite("graphics/spotlight.txt", null, false, 10, 18);
		_narwhal = new Narwhal();
	}

	void draw() {
		move(0,0);
		_score.draw();
		_spotlight.drawSprite();
		_arrowSect.draw();
		_dancingMan.draw();
		_narwhal.draw();
	}	
}
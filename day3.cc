#include<iostream>
#include<map>
#include<vector>

/**
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and
then an elf at the North Pole calls him via radio and tells him where to move
next. Moves are always exactly one house to the north (^), south (v), east (>),
or west (<). After each move, he delivers another present to the house at his
new location.

However, the elf back at the north pole has had a little too much eggnog, and so
his directions are a little off, and Santa ends up visiting some houses more
than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to
the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house
at his starting/ending location. ^v^v^v^v^v delivers a bunch of presents to some
very lucky children at only 2 houses.

--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of
himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the
same starting house), then take turns moving based on instructions from the elf,
who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then
Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back
where they started. ^v^v^v^v^v now delivers presents to 11 houses, with Santa
going one direction and Robo-Santa going the other.
*/

class Grid {
	public:
	Grid() {
		count_of_houses_gifted = 1;
		grid[0][0] = 1;
	}
	uint getCountOfHousesGifted() {
		return count_of_houses_gifted;
	}
	void drop(int const coord_x, int const coord_y) {
		if(0 == grid[coord_y][coord_x]) {
			++count_of_houses_gifted;
		}
		++grid[coord_y][coord_x];
	}

	private:
	uint count_of_houses_gifted;
	std::map<int, std::map<int, int>> grid;
};

class Giver {
	public:
	Giver(Grid* const grid) : _grid(grid), _coord_x(0), _coord_y(0) {}
	void give() {
		_grid->drop(_coord_x, _coord_y);
		std::cout << "x: " << _coord_x << ", y: " << _coord_y << std::endl;
	}
	void move(char const direction) {
		switch(direction) {
			case '^':
				++_coord_y;
				break;
			case '>':
				++_coord_x;
				break;
			case 'v':
				--_coord_y;
				break;
			case '<':
				--_coord_x;
				break;
			default:
				std::cerr << "Unknown input: " << direction << std::endl;
				break;
		}

	}

	private:
	Grid* _grid;
	int _coord_x;
	int _coord_y;
};

class Director {
	public:
	Director(std::string const directions, std::vector<Giver*> givers)
	: _directions(directions), _givers(givers) {}

	/**
	Method direct processes the direction characters of the direction string one-by-one.
	These are fed to the giver objects cyclically.
	In other words, when the last giver in the list processes the direction, the first giver is next. */
	void direct() {
		std::vector<Giver*>::const_iterator giver_iterator = _givers.begin();
	    for(char direction : _directions) {
	    	std::cout << "read: " << direction << std::endl;

	    	(*giver_iterator)->move(direction);
	    	(*giver_iterator)->give();

			++giver_iterator;
			if(giver_iterator == _givers.end()) {
				giver_iterator = _givers.begin();
			}
	    }
	}
	private:
	std::string const _directions;
	std::vector<Giver*> _givers;
};

int part_one(std::string const directions) {
	Grid grid = Grid();
	Giver santa = Giver(&grid);
	std::vector<Giver*> givers {
		&santa
	};
	Director elf = Director(directions, givers);
	elf.direct();

	std::cout << "The total unique number of presents is: " << grid.getCountOfHousesGifted() << std::endl;
	return grid.getCountOfHousesGifted();
}

int part_two(std::string const directions) {
	Grid grid = Grid();
	Giver santa = Giver(&grid);
	Giver robo_santa = Giver(&grid);
	std::vector<Giver*> givers {
		&santa, &robo_santa
	};
	Director elf = Director(directions, givers);
	elf.direct();

	std::cout << "The total unique number of presents is: " << grid.getCountOfHousesGifted() << std::endl;
	return grid.getCountOfHousesGifted();
}

/**
Test test_part_one tests the different collaborators with the inputs
and outputs provided in the examples of part 1.
*/
bool test_part_one() {
	int const count_of_subtests = 3;
	std::string const input[count_of_subtests] = {">", "^>v<", "^v^v^v^v^v"};
	int const expected[count_of_subtests] = {2, 4, 2};

    bool passing = true;
    for(int index_of_subtests = 0; index_of_subtests < count_of_subtests; ++index_of_subtests) {
		int const count_of_unique_presents = part_one(input[index_of_subtests]);
		if(expected[index_of_subtests] != count_of_unique_presents) {
			std::cout << "Failed on subtest " << index_of_subtests << std::endl;
			passing = false;
			break;
		} else {
			std::cout << "Passed subtest " << index_of_subtests << std::endl;
		}
	}
	return passing;
}

bool test_part_two() {
	int const count_of_subtests = 3;
	std::string const input[count_of_subtests] = {"^v", "^>v<", "^v^v^v^v^v"};
	int const expected[count_of_subtests] = {3,3, 11};
	bool passing = true;
    for(int index_of_subtests = 0; index_of_subtests < count_of_subtests; ++index_of_subtests) {
		int const count_of_unique_presents = part_two(input[index_of_subtests]);
		if(expected[index_of_subtests] != count_of_unique_presents) {
			std::cout << "Failed on subtest " << index_of_subtests << std::endl;
			passing = false;
			break;
		} else {
			std::cout << "Passed subtest " << index_of_subtests << std::endl;
		}

	}
	return true;
}

int main(int argc, char* argv[]) {
	bool passing = test_part_one();
	passing = passing && test_part_two();
	if(passing) {
	    std::string input_data;
	    std::getline(std::cin, input_data);
		part_two(input_data);
	}
}
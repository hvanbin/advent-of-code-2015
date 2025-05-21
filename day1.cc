#include <iostream>

/**
 * --- Day 1: Not Quite Lisp ---

Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.

An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:

	(()) and ()() both result in floor 0.
	((( and (()(()( both result in floor 3.
	))((((( also results in floor 3.
	()) and ))( both result in floor -1 (the first basement level).
	))) and )())()) both result in floor -3.

To what floor do the instructions take Santa?

--- Part Two ---

Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

	) causes him to enter the basement at character position 1.
	()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?
 */

/**
 * @brief Count the number of floors Santa goes up and down.
 * @param inputs Input string of parentheses.
 */
int find_level(std::string const& inputs) {
	int floor_level = 0;
	for(char input : inputs) {
		if(input == '(') { floor_level += 1;}
		else if (input == ')') { floor_level -= 1;}
	}
	return floor_level;
}

/**
 * @brief Find the position of the first character that causes Santa to enter the basement.
 * @param inputs Input string of parentheses.
 * @return Position of the first character that causes Santa to enter the basement, or -1 if not found.
 */
int find_basement(std::string const& inputs) {
	short floor_level = 0;
	short position = 0;
	for(char input : inputs) {
		++position;
		if(input == '(') { floor_level += 1;}
		else if (input == ')') { floor_level -= 1;}
		if(-1 == floor_level) {
			return position;
		}
	}
	return -1;
}

/**
 * @brief Test the find_level function.
 * @return true if the test passes, false otherwise.
 */
bool test_find_level() {
	std::string inputs[] = {
		"(())",
		"()()",
		"(((",
		"(()(()(",
		"))(((((",
		"())",
		"))(",
		")))",
		")())())",
	};
	int expected_floor_levels[] = {
		0, 0, 3, 3, 3, -1, -1, -3, -3,
	};
	for(size_t i = 0; i < std::size(inputs); ++i) {
		int floor_level = find_level(inputs[i]);
		if(expected_floor_levels[i] != floor_level) {
			std::cerr << "test failed for input " << inputs[i] << std::endl;
			return false;
		}
	}
	return true;
}

/**
 * @brief Test the find_basement function.
 * @return true if the test passes, false otherwise.
 */
bool test_find_basement() {
	std::string inputs[] = {
		")",
		"()())",
	};
	int expected_basement_positions[] = {
		1, 5,
	};
	for(size_t i = 0; i < std::size(inputs); ++i) {
		int basement_position = find_basement(inputs[i]);
		if(expected_basement_positions[i] != basement_position) {
			std::cerr << "test failed for input " << inputs[i] << std::endl;
			return false;
		}
	}
	return true;
}

/**
 * @brief Count the number of floors Santa goes up and down.
 * If the character is a '(', increase the floor level by one.
 * If the character is a ')', decrease the floor level by one.
 * For part 2, answer for the correct position to reach floor_level -1.
 * @param argc Number of command-line arguments.
 * @param argv Command-line arguments.
 * @return EXIT_SUCCESS on success, EXIT_FAILURE on failure.
 */
int main(int argc, char* argv[]) {
	if(!test_find_level()) {
		return EXIT_FAILURE;
	}
	if(!test_find_basement()) {
		return EXIT_FAILURE;
	}
	std::string inputs;
	std::cin >> inputs;
	
	std::cout << "floor level " << find_level(inputs) << std::endl;
	std::cout << "basement position " << find_basement(inputs) << std::endl;
	return EXIT_SUCCESS;
}

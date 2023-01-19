#include <iostream>
#include <string>
#include <sstream>
#include <algorithm>
#include <array>
#include <tuple>

/*
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?
*/

class Present {
	public:
	Present(int length, int width, int height) : _length(length), _width(width), _height(height) {}
	int getWrappingArea() {
	    int lw, wh, lh;

	    lw = _length * _width;
	    wh = _width * _height;
	    lh = _length * _height;

	    int surface_area = 2 * (lw + wh + lh);

	    int* low = NULL;
	    low = (lw > wh) ? &wh : &lw;
	    low = (*low > lh) ? &lh : low;
	
	    return *low + surface_area;
	}
	int getRibbonLength() {
	    /* Find the two shortest sides.
	     * This is kind of a trashy algorithm. */
	    int l_or_w = (_length > _width)? _width : _length;
	    int w_or_h = (_width > _height)? _height : _width;
	    int l_or_h = (_length > _height)? _height : _length;
	    /* Among these three combinations, there are two values.
	     * The lowest of all is the lowest of these three.
	     * The "lower" is the greatest of these three. */
	    int lower, lowest = -1;
	    if(l_or_w > w_or_h) {
	    	lowest = w_or_h;
	    	lower = l_or_w;
	    } else if (w_or_h > l_or_w) {
	    	lowest = l_or_w;
	    	lower = w_or_h;
	    } else {
	    	/* lw and wh are the same, so after two minimizations,
	    	 * one of these _must_ be the lowest. The other is
	    	 * safe to assume to be the second-lowest, because of
	    	 * the first reduction. */
	    	lowest = l_or_w;
	    	lower = l_or_h;
	    }
	    int shortest_perimeter = 2 * (lower + lowest);
	    int bow_length = _length * _width * _height;
	    return shortest_perimeter + bow_length;

	}
	private:
	int _length;
	int _width;
	int _height;
};

struct PresentTestCase {
    int length;
    int width;
    int height;
    int expected_wrapping;
    int expected_ribbon;
};

bool test_present() {
    const std::array<PresentTestCase, 2> test_cases = {{
        {2, 3, 4, 58, 34},
        {1, 1, 10, 43, 14}
    }};
    
    bool passing = true;
    for (const auto& test : test_cases) {
        Present present(test.length, test.width, test.height);
        passing = passing && (test.expected_wrapping == present.getWrappingArea());
        passing = passing && (test.expected_ribbon == present.getRibbonLength());
    }
    return passing;
}

/**
 * Function calculate reads from standard input line-by-line to aggregate answers into totals.
 * To do this, it creates present objects based on the input and adds them to the aggregation.
 */
int calculate() {
	long total_wrapping_paper = 0;
	long total_ribbon = 0;
	short l, w, h;

	std::string line;
	while (std::getline(std::cin, line)) {
		std::replace(line.begin(), line.end(), 'x', ' ');
		std::istringstream iss(line);
		iss >> l >> w >> h;

        Present present(l, w, h);
		total_wrapping_paper += present.getWrappingArea();
		total_ribbon += present.getRibbonLength();
	}
	std::cout << "total wrapping paper: " << total_wrapping_paper << std::endl;
	std::cout << "total ribbon: " << total_ribbon << std::endl;
	return EXIT_SUCCESS;
}

int main(int argc, char* argv[]) {
	if(!test_present()) { return EXIT_FAILURE; }
	calculate();
	return EXIT_SUCCESS;
}

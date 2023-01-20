#ifndef MAP_H
#define MAP_H
#include "list.h"
struct Map;
typedef struct Map Map;
#endif

/*

struct Map {
	size_t y_boundary;
	size_t x_boundary;
	bool* data[];
}

void map_init(Map* map)
{
    data = malloc(sizeof(double[n][n]));
}

void map_insert(Map* map, size_t par_x, size_t par_y, bool val) {
        if(par_y > map->y_boundary) {
	    map->data = realloc(map->data, sizeof(y * sizeof(*data));
	    map->y_boundary = y;
	}
	if(par_x > map->x_boundary) {
	    realloc
	    set new boundary
	}
	map->data[y][x] = val;
}

void row_free(*Row row) {
	free(row)
	row = NULL;
}

void map_free(*Map map) {
	for(Row* row = map; row < row + map.size; ++row) {
		free_row(row);
	}
	col = NULL;
}

*/


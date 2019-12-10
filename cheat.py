def get_path(wire):
    i = j = 0
    step = 1
    path = {}
    for visit in wire:
        dir, position = visit[0], int(visit[1:])
        x = y = 0
        if dir == 'U':
            x = -1
        elif dir == 'D':
            x = 1
        elif dir == 'L':
            y = -1
        elif dir == 'R':
            y = 1
        else:
            raise Exception('Unexpected direction')

        for _ in range(position):
            i += x
            j += y
            path[(i, j)] = step
            step += 1

    return path

def dist(p, q):
    return abs(p[0] - q[0]) + abs(p[1] - q[1])

def calculate_min_distance(intersections):
    return min(dist(point, (0, 0)) for point in intersections)

def calculate_min_steps(intersections, steps_a, steps_b):
    min_steps = float('inf')
    for point in intersections:
        if point not in steps_a or point not in steps_b:
            raise Exception('Not an shared path intersection.')
        min_steps = min(min_steps, steps_a[point] + steps_b[point])
    return min_steps

data = open('day3.txt', 'r').readlines()
wire1, wire2 = data[0].strip().split(','), data[1].strip().split(',')
path_wire1, path_wire2 = get_path(wire1), get_path(wire2)
intersections  = list(set(path_wire1.keys()) & set(path_wire2.keys()))

# part 1
print(calculate_min_distance(intersections))

# part 2
print(calculate_min_steps(intersections, path_wire1, path_wire2))

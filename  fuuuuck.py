def rewind(line):
	while line.prev is not None and line.prev.prev is not None:
		line = line.prev
	return line


class Line:
	def __init__(self, points, direction, prev_line=None):
		self.a = points[0]
		self.b = points[-1]
		self.direction = direction
		self.points = points
		self.prev = prev_line
		self.next = None

	def append(self, other):
		if self.next is not None:
			self.next.append(other)
		else:
			self.next = other


	def distance_to_intersection(self, intersection):
		x, y = intersection
		if self.direction is 'U':
			return abs(y - self.a[1])
		elif self.direction is 'D':
			return abs(self.a[1] - y)
		elif self.direction is 'L':
			return abs(self.a[0] - x)
		elif self.direction is 'R':
			return abs(x - self.a[0])

	def runs_parallel(self, other):
		if self.direction in 'UD' and other.direction in 'UD':
			return True
		if self.direction in 'LR' and other.direction in 'LR':
			return True
		
		return False

	def min_max(self):
		if self.a[0] == self.b[0]:
			return (
				max(self.a[1], self.b[1]),
				min(self.a[1], self.b[1])
			)
		if self.a[1] == self.b[1]:
			return (
				max(self.a[0], self.b[0]),
				min(self.a[0], self.b[0])
			)

	def intersects_with(self, other):
		max_bound, min_bound = self.min_max()
		other_max, other_min = other.min_max()
		if self.runs_parallel(other):
			return False

		# if self.a[0] == self.b[0]:
		# 	# AX is not changing
		# 	# BX IS
		# 	# print(self, other)
		# 	if other_max > self.a[0] and other_min < self.a[0]:
		# 		print(self, other)
		if self.a[0] == self.b[0]:
			x = self.a[0]
			min_y = min(self.a[1], self.b[1])
			max_y = max(self.a[1], self.b[1])
			# DOES OTHER Y FIT INSIDE MIN_Y AND MAX_Y
			other_y = other.a[1]

			max_x = max(other.a[0], other.b[0])
			min_x = min(other.a[0], other.b[0])
			if other_y > min_y and other_y < max_y and x > min_x and x < max_x:
				return (self.a[0], other.b[1])

		elif self.a[1] == self.b[1]:
			y = self.a[1]
			min_x = min(self.a[0], self.b[0])
			max_x = max(self.a[0], self.b[0])
			# DOES OTHER Y FIT INSIDE MIN_Y AND MAX_Y
			other_x = other.a[0]

			max_y = max(other.a[1], other.b[1])
			min_y = min(other.a[1], other.b[1])

			if other_x > min_x and other_x < max_x and y > min_y and y < max_y:
				return (other.a[0], self.b[1])
			# y = other.a[1]
			# # if y is greater than the min and less than the max?
			# if y < self.a[0]:
				
			# if min_bound >= other_min and max_bound <= other_max:
				

			
	def __str__(self):
		return f"{self.points} - {self.direction}"
		

class LinkedList(object):
	def __init__(self, x=None, y=None, prev=None, direction=None):
		self.prev = prev
		self.x = x
		self.y = y
		self.next = None
		self.direction = direction

	def push(self, x, y, direction):
		self.next = LinkedList(x, y, self, direction)
		return self.next

	def is_not_parallel(self, other):
		if self.direction is 'U' or self.direction is 'D' and other.direction is 'L' or other.direction is 'R':
			return True
		
		if self.direction is 'L' or self.direction is 'R' and other.direction is 'U' or other.direction is 'D':
			return True

		return False

	def last(self):
		link = self
		while link.next is not None:
			link = link.next

		return link

	def __str__(self):
		return f"LinkedList({self.x}, {self.y}, {self.direction})"


def generate_path(instructions):
	path = []
	cursor = [0, 0]
	x, y = cursor
	link = LinkedList(0, 0)
	line = None
	for instruction in instructions:
		direction = instruction[0]
		distance = int(instruction[1:])
		subpath = []
		if direction is 'U':
			for i in range(0, distance + 1):
				link = link.push(x, y + i, direction)
				subpath.append((x, y + i))
			# y += distance
		elif direction is 'D':
			for i in range(0, distance + 1):
				link = link.push(x, y - i, direction)
				subpath.append((x, y - i))
			# y -= distance
		elif direction is 'L':
			for i in range(0, distance + 1):
				link = link.push(x - i, y, direction)
				subpath.append((x - i, y))
			# x -= distance
		elif direction is 'R':
			for i in range(0, distance + 1):
				link = link.push(x + i, y, direction)
				subpath.append((x + i, y))
			# x += distance
		x, y = subpath[-1]
		if line is None:
			line = Line(subpath, direction)
		else:
			line.append(Line(subpath, direction))

		path.append(Line(subpath, direction))


	return (path, rewind(link), line)

def find_point(a, b):
	x, y = b.x, b.y
	point = None
	while a is not None:
		if a.x is x and a.y is y:
			point = a
			break

		a = a.next
	
	return point




def by_direction(direction, link):
	points = []
	while link.next is not None:
		if link.direction in link.direction:
			points.append((link.x, link.y))
		link = link.next

	return points

class Intersection:
	def __init__(self, x, y, lines=None):
		self.x = x
		self.y = y
		self.lines = lines

	def steps_taken(self):
		line_a, line_b = self.lines

		if line_a.direction is 'U':
			# x is static, so figure out the distance from the start.y to `y`
			# y lower to higher
			# subtract `y` from line origin
			return abs(self.y) - abs(line_a.b[1])



def distance_to_origin(x, y):
	return abs(x) + abs(y)

with open("./day3sample.txt") as f:
	first_path, link_first, first_line = generate_path(f.readline().split(","))
	second_path, link_second, second_line = generate_path(f.readline().split(","))
	intersections = []
	steps_taken_first_path = 0
	steps_taken_second_path = 0
	# print([str(x) for x in second_path])
	for (i, fp) in enumerate(first_path):
		for (ix, sp) in enumerate(second_path):
			# print(fp, fp.min_max())
			# print(sp, sp.min_max())
			a = fp.intersects_with(sp)
			if a:
				intersections.append((a, ))
	import ipdb; ipdb.set_trace()
	print(min([distance_to_origin(*x) for x in intersections]))

	# print([str(x) for x in first_path])
	# print([x.min_max() for x in first_path])
	# # import ipdb; ipdb.set_trace()
	# print(intersections)
	# for line in first_path:
	# 	a_origin = line[0]
	# 	a_dest = line[-1]
	# 	print(a_origin)

	# sample_1 = first_path[25]
	# sample_2 = second_path[81]

	# intersections = []

	# # link_first.prev = None
	# # link_second.prev = None
	# print(len(first_path), len(second_path))
	# while link_first is not None:
	# 	intersection = find_point(link_second, link_first)
	# 	if intersection:
	# 		intersections.append(intersection)
	# 	link_first = link_first.next
	# print([str(x) for x in intersections])
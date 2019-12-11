from functools import reduce

class Orbit:
	def __init__(self, name, other=None, distance=1):
		self.name = name
		self.orbits = other
		self.orbited_by = {}
		self.distance = distance
	
	def insert(self, parent, child):
		print(parent, child)
		if parent == self.name:
			child = self.orbited_by[child] = Orbit(child, other=self, distance=self.distance + 1)
			return child
		if parent in self.orbited_by:
			return self.orbited_by[parent].insert(parent, child)
		else:
			for node in self.orbited_by.values():
				return node.insert(parent, child)

	def add_orbit(self, other_name):
		child = self.orbited_by[other_name] = Orbit(other_name, other=self, distance=self.distance + 1)
		return child

	def rewind_to_root(self):
		if self.orbits is None:
			return self
		
		return self.orbits.rewind_to_root()

	def find(self, node_name):
		if node_name == self.name:
			return self

		if node_name in self.orbited_by:
			return self.orbited_by[node_name]
		
		for node in self.orbited_by.values():
			return node.find(node_name)

	def rewind_to(self, root_name):
		orbit = self

		while True:
			if orbit.orbits and orbit.orbits.name == root_name:
				orbit = orbit.orbits
			elif root_name in orbit.orbited_by:
				orbit = orbit.orbited_by[root_name]
			else:
				orbit = orbit.orbits

			if orbit.name == root_name:
				break

		return orbit

	def does_orbit(self, name):
		return name in self.orbited_by

	def __repr__(self):
		return f"Orbit({self.name})"


def chunks(iterable, size=2):
	i = 0
	length = len(iterable)

	while True:
		# try:
		chunk = iterable[i:i+size]
		# except IndexError:
		# 	break
		if len(chunk) == 1:
			yield chunk[0], None
		else:
			yield chunk


		i += 1
		if i >= length:
			break


def map_nodes(tree, data):
	node = tree
	for orbit in data:
		nodes = node.keys()
		p, c = orbit.split(')')
		if len(nodes) == 0:
			node = tree[p] = {
				c: None
			}
			continue

		# if the parent is NOT in the current node, then traversal may need to be reset?
		# in a straight orbit line, p is the previous iterations child, so the key should be set
		# 

		# if the child node for the parent was already set, but the child itself was not initialized
		if p in node and node[p] is None:
			node = node[p] = {
				c: None
			}
		elif p in node and node[p][c] is None:
			import ipdb; ipdb.set_trace()
		else:
			import ipdb; ipdb.set_trace()


class OrbitBody(dict):
	def __init__(self, name, parent=None):
		self.name = name
		self.parent = parent
		self.orbited_by = {}
		if parent is not None:
			self.distance = parent.distance + 1
		else:
			self.distance = 0

	def update_distance(self):
		name = self.name
		node = self
		distance = 0
		while node.parent is not None:
			distance += node.distance
			node = node.parent

		node.distance += distance

	def __repr__(self):
		if self.parent is None:
			return f"Orbit({self.name})"

		keys = []

		node = self
		while node.parent is not None:
			keys.append(node.parent.name)
			node = node.parent

		return f"Orbit({self.name}) {keys}"
	
	def __getitem__(self, key):
		return self.orbited_by[key]

	def get_path(self):
		this = self
		path = []
		while this.parent is not None:
			this = this.parent
			path.append(this.name)

		return [x for x in reversed(path)]
	
	def find_divergence(self, other):
		this_path = self.get_path()
		other_path = other.get_path()
		i = 0
		while True:
			if this_path[i] != other_path[i]:
				break
			i += 1

		return len(this_path[i - 1:-1]) + len(other_path[i - 1:-1])

	def find(self, node_name):
		if node_name == self.name:
			return self

		if node_name in self.orbited_by:
			return self.orbited_by[node_name]

		for node in self.orbited_by.values():
			v = node.find(node_name)
			if v is not None:
				return v

	def get_total(self):
		if self.orbited_by is None:
			return self.distance
		return reduce(lambda x, y: x + y.get_total(), self.orbited_by.values(), self.distance)
	

def get_data():
	with open('./day06.txt') as fs:
		lines = [x.split(')') for x in fs.read().split("\n")]
		primary = {}
		root = {}
		for orbit_pair in lines:
			parent, child = orbit_pair
			if parent in root:
				root[parent].append(child)
			else:
				root[parent] = [child]
		
		return root


def remap(p_key, r, parent_orbit=None):
	if p_key not in r:
		return parent_orbit
	else:
		parent_orbit.orbited_by = { k: remap(k, r, parent_orbit=OrbitBody(k, parent=parent_orbit)) for k in r[p_key] }
		return parent_orbit


def count_data(k, branch, count=1):
	if branch[k] is None:
		print('END')
		return 1
	
	for key in branch[k].keys():
		count += count_data(key, branch[k], count=count)

	return count
	




data = { 'COM': remap('COM', get_data(), OrbitBody('COM')) }
num = 0
print(data)
primary = data['COM']
print(primary.get_total())
the_thing = primary.find('YOU')
santa = primary.find('SAN')
divergence = the_thing.find_divergence(santa)

# print(count_data('COM', data))
# for whatever in count_data('B', data):
# 	import ipdb; ipdb.set_trace()
	



import ipdb; ipdb.set_trace()




	# for parent in root.keys():
	# 	children = root[parent]
	# 	if orbit is None:
	# 		orbit = Orbit(parent)
	# 	for child in children:
	# 		orbit = orbit.insert(parent, child)






	# while True:

	# orbit = orbit.add_orbit(p)
	# orbit = orbit.add_orbit(c)
	# count = 0



	# while True:
	# 	for orbit_pair in lines[1:]:
	# 		parent, child = orbit_pair
	# 		print(parent, child)
	# 		if orbit.name == parent:
	# 			orbit = orbit.add_orbit(child)
	# 			count += orbit.distance
	# 		else:
	# 			orbit = orbit.rewind_to_root()
	# 			if orbit.find(parent) is not None:
	# 				orbit = orbit.add_orbit(child)
	# 				count += orbit.distance + 1
	# 			else:
	# 				orbit = orbit.add_orbit(parent)
	# 				orbit = orbit.add_orbit(child)

		# for chunk in chunks(lines):
		# 	node = primary
		# 	p, c = chunk
		# 	curr_parent, curr_child = p
		# 	next_parent, next_child = c
		# 	orbit = orbit.add_orbit(curr_parent)
		# 	orbit.add_orbit()

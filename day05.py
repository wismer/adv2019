# class Operation:
# 	def __init__(self):


# fetch value from index

# use value from current position


def parse_operation(operation, raw_operations):
	padded = operation.rjust(5, '0')
	operation, modes = int(padded[-2:]), padded[0:4]
	p1 = lambda v: raw_operations[v] if padded[0] is '0' else lambda v: v
	p2 = lambda v: raw_operations[v] if padded[1] is '0' else lambda v: v
	p3 = lambda v: raw_operations[v] if padded[2] is '0' else lambda v: v

	# def add(a):
	# 	def add(b):
	# 		def update(idx):
	# 			raw_operations[p3(idx)] = p1(a) + p2(b)
	# 			return (idx, p1(a) + p2(b))
	# 		return update
	# 	return add

	# def mult(a):
	# 	def mult(b):
	# 		def update(idx):
	# 			return (idx, p1(a) * p2(b))
	# 		return update
	# 	return mult

	if operation is 0:
		return (1, [])
	elif operation is 1:
		return (3, modes)
	elif operation is 2:
		return (3, modes)
	elif operation is 3:
		return (2, [lambda xs, i: 1])
	elif operation is 4:
		return (2, [lambda xs, i: print(i)])




with open("./day05.txt") as f:
	ops = [int(x) for x in f.read().split(',')]
	idx = 0
	input_v = 8
	inc = None
	s = None
	# while True:
	while True:
		op = str(ops[idx])
		padded = op.rjust(5, '0')
		operation, modes = int(padded[-2:]), padded[0:3]
		def positional(i):
			return ops[i]

		p1 = positional if modes[2] is '0' else lambda v: v
		p2 = positional if modes[1] is '0' else lambda v: v

		# copy the parameters?
		if operation == 1 or operation == 2:
			inc = 4
			rest = ops[(idx + 1):(idx + 4)]

		elif operation == 3 or operation == 4:
			inc = 2
			if operation == 3:
				rest = ops[idx + 1]
			else:
				rest = p1(ops[idx + 1])
		elif operation == 5 or operation == 6:
			param1 = p1(ops[idx + 1])
			param2 = p2(ops[idx + 2])

			if operation == 5 and param1 != 0:
				idx = param2
				continue
			elif operation == 5:
				idx += 3
				continue
			elif operation == 6 and param1 == 0:
				idx = param2
				continue
			elif operation == 6:
				idx += 3
				continue
		elif operation == 7 or operation == 8:
			if operation == 7:
				import ipdb; ipdb.set_trace()
			param1 = p1(ops[idx + 1])
			param2 = p2(ops[idx + 2])

			if operation == 7:
				v = 1 if param1 < param2 else 0
			else:
				v = 1 if param1 == param2 else 0
			pos = positional(ops[idx + 3])
			ops[pos] = v
			idx += 4
			continue
		elif operation == 99:
			print(ops)
			break
		else:
			# assume operation 0?
			idx += 1
			# import ipdb; ipdb.set_trace()
			continue
			
		if operation == 1:
			a, b = p1(rest[0]), p2(rest[1])

			# if a < 0 or b < 0:
			# 	import ipdb; ipdb.set_trace()
			try:
				ops[rest[2]] = p1(rest[0]) + p2(rest[1])
			except:
				import ipdb; ipdb.set_trace()
		elif operation == 2:
			ops[rest[2]] = p1(rest[0]) * p2(rest[1])
		elif operation == 3:
			ops[rest] = input_v
		elif operation == 4:
			print(f"output: {rest}")
		else:
			import ipdb; ipdb.set_trace()

		idx += inc

		if idx >= len(ops):
			break



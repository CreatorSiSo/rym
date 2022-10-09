say_hello = False

if say_hello:
	print("Hello World!")
else:
	print("Bye World!")

if not say_hello:
	print("`say_hello` is not `true`")

if True:
	""
elif False:
	print("testing")
else:
	print("nope")

x = 0
while True:
	print("Round:", x)
	x += 1
	if x > 99:
		break

	y = 0
	while True:
		print(y)
		y += 1
		if y >= 99:
			break

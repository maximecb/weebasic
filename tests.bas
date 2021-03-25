# Nonzero is true, 0 is false
assert 1
#assert 0

# Basic arithmetic
assert 1 == 1
assert 1 < 2
assert 1 + 2

# Local variables
let x = 1
assert x
assert x == 1

# If statements
if 0 then assert 0
if 1 then assert 1
if 1 < 2 then assert 1

if 1 then begin
    let y = 1
    let z = y + 1
    assert z
end

# TODO: there is no ability to parse or print strings yet
#print "All tests OK"
print 1

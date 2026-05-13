def double_it(x: Int) -> Int:
    var doubled = x * 2
    return doubled


def total_after_increment(start: Int) -> Int:
    var next_value = start + 1
    var total = double_it(next_value)
    return total


def main():
    var start = 20
    var result = total_after_increment(start)
    print(result)
    for i in range(10):
        print(i)

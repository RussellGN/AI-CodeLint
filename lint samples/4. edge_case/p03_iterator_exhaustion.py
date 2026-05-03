def process(nums):
    gen = (x * 2 for x in nums)
    total = sum(gen)
    items = list(gen)
    return total, items

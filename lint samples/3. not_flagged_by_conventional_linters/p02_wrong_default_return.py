def first_negative(nums):
    for n in nums:
        if n < 0:
            return n
    return 0

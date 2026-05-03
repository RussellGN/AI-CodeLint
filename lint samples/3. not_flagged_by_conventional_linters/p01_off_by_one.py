def find_last(arr, target):
    for i in range(len(arr) - 1):
        if arr[i] == target:
            return i
    return -1

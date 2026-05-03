def read_file(path):
    try:
        with open(path) as f:
            return f.read()
    except:
        return None

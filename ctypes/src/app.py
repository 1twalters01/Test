import ctypes

def start():
    testlib = ctypes.CDLL('./example.so')
    testlib.printTest()

import sys

def main(filename: str) -> None:
    with open(filename, "wb") as f:
        for _ in range(4096 * 8):
            f.write((0).to_bytes(1))
        pass
    print("ran", filename)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("please specify db file", file=sys.stderr) 
        exit()
    main(sys.argv[1])

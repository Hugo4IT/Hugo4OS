import os

def main():
    buffer = bytearray()
    for i in range(255):
        for j in range(255):
            buffer.append(int(float(i)/255.0*float(j)))

    with open(os.path.join(os.path.dirname(__file__), "lerp-lookup-table.bin"), "wb+") as file:
        file.write(buffer)

if __name__ == "__main__":
    main()
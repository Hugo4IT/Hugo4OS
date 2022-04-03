import os

def main():
    div_buffer = bytearray()
    mult_buffer = bytearray()
    for i in range(256):
        for j in range(256):
            mult_buffer.extend(int(float(i)/255.0*float(j)/255.0 * 255.0).to_bytes(2, byteorder="little", signed=False))
            if i == 0 or j == 0:
                div_buffer.extend(bytearray([0, 0]))
            else:
                div_buffer.extend(int((float(i)/255.0)/(float(j)/255.0) * 255.0).to_bytes(2, byteorder="little", signed=False))

    with open(os.path.join(os.path.dirname(__file__), "color-mult-lookup-table.bin"), "wb+") as file:
        file.write(mult_buffer)
    with open(os.path.join(os.path.dirname(__file__), "color-div-lookup-table.bin"), "wb+") as file:
        file.write(div_buffer)

if __name__ == "__main__":
    main()
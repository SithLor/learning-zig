def make_crc_table():
    crc_table = []
    for i in range(256):
        crc = i
        for _ in range(8):
            if crc & 1:
                crc = (crc >> 1) ^ 0xEDB88320
            else:
                crc >>= 1
        crc_table.append(crc)
    return crc_table

crc_table = make_crc_table()
print(crc_table)
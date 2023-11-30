import fileinput

def write_into_field(path_to_table: str, field_name: str, data: str):
    with fileinput.FileInput(path_to_table, inplace = True) as f: 
        for line in f: 
            if field_name in line: 
                print("{},{}".format(line.replace("\n", ''), data), end ='\n') 
            else: 
                print(line, end ='') 

write_into_field("../../databases/qwerty/users.alicedb", "username", "0xjanus")
write_into_field("../../databases/qwerty/users.alicedb", "password", "0xjanusPass")
